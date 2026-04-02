use std::time::Instant;

use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::extract::rows::extract_new_rows;
use crate::algos::f4::linear::echelon::PolynomialEchelonReducer;
use crate::algos::f4::linear::reducer::BatchReducer;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::critical_pair::CriticalPair;
use crate::algos::f4::pairs::selector::{MinDegreeSelector, PairSelector};
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;
use crate::algos::f4::symbolic::preprocess::symbolic_preprocess;
use crate::algos::f4::symbolic::seeds::SymbolicSeed;
use crate::algos::f4::trace::{F4TraceCtx, SharedF4Tracer};

use crate::basis::GrobnerBasis;
use crate::{minimize_in_place, reduce_in_place, BasisPostOptionsKind};

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub fn run_f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options, tracer: Option<SharedF4Tracer>) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    if opts.batch_size == 0 {
        return Err(F4Error::InvalidBatchSize);
    }

    let run_started = Instant::now();
    let trace = F4TraceCtx::new(tracer);

    let criterion = ProductCriterion;
    let mut selector = MinDegreeSelector::new(opts.batch_size);
    let reducer = PolynomialEchelonReducer;

    let mut state = F4State::new();

    for mut f in fs.into_iter() {
        if f.is_zero() {
            continue;
        }

        if opts.normalize_inputs {
            f.normalize_in_place(ctx)?;
        }

        if f.is_zero() {
            continue;
        }

        update_with_polynomial(&mut state.basis, &mut state.pending, f, &criterion)?;
    }

    if state.basis.is_empty() {
        return Ok(GrobnerBasis::empty_in(ctx));
    }

    trace.on_init_complete(state.basis.len(), state.pending.len());

    let main_loop_started = Instant::now();

    while !state.is_done() {
        state.advance_iteration();
        trace.on_iteration_start(state.iteration, state.pending.len(), state.basis.len());

        let iter_started = Instant::now();

        let t0 = Instant::now();
        let pairs = state.pending.drain_all();
        let selection = selector.select(pairs);
        let selected_pairs = selection.selected;
        state.pending.replace(selection.remaining);
        let dt_selection = t0.elapsed();
        trace.add_selection_time(dt_selection);
        trace.on_pairs_selected(&selected_pairs, state.pending.len());

        if selected_pairs.is_empty() {
            continue;
        }

        let t0 = Instant::now();
        let ld = build_ld(&selected_pairs);
        let dt_build_ld = t0.elapsed();
        trace.add_build_ld_time(dt_build_ld);

        let t0 = Instant::now();
        let symbolic_rows = symbolic_preprocess(ctx, &ld, &state.basis, &state.history)?;
        let dt_symbolic = t0.elapsed();
        trace.add_symbolic_time(dt_symbolic);
        trace.on_symbolic_complete(symbolic_rows.len());

        let t0 = Instant::now();
        let reduced_rows = reducer.reduce(ctx, &symbolic_rows)?;
        let dt_reduction = t0.elapsed();
        trace.add_reduction_time(dt_reduction);
        trace.on_reduction_complete(reduced_rows.len());

        let t0 = Instant::now();
        let mut new_rows = extract_new_rows(&symbolic_rows, &reduced_rows, &ctx.order)?;
        let dt_extraction = t0.elapsed();
        trace.add_extraction_time(dt_extraction);
        trace.on_rows_extracted(new_rows.len());

        let mut dt_normalize_extracted = std::time::Duration::ZERO;
        if opts.normalize_extracted {
            let t0 = Instant::now();
            for row in &mut new_rows {
                if !row.is_zero() {
                    row.normalize_in_place(ctx)?;
                }
            }
            dt_normalize_extracted = t0.elapsed();
            trace.add_normalize_extracted_time(dt_normalize_extracted);
        }

        state.push_history(symbolic_rows, reduced_rows);

        let t0 = Instant::now();
        for row in new_rows {
            if row.is_zero() {
                trace.on_zero_extracted();
                continue;
            }

            let row = if opts.safety_reduce_extracted { row } else { row };

            update_with_polynomial(&mut state.basis, &mut state.pending, row, &criterion)?;
            trace.on_inserted(state.basis.len(), state.pending.len());
        }
        let dt_insert_update = t0.elapsed();
        trace.add_insert_update_time(dt_insert_update);

        trace.on_iteration_timing(
            state.iteration,
            iter_started.elapsed(),
            dt_selection,
            dt_build_ld,
            dt_symbolic,
            dt_reduction,
            dt_extraction,
            dt_normalize_extracted,
            dt_insert_update,
        );
    }

    trace.add_main_loop_time(main_loop_started.elapsed());

    let mut gb = GrobnerBasis::new(ctx.id(), state.basis);

    let t0 = Instant::now();
    match opts.post {
        BasisPostOptionsKind::None => {}
        BasisPostOptionsKind::Minimal => minimize_in_place(ctx, &mut gb)?,
        BasisPostOptionsKind::Reduced => reduce_in_place(ctx, &mut gb)?,
    }
    trace.add_post_process_time(t0.elapsed());

    trace.set_total_run_time(run_started.elapsed());
    trace.on_loop_complete(gb.as_slice().len(), 0);

    Ok(gb)
}

fn build_ld<M>(pairs: &[CriticalPair<M>]) -> Vec<SymbolicSeed<M>>
where
    M: Clone + Monomial + MonomialAlgos,
{
    let mut out = Vec::with_capacity(pairs.len() * 2);

    for pair in pairs {
        let left = pair.left();
        let right = pair.right();

        out.push(SymbolicSeed { basis_index: left.basis_index, multiplier: left.multiplier.clone() });

        out.push(SymbolicSeed { basis_index: right.basis_index, multiplier: right.multiplier.clone() });
    }

    out
}
