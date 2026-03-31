use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::extract::extract_new_polynomials;
use crate::algos::f4::matrix::build_dense_matrix;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::trace::{F4TraceCtx, SharedF4Tracer, WhileKind};
use crate::algos::post::BasisPostOptionsKind;
use crate::basis::GrobnerBasis;
use crate::batch::{PairBatchSelector, SameKeyBatchSelector};
use crate::matrix::reduce::row_reduce_dense;
use crate::symbolic::{build_pair_seed_rows, symbolic_preprocess};
use crate::trace::CorePhaseKind;
use crate::{minimize_in_place, reduce_in_place, PairQueue, PairSetView, PairUpdate};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};
use std::hash::Hash;
use std::time::Instant;

/// Run the current F4 engine.
///
/// This is still a basic F4-style pipeline:
///
/// 1. initialize the basis from input generators,
/// 2. seed the initial critical-pair set,
/// 3. repeatedly:
///    - select a batch of pairs,
///    - build batch S-polynomials,
///    - perform symbolic preprocessing,
///    - build and reduce the coefficient matrix,
///    - extract new polynomials,
///    - append accepted nonzero polynomials to the basis,
///    - update the pending pair set.
///
/// # Notes
///
/// At the moment, symbolic preprocessing is still the simplified `v1` variant,
/// so this engine remains a stepping stone toward a more faithful F4
/// implementation.
pub fn run_f4<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options, mut pairs: Q, update: &mut U, tracer: Option<SharedF4Tracer>) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
    U::Key: PartialEq,
    <<P as PolynomialView>::Term as TermView>::Mono: Hash,
{
    let trace = F4TraceCtx::new(tracer);

    let generators = fs.into_iter().collect::<Vec<_>>();
    validate_inputs(&generators, &opts)?;

    let init_t0 = Instant::now();
    let mut gb = init_basis(ctx, generators, &opts)?;
    trace.add_phase_time(CorePhaseKind::Init, init_t0.elapsed());
    trace.on_init_complete(gb.len());

    if gb.is_empty() {
        trace.print_summary(0, pairs.len());
        return Ok(gb);
    }

    let seed_t0 = Instant::now();
    seed_initial_pairs(&gb, &mut pairs, update, &trace)?;
    trace.add_phase_time(CorePhaseKind::Seed, seed_t0.elapsed());

    let mut batch_selector = SameKeyBatchSelector::new(opts.batch_size);

    let while_t0 = Instant::now();
    while !pairs.is_empty() {
        let t0 = Instant::now();
        let batch = batch_selector.select_batch(&mut pairs);
        trace.add_while_time(WhileKind::BatchSelect, t0.elapsed());

        if batch.is_empty() {
            break;
        }
        trace.on_batch_selected(batch.len());

        let t0 = Instant::now();
        let seeds = build_pair_seed_rows(&gb, &batch)?;
        trace.add_while_time(WhileKind::SeedRows, t0.elapsed());
        trace.on_seed_rows(seeds.len());

        if seeds.is_empty() {
            trace.maybe_emit_progress(gb.len(), pairs.len());
            continue;
        }

        let t0 = Instant::now();
        let symbolic = symbolic_preprocess(ctx, &gb, seeds)?;
        trace.add_while_time(WhileKind::Symbolic, t0.elapsed());
        trace.on_symbolic(symbolic.reducer_rows.len(), symbolic.all_rows.len());

        let t0 = Instant::now();
        let dense = build_dense_matrix(ctx, &symbolic)?;
        trace.add_while_time(WhileKind::MatrixBuild, t0.elapsed());
        trace.on_matrix_shape(dense.nrows(), dense.ncols());

        let t0 = Instant::now();
        let reduced = row_reduce_dense(ctx, dense)?;
        trace.add_while_time(WhileKind::RowReduce, t0.elapsed());

        let t0 = Instant::now();
        let extracted = extract_new_polynomials(
            ctx,
            &gb,
            reduced,
            opts.normalize_extracted,
            opts.safety_reduce_extracted,
        )?;
        trace.add_while_time(WhileKind::Extract, t0.elapsed());
        trace.on_extracted(extracted.len());

        for p in extracted {
            if p.is_zero() {
                trace.on_skipped_zero_extracted();
                continue;
            }

            let new_index = gb.len();
            gb.push(p);

            let t0 = Instant::now();
            update.on_new_poly(&gb, &mut pairs, new_index)?;
            trace.add_while_time(WhileKind::PairUpdate, t0.elapsed());

            trace.on_inserted_poly(gb.len());
        }

        trace.maybe_emit_progress(gb.len(), pairs.len());
    }
    trace.add_phase_time(CorePhaseKind::WhileLoop, while_t0.elapsed());

    let post_t0 = Instant::now();
    postprocess_basis(ctx, &mut gb, &opts)?;
    trace.add_phase_time(CorePhaseKind::Post, post_t0.elapsed());

    trace.set_gb_len(gb.len());
    trace.print_summary(gb.len(), pairs.len());

    Ok(gb)
}

/// Seed the initial pair set from the starting basis.
fn seed_initial_pairs<P, Q, U>(gb: &GrobnerBasis<P>, pairs: &mut Q, update: &mut U, trace: &F4TraceCtx) -> Result<()>
where
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
{
    for k in 0..gb.len() {
        let t0 = Instant::now();
        update.on_new_poly(gb, pairs, k)?;
        trace.add_while_time(WhileKind::PairUpdate, t0.elapsed());
    }

    Ok(())
}

/// Initialize the working Gröbner basis from the input generators.
///
/// This helper:
///
/// - optionally normalizes each input polynomial,
/// - drops zero generators,
/// - preserves input order among surviving generators.
fn init_basis<P, F, O>(ctx: &RingCtx<F, O>, generators: Vec<P>, opts: &F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut gb = GrobnerBasis::empty_in(ctx);

    for mut p in generators {
        if opts.normalize_inputs {
            p.normalize_in_place(ctx)?;
        }

        if !p.is_zero() {
            gb.push(p);
        }
    }

    Ok(gb)
}

/// Validate top-level F4 input options and generator collection.
fn validate_inputs<P>(generators: &[P], opts: &F4Options) -> Result<()> {
    if generators.is_empty() {
        return Err(F4Error::EmptyInput);
    }

    if opts.batch_size == 0 {
        return Err(F4Error::InvalidBatchSize);
    }

    Ok(())
}

fn postprocess_basis<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>, opts: &F4Options) -> Result<()>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    match opts.post {
        BasisPostOptionsKind::None => {}
        BasisPostOptionsKind::Minimal => minimize_in_place(ctx, gb)?,
        BasisPostOptionsKind::Reduced => reduce_in_place(ctx, gb)?,
    }

    Ok(())
}
