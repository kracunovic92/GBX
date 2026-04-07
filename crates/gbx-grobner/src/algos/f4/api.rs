use crate::algos::f4::engine::run_f4;
use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::basis::GrobnerBasis;
use std::hash::Hash;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Compute a Gröbner basis using the default F4 engine configuration.
pub fn f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Hash,
{
    run_f4(ctx, fs, opts)
}
