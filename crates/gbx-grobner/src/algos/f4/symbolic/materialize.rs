//! Materialization of symbolic products.

use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::types::{ProductSource, UnevaluatedProduct};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Materializes an unevaluated product into a polynomial row.
///
/// The source row is cloned and multiplied by the product multiplier.
pub fn materialize_product<P, F, O>(ctx: &RingCtx<F, O>, product: &UnevaluatedProduct<Monomial>, basis: &[P], history: &[BatchHistory<P>]) -> Result<P>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let base = resolve_product_source(product.source, basis, history)?;

    let mut row = base.clone();

    row.mul_monomial_assign_raw(ctx, &product.multiplier)?;

    Ok(row)
}

fn resolve_product_source<'a, P>(source: ProductSource, basis: &'a [P], history: &'a [BatchHistory<P>]) -> Result<&'a P> {
    match source {
        ProductSource::Basis(index) => basis
            .get(index)
            .ok_or(F4Error::MissingBasisPolynomial { index }),

        ProductSource::HistoryReducedRow { batch_index, row_index } => history
            .get(batch_index)
            .and_then(|batch| batch.f_j_tilde.get(row_index))
            .ok_or(F4Error::MissingHistoryRow { batch_index, row_index }),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::algos::f4::symbolic::types::UnevaluatedProduct;
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    #[test]
    fn materializes_basis_product() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f: P = poly![&ring; (1, [1, 0]), (2, [0, 1])].unwrap();
        let basis = vec![f];

        let product = UnevaluatedProduct::from_basis(0, Monomial::from_slice(&[1, 1]));

        let row = materialize_product(&ring, &product, &basis, &[]).unwrap();

        assert_eq!(row.len(), 2);
        assert_eq!(row.leading_mono(), Some(&Monomial::from_slice(&[2, 1])));
    }

    #[test]
    fn missing_basis_source_returns_error() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let product = UnevaluatedProduct::from_basis(4, Monomial::from_slice(&[0, 0]));

        let err = materialize_product::<P, _, _>(&ring, &product, &[], &[]).unwrap_err();

        assert!(matches!(err, F4Error::MissingBasisPolynomial { index: 4 }));
    }

    #[test]
    fn missing_history_source_returns_error() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let product = UnevaluatedProduct::from_history_reduced_row(1, 2, Monomial::from_slice(&[0, 0]));

        let err = materialize_product::<P, _, _>(&ring, &product, &[], &[]).unwrap_err();

        assert!(matches!(
            err,
            F4Error::MissingHistoryRow { batch_index: 1, row_index: 2 }
        ));
    }
}
