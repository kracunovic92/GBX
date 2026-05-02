use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::types::{SymbolicProduct, SymbolicSource};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Evaluate `mult((t, f)) = t * f`.
///
/// This is the `mult(...)` operation from F4 symbolic preprocessing.
pub fn materialize_product<P, F, O>(ctx: &RingCtx<F, O>, product: &SymbolicProduct<Monomial>, basis: &[P], history: &[BatchHistory<P>]) -> Result<P>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let base = resolve_product_source(product, basis, history)?;

    let mut out = base.clone();
    out.mul_monomial_assign_raw(ctx, &product.multiplier)?;
    out.normalize_in_place(ctx)?;

    Ok(out)
}

/// Resolve the polynomial referenced by a symbolic product source.
fn resolve_product_source<'a, P>(product: &SymbolicProduct<Monomial>, basis: &'a [P], history: &'a [BatchHistory<P>]) -> Result<&'a P> {
    match &product.source {
        SymbolicSource::Basis(index) => basis
            .get(*index)
            .ok_or(F4Error::MissingBasisPolynomial { index: *index }),

        SymbolicSource::HistoryReducedRow { batch_index, row_index } => history
            .get(*batch_index)
            .and_then(|batch| batch.f_j_tilde.get(*row_index))
            .ok_or(F4Error::MissingHistoryRow { batch_index: *batch_index, row_index: *row_index }),
    }
}

pub fn materialize_products<P, F, O>(ctx: &RingCtx<F, O>, products: &[SymbolicProduct<Monomial>], basis: &[P], history: &[BatchHistory<P>]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    products
        .iter()
        .map(|product| materialize_product(ctx, product, basis, history))
        .collect()
}
