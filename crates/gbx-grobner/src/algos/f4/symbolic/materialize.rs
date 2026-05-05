use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::types::{ProductSource, UnevaluatedProduct};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Materializes an unevaluated product into a polynomial row.
///
/// This implements the `mult(...)` operation from F4 symbolic preprocessing:
///
/// ```text
/// mult(m, f) = m * f
/// ```
///
/// The product source may refer either to a current basis polynomial or to a
/// reduced row from previous batch history.
pub fn materialize_product<P, F, O>(ctx: &RingCtx<F, O>, product: &UnevaluatedProduct<Monomial>, basis: &[P], history: &[BatchHistory<P>]) -> Result<P>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let base = resolve_product_source(product, basis, history)?;

    let mut out = base.clone();
    out.mul_monomial_assign_raw(ctx, &product.multiplier)?;

    Ok(out)
}

/// Resolves the source row referenced by an unevaluated product.
fn resolve_product_source<'a, P>(product: &UnevaluatedProduct<Monomial>, basis: &'a [P], history: &'a [BatchHistory<P>]) -> Result<&'a P> {
    match product.source {
        ProductSource::Basis(index) => basis
            .get(index)
            .ok_or(F4Error::MissingBasisPolynomial { index }),

        ProductSource::HistoryReducedRow { batch_index, row_index } => history
            .get(batch_index)
            .and_then(|batch| batch.f_j_tilde.get(row_index))
            .ok_or(F4Error::MissingHistoryRow { batch_index, row_index }),
    }
}
