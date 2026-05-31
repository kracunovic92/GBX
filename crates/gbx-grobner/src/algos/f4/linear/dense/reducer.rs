//! Dense F4 matrix reducer.

use crate::algos::f4::error::Result;
use crate::linear::BatchReducer;
use crate::linear::dense::build::build_dense_matrix;
use crate::linear::dense::echelon::row_echelon_dense;
use crate::linear::dense::extract::extract_new_rows_from_dense;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Dense matrix-based F4 reducer.
#[derive(Debug, Default, Clone, Copy)]
pub struct DenseF4MatrixReducer;

impl<P, F, O> BatchReducer<P, F, O> for DenseF4MatrixReducer
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        dense_matrix_reduce(ctx, rows)
    }
}

/// Reduces symbolic rows using the dense F4 matrix backend.
///
/// # Errors
///
/// Returns an error if dense row echelon reduction fails or reduced rows cannot
/// be decoded into polynomials.
pub fn dense_matrix_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let mut matrix = build_dense_matrix(rows, &ctx.order);

    row_echelon_dense(&ctx.field, &mut matrix.matrix.rows)?;

    extract_new_rows_from_dense::<P, F, O>(ctx, rows, &matrix.matrix.rows, &matrix.columns)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    #[test]
    fn dense_matrix_reduce_returns_empty_for_empty_input() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let rows: Vec<P> = Vec::new();

        let reduced = dense_matrix_reduce(&ring, &rows).unwrap();

        assert!(reduced.is_empty());
    }

    #[test]
    fn reducer_trait_delegates_to_dense_matrix_reduce() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let rows: Vec<P> = Vec::new();
        let reducer = DenseF4MatrixReducer;

        let output_rows = reducer.reduce(&ring, &rows).unwrap();

        assert!(output_rows.is_empty());
    }

    #[test]
    fn dense_matrix_reduce_extracts_new_leading_row() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        // Symbolic rows:
        //   r0 = x^2 + y
        //   r1 = x^2 + x
        //
        // After eliminating r1 by r0, a row with leading monomial x should appear.
        let rows: Vec<P> = vec![poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(), poly![&ring; (1, [2, 0]), (1, [1, 0])].unwrap()];

        let reduced = dense_matrix_reduce(&ring, &rows).unwrap();

        assert_eq!(reduced.len(), 1);
        assert_eq!(
            reduced[0].leading_mono(),
            Some(&Monomial::from_slice(&[1, 0]))
        );
    }
}
