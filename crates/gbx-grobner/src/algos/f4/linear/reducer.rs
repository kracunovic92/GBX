use crate::algos::f4::error::Result;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;

/// Batch linear reducer used by F4.
///
/// The current implementation may operate directly on polynomial rows,
/// but this trait is designed so a future matrix-based reducer can be
/// plugged into the same engine.
pub trait BatchReducer<P, F, O>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>;
}
