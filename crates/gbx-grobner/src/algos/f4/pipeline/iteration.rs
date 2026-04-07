use std::hash::Hash;
use std::time::{Duration, Instant};

use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::selector::MinDegreeSelector;
use crate::algos::f4::pipeline::insert::insert_new_rows;
use crate::algos::f4::pipeline::reduction::reduction_phase;
use crate::algos::f4::pipeline::select::select_pairs_phase;
use crate::algos::f4::state::F4State;
use crate::instrumentation::{f4_debug, f4_info};

use crate::linear::DenseF4MatrixReducer;
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

fn pct(part: Duration, total: Duration) -> f64 {
    if total.is_zero() { 0.0 } else { 100.0 * part.as_secs_f64() / total.as_secs_f64() }
}

pub fn run_iteration<P, F, O>(
    ctx: &RingCtx<F, O>,
    state: &mut F4State<P>,
    opts: &F4Options,
    selector: &mut MinDegreeSelector,
    reducer: &DenseF4MatrixReducer,
    criterion: &ProductCriterion,
) -> Result<()>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Hash,
{
    state.advance_iteration();

    f4_debug!(
        iteration = state.iteration,
        pending_len = state.pending.len(),
        basis_len = state.basis.len(),
        "starting iteration"
    );

    let iter_started = Instant::now();

    let t0 = Instant::now();
    let pair_selection = select_pairs_phase(state, selector);
    let select_dt = t0.elapsed();

    if pair_selection.selected_pairs.is_empty() {
        let total_dt = iter_started.elapsed();

        f4_info!(
            iteration = state.iteration,
            total_ms = total_dt.as_secs_f64() * 1000.0,
            select_ms = select_dt.as_secs_f64() * 1000.0,
            select_pct = pct(select_dt, total_dt),
            "iteration finished with no selected pairs"
        );
        return Ok(());
    }

    let t1 = Instant::now();
    let reduction = reduction_phase(
        ctx,
        opts,
        &pair_selection.l_d,
        &state.basis,
        &state.history,
        reducer,
    )?;
    let reduction_dt = t1.elapsed();

    let t2 = Instant::now();
    state.push_history(reduction.f_d.rows.clone(), reduction.f_d_tilde.clone());
    let push_history_dt = t2.elapsed();

    let extracted_count = reduction.extracted_rows.len();

    let t3 = Instant::now();
    insert_new_rows::<P, F, O>(ctx, state, reduction.extracted_rows, criterion)?;
    let insert_dt = t3.elapsed();

    let total_dt = iter_started.elapsed();

    f4_info!(
        iteration = state.iteration,
        pending_len = state.pending.len(),
        basis_len = state.basis.len(),
        selected_pairs = pair_selection.selected_pairs.len(),
        l_d_len = pair_selection.l_d.len(),
        extracted_rows = extracted_count,
        total_ms = total_dt.as_secs_f64() * 1000.0,
        select_ms = select_dt.as_secs_f64() * 1000.0,
        select_pct = pct(select_dt, total_dt),
        reduction_ms = reduction_dt.as_secs_f64() * 1000.0,
        reduction_pct = pct(reduction_dt, total_dt),
        push_history_ms = push_history_dt.as_secs_f64() * 1000.0,
        push_history_pct = pct(push_history_dt, total_dt),
        insert_ms = insert_dt.as_secs_f64() * 1000.0,
        insert_pct = pct(insert_dt, total_dt),
        "iteration timing"
    );

    f4_debug!(
        iteration = state.iteration,
        pending_len = state.pending.len(),
        basis_len = state.basis.len(),
        "finished iteration"
    );

    Ok(())
}
