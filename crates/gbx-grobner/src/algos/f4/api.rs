use crate::algos::f4::engine::run_f4;
use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::basis::GrobnerBasis;
use crate::pairs::{PairQueue, StackPairs};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use std::hash::Hash;

use crate::algos::f4::trace::SharedF4Tracer;
use crate::{LcmDegreeKey, NaivePairUpdater, NoPairCriterion, PairSetView, PairUpdate};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Compute a Gröbner basis using the F4 algorithm with default pair machinery.
pub fn f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    StackPairs: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Hash,
{
    f4_with(
        ctx,
        fs,
        opts,
        StackPairs::default(),
        &mut NaivePairUpdater::new(NoPairCriterion, LcmDegreeKey),
    )
}

/// Compute a Gröbner basis using the F4 algorithm with caller-supplied pair machinery.
pub fn f4_with<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options, pairs: Q, update: &mut U) -> Result<GrobnerBasis<P>>
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
    run_f4(ctx, fs, opts, pairs, update, None)
}

pub fn f4_traced<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options, pairs: Q, update: &mut U, tracer: SharedF4Tracer) -> Result<GrobnerBasis<P>>
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
    run_f4(ctx, fs, opts, pairs, update, Some(tracer))
}
