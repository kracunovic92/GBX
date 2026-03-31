use super::bounds::BuchbergerTerm;
use super::init::init_basis;
use super::options::BuchbergerOptions;
use super::trace::{BuchbergerTraceCtx, SharedBuchbergerTracer, WhileKind};
use crate::algos::post::BasisPostOptionsKind;
use crate::algos::{minimize_in_place, reduce_in_place};
use crate::trace::CorePhaseKind;
use crate::{s_polynomial_in, BuchbergerError, GrobnerBasis, PairFilter, PairQueue, PairSetView, PairUpdate};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;
use std::time::Instant;

struct PreparedReducer<P>
where
    P: PolynomialView + Clone,
    P::Term: TermView,
{
    poly: P,
    lm: <P::Term as TermView>::Mono,
    inv_lc: <P::Term as TermView>::Coeff,
    lm_degree: u32,
}

pub(crate) fn run_buchberger<P, F, O, Q, U, Pf>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    pairs: Q,
    update: U,
    filter: Pf,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
    Pf: PairFilter<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, filter, None)
}

pub(crate) fn run_buchberger_traced<P, F, O, Q, U, Pf>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    pairs: Q,
    update: U,
    filter: Pf,
    tracer: SharedBuchbergerTracer,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
    Pf: PairFilter<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, filter, Some(tracer))
}

fn run_buchberger_impl<P, F, O, Q, U, Pf>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    mut pairs: Q,
    mut update: U,
    mut filter: Pf,
    tracer: Option<SharedBuchbergerTracer>,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
    Pf: PairFilter<P>,
{
    let trace = BuchbergerTraceCtx::new(tracer);

    let mut gb = init_basis_traced(ctx, fs, opts, &trace)?;
    if gb.is_empty() {
        trace.print_empty_summary(&pairs);
        return Ok(gb);
    }

    let mut reducers = build_reducers(ctx, gb.as_slice())?;

    seed_initial_pairs_traced(&gb, &mut pairs, &mut update, &trace)?;

    let while_t0 = Instant::now();

    while let Some(pair) = pairs.pop() {
        let i = pair.i;
        let j = pair.j;

        if !should_process_pair_traced(&gb, &pairs, &mut filter, i, j, &trace) {
            continue;
        }

        let fi = gb.get(i).ok_or(BuchbergerError::InvariantViolation)?;
        let fj = gb.get(j).ok_or(BuchbergerError::InvariantViolation)?;

        let r = compute_remainder_traced(ctx, fi, fj, &reducers, &trace)?;

        if r.is_zero() {
            trace.on_zero_reduction();
            continue;
        }

        if r.is_nonzero_constant() {
            trace.on_unit_reduction();
            trace.add_phase_time(CorePhaseKind::WhileLoop, while_t0.elapsed());
            trace.print_unit_summary(&pairs);
            return Ok(GrobnerBasis::new(ctx.id(), vec![r]));
        }

        let new_index = gb.len();
        gb.push(r);

        let new_poly = gb
            .get(new_index)
            .ok_or(BuchbergerError::InvariantViolation)?;
        insert_reducer_sorted(ctx, &mut reducers, new_poly.clone())?;

        trace.on_inserted_poly(gb.len(), pairs.len());

        update_after_insert_traced(&gb, &mut pairs, &mut update, new_index, &trace)?;
    }

    trace.add_phase_time(CorePhaseKind::WhileLoop, while_t0.elapsed());

    postprocess_basis_traced(ctx, &mut gb, opts, &trace)?;
    trace.set_gb_len(gb.len());
    trace.print_summary(&gb, &pairs);

    Ok(gb)
}

fn init_basis_traced<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions, trace: &BuchbergerTraceCtx) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let t0 = Instant::now();
    let gb = init_basis(ctx, fs, opts)?;
    trace.add_phase_time(CorePhaseKind::Init, t0.elapsed());
    trace.on_init_complete(gb.len());
    Ok(gb)
}

fn mono_degree<M>(m: &M) -> Result<u32, BuchbergerError>
where
    M: Monomial + MonomialAlgos + MonomialView<Word = u32>,
{
    m.degree_hint().ok_or(BuchbergerError::InvariantViolation)
}

