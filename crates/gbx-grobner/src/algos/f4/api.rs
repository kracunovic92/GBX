//! Entry point for my algo
use crate::algos::f4::engine::run_f4;
use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::basis::GrobnerBasis;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

use std::hash::Hash;

/// Compute a Gröbner basis using the default F4 engine configuration.
pub fn f4<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: F4Options) -> Result<GrobnerBasis<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone + Send + std::marker::Sync,
    P::Coeff: Copy + Eq + Default,
    Monomial: Clone + Eq + Hash + Default,
    <P as PolynomialView>::Coeff: Send,
    <P as PolynomialView>::Coeff: Sync,
{
    run_f4(ctx, fs, opts.validated()?)
}
