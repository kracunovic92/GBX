use crate::algos::f4::error::Result;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Batch linear reducer used by F4.
pub trait BatchReducer<P, F, O>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>;
}