fn build_reducers<P, F, O>(ctx: &RingCtx<F, O>, basis: &[P]) -> Result<Vec<PreparedReducer<P>>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut reducers = Vec::new();

    for g in basis.iter().filter(|g| !g.is_zero()) {
        let lt = g
            .leading_term()
            .ok_or(BuchbergerError::InvariantViolation)?;

        let inv_lc = ctx
            .field
            .try_inv(*lt.coeff())
            .ok_or(BuchbergerError::InvariantViolation)?;

        reducers.push(PreparedReducer { poly: g.clone(), lm: lt.mono().clone(), inv_lc, lm_degree: mono_degree(lt.mono())? });
    }

    reducers.sort_by(|a, b| ctx.order.cmp(&a.lm, &b.lm));
    Ok(reducers)
}

fn insert_reducer_sorted<P, F, O>(ctx: &RingCtx<F, O>, reducers: &mut Vec<PreparedReducer<P>>, poly: P) -> Result<(), BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    if poly.is_zero() {
        return Ok(());
    }

    let (lm, inv_lc, lm_degree) = {
        let lt = poly
            .leading_term()
            .ok_or(BuchbergerError::InvariantViolation)?;

        let lm = lt.mono().clone();
        let inv_lc = ctx
            .field
            .try_inv(*lt.coeff())
            .ok_or(BuchbergerError::InvariantViolation)?;
        let lm_degree = mono_degree(&lm)?;

        (lm, inv_lc, lm_degree)
    };

    let prepared = PreparedReducer { poly, lm, inv_lc, lm_degree };

    let pos = reducers
        .binary_search_by(|probe| ctx.order.cmp(&probe.lm, &prepared.lm))
        .unwrap_or_else(|pos| pos);

    reducers.insert(pos, prepared);
    Ok(())
}

fn seed_initial_pairs_traced<P, Q, U>(gb: &GrobnerBasis<P>, pairs: &mut Q, update: &mut U, trace: &BuchbergerTraceCtx) -> Result<(), BuchbergerError>
where
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
{
    let phase_t0 = Instant::now();

    for k in 0..gb.len() {
        let t0 = Instant::now();
        update
            .on_new_poly(gb, pairs, k)
            .map_err(BuchbergerError::from)?;
        trace.add_while_time(WhileKind::PairUpdate, t0.elapsed());
    }

    trace.add_phase_time(CorePhaseKind::Seed, phase_t0.elapsed());
    Ok(())
}

fn should_process_pair_traced<P, Q, Pf>(gb: &GrobnerBasis<P>, pairs: &Q, filter: &mut Pf, i: usize, j: usize, trace: &BuchbergerTraceCtx) -> bool
where
    Q: PairSetView,
    Pf: PairFilter<P>,
{
    let t0 = Instant::now();
    let keep = filter.keep_pair(gb, pairs, i, j);
    trace.add_while_time(WhileKind::PairFilter, t0.elapsed());
    keep
}

fn compute_remainder_traced<P, F, O>(ctx: &RingCtx<F, O>, fi: &P, fj: &P, reducers: &[PreparedReducer<P>], trace: &BuchbergerTraceCtx) -> Result<P, BuchbergerError>
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
    let r = s.normal_form_prepared(
        ctx,
        reducers
            .iter()
            .map(|red| (&red.poly, &red.lm, red.inv_lc, red.lm_degree)),
    )?;
    trace.add_while_time(WhileKind::NormalForm, nf_t0.elapsed());

    Ok(r)
}

fn update_after_insert_traced<P, Q, U>(gb: &GrobnerBasis<P>, pairs: &mut Q, update: &mut U, new_index: usize, trace: &BuchbergerTraceCtx) -> Result<(), BuchbergerError>
where
    Q: PairQueue<Key = U::Key> + PairSetView,
    U: PairUpdate<P>,
    P: PolynomialView,
{
    let t0 = Instant::now();
    update
        .on_new_poly(gb, pairs, new_index)
        .map_err(BuchbergerError::from)?;
    trace.add_while_time(WhileKind::PairUpdate, t0.elapsed());
    trace.maybe_emit_progress(gb, pairs);
    Ok(())
}

fn postprocess_basis_traced<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>, opts: BuchbergerOptions, trace: &BuchbergerTraceCtx) -> Result<(), BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let t0 = Instant::now();

    match opts.post {
        BasisPostOptionsKind::None => {}
        BasisPostOptionsKind::Minimal => minimize_in_place(ctx, gb)?,
        BasisPostOptionsKind::Reduced => reduce_in_place(ctx, gb)?,
    }

    trace.add_phase_time(CorePhaseKind::Post, t0.elapsed());
    Ok(())
}
