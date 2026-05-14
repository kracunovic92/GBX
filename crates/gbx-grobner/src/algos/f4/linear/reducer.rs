//! Shared reducer trait for F4 linear backends.

use crate::algos::f4::error::Result;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Batch reducer used by the F4 linear phase.
pub trait BatchReducer<P, F, O>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
{
    /// Reduces a batch of symbolic rows and returns newly discovered rows.
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>;
}
