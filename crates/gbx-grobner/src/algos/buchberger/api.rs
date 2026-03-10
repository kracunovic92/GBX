use super::engine::run_buchberger;
use super::engine::run_buchberger_traced;
use super::options::BuchbergerOptions;
use crate::buchberger::bounds::BuchbergerTerm;
use crate::buchberger::trace::SharedTracer;
use crate::{baseline_update, BuchbergerError, GrobnerBasis, PairQueue, PairUpdate, StackPairs};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialReduce};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;

/// Compute a Gröbner basis using Buchberger's algorithm and the default
/// pair-management strategy.
///
/// The default configuration uses:
///
/// - a LIFO pair queue ([`StackPairs`]),
/// - the library's baseline pair-update strategy.
///
/// Use [`buchberger_with`] to customize pair selection and pair generation.
///
/// # Errors
///
/// Returns [`BuchbergerError`] if:
///
/// - an input polynomial does not belong to `ctx`,
/// - S-polynomial construction fails,
/// - polynomial reduction or normalization fails,
/// - final post-processing fails.
pub fn buchberger<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    run_buchberger(ctx, fs, opts, StackPairs::new(), baseline_update())
}

/// Compute a Gröbner basis using Buchberger's algorithm with user-supplied
/// pair-management components.
///
/// This variant allows callers to choose:
///
/// - the pair queue `Q`, which controls pair selection order,
/// - the pair update strategy `U`, which controls how new critical pairs are
///   generated and filtered.
///
/// # Errors
///
/// Returns [`BuchbergerError`] if:
///
/// - an input polynomial does not belong to `ctx`,
/// - S-polynomial construction fails,
/// - polynomial reduction or normalization fails,
/// - final post-processing fails,
/// - an internal invariant is violated.
///
/// # Notes
///
/// The correctness of the result depends on the supplied pair-update strategy.
/// A custom update strategy must not discard required pairs.
pub fn buchberger_with<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions, pairs: Q, update: U) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    run_buchberger(ctx, fs, opts, pairs, update)
}

/// Compute a Gröbner basis using Buchberger's algorithm with user-supplied
/// pair-management components and tracing enabled.
///
/// Tracing is intended for diagnostics and benchmarking. It does not change the
/// mathematical result.
pub fn buchberger_with_tracer<P, F, O, Q, U>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    pairs: Q,
    update: U,
    tracer: SharedTracer,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    run_buchberger_traced(ctx, fs, opts, pairs, update, tracer)
}
