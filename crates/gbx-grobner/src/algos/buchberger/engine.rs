use super::bounds::BuchbergerTerm;
use super::init::init_basis;
use super::options::{BasisPost, BuchbergerOptions};
use super::trace::{BuchbergerTraceCtx, PhaseKind, SharedBuchbergerTracer, WhileKind};
use crate::algos::{minimize_in_place, reduce_in_place};
use crate::{s_polynomial_in, BuchbergerError, GrobnerBasis, PairQueue, PairSetView, PairUpdate};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;
use std::time::Instant;

pub(crate) fn run_buchberger<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions, pairs: Q, update: U) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue + PairSetView,
    U: PairUpdate<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, None)
}

pub(crate) fn run_buchberger_traced<P, F, O, Q, U>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    pairs: Q,
    update: U,
    tracer: SharedBuchbergerTracer,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue + PairSetView,
    U: PairUpdate<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, Some(tracer))
}

fn run_buchberger_impl<P, F, O, Q, U>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    mut pairs: Q,
    mut update: U,
    tracer: Option<SharedBuchbergerTracer>,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue + PairSetView,
    U: PairUpdate<P>,
{
    let trace = BuchbergerTraceCtx::new(tracer);

    let init_t0 = Instant::now();
    let mut gb = init_basis(ctx, fs, opts)?;
    trace.add_phase_time(PhaseKind::Init, init_t0.elapsed());
    trace.on_init_complete(gb.len());

    if gb.is_empty() {
        trace.print_empty_summary(&pairs);
        return Ok(gb);
    }

    let seed_t0 = Instant::now();
    seed_initial_pairs(&gb, &mut pairs, &mut update, &trace);
    trace.add_phase_time(PhaseKind::Seed, seed_t0.elapsed());

    let while_t0 = Instant::now();

    while let Some((_key, i, j)) = pairs.pop() {
        let fi = gb.get(i).ok_or(BuchbergerError::InvariantViolation)?;
        let fj = gb.get(j).ok_or(BuchbergerError::InvariantViolation)?;

        let r = compute_remainder(ctx, fi, fj, gb.as_slice(), opts, &trace)?;

        if r.is_zero() {
            trace.on_zero_reduction();
            continue;
        }

        if r.is_nonzero_constant() {
            trace.on_unit_reduction();
            trace.add_phase_time(PhaseKind::WhileLoop, while_t0.elapsed());
            trace.print_unit_summary(&pairs);
            return Ok(GrobnerBasis::new(ctx.id(), vec![r]));
        }

        let new_idx = gb.len();
        gb.push(r);
        trace.on_inserted_poly(gb.len(), pairs.len());

        let pair_update_t0 = Instant::now();
        update.on_new_poly(&gb, &mut pairs, new_idx);
        trace.add_while_time(WhileKind::PairUpdate, pair_update_t0.elapsed());
        trace.maybe_emit_progress(&gb, &pairs);
    }

    trace.add_phase_time(PhaseKind::WhileLoop, while_t0.elapsed());

    let post_t0 = Instant::now();
    postprocess_basis(ctx, &mut gb, opts)?;
    trace.add_phase_time(PhaseKind::Post, post_t0.elapsed());
    trace.set_gb_len(gb.len());
    trace.print_summary(&gb, &pairs);

    Ok(gb)
}

fn seed_initial_pairs<P, Q, U>(gb: &GrobnerBasis<P>, pairs: &mut Q, update: &mut U, trace: &BuchbergerTraceCtx)
where
    Q: PairQueue + PairSetView,
    U: PairUpdate<P>,
{
    for k in 0..gb.len() {
        let t0 = Instant::now();
        update.on_new_poly(gb, pairs, k);
        trace.add_while_time(WhileKind::PairUpdate, t0.elapsed());
        trace.on_seeded_pair(pairs.len());
    }
}

fn compute_remainder<P, F, O>(ctx: &RingCtx<F, O>, fi: &P, fj: &P, gb: &[P], _opts: BuchbergerOptions, trace: &BuchbergerTraceCtx) -> Result<P, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let s_t0 = Instant::now();
    let s = s_polynomial_in(ctx, fi, fj)?;
    trace.add_while_time(WhileKind::SPolynomial, s_t0.elapsed());

    let nf_t0 = Instant::now();
    let r = s.normal_form(ctx, gb.iter())?;
    trace.add_while_time(WhileKind::NormalForm, nf_t0.elapsed());

    Ok(r)
}

fn postprocess_basis<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>, opts: BuchbergerOptions) -> Result<(), BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    match opts.post {
        BasisPost::None => {}
        BasisPost::Minimal => minimize_in_place(ctx, gb)?,
        BasisPost::Reduced => reduce_in_place(ctx, gb)?,
    }
    Ok(())
}
