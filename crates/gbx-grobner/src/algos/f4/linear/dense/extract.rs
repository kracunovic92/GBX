//! Extraction of new F4 rows from reduced dense matrix rows.

use std::collections::HashSet;

use crate::algos::f4::error::Result;
use crate::linear::dense::echelon::first_nonzero_col;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::Term;

/// Collects leading monomials of the original symbolic rows.
pub fn collect_symbolic_heads<P>(rows: &[P]) -> HashSet<Monomial>
where
    P: PolynomialView,
{
    rows.iter()
        .filter_map(|row| row.leading_mono().cloned())
        .collect()
}

/// Decodes one dense row into a polynomial.
///
/// # Errors
///
/// Returns an error if polynomial construction or normalization fails.
pub fn decode_dense_row<P, F, O>(ctx: &RingCtx<F, O>, coeffs: &[P::Coeff], columns: &[Monomial]) -> Result<P>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let mut terms = Vec::new();
    let zero = P::Coeff::default();

    for (coeff, mono) in coeffs.iter().zip(columns.iter()) {
        if *coeff == zero {
            continue;
        }

        terms.push(Term::new(*coeff, mono.clone()));
    }

    Ok(P::from_terms_in(ctx, terms)?)
}

/// Extracts rows whose leading monomial is new relative to symbolic input.
///
/// # Errors
///
/// Returns an error if any extracted dense row cannot be decoded into a
/// polynomial.
pub fn extract_new_rows_from_dense<P, F, O>(ctx: &RingCtx<F, O>, symbolic_rows: &[P], reduced_rows: &[Vec<P::Coeff>], columns: &[Monomial]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let symbolic_heads = collect_symbolic_heads(symbolic_rows);
    let mut out = Vec::new();

    for row in reduced_rows {
        let Some(head_col) = first_nonzero_col(row) else {
            continue;
        };

        let head_mono = &columns[head_col];

        if symbolic_heads.contains(head_mono) {
            continue;
        }

        let poly = decode_dense_row::<P, F, O>(ctx, row, columns)?;

        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
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
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::FieldCtx;

    type P = Polynomial<FpElem>;

    #[test]
    fn collect_symbolic_heads_skips_zero_rows() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let zero = P::zero_in(&ring);
        let row: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();

        let heads = collect_symbolic_heads(&[zero, row]);

        assert_eq!(heads.len(), 1);
        assert!(heads.contains(&Monomial::from_slice(&[2, 0])));
    }

    #[test]
    fn decode_dense_row_skips_zero_coefficients() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let columns = vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let coeffs = vec![FieldCtx::elem(&ring.field, 0), FieldCtx::elem(&ring.field, 3), FieldCtx::elem(&ring.field, 5)];

        let row: P = decode_dense_row(&ring, &coeffs, &columns).unwrap();

        assert_eq!(row.len(), 2);
        assert_eq!(row.leading_mono(), Some(&Monomial::from_slice(&[1, 0])));
    }

    #[test]
    fn decode_zero_dense_row_returns_zero_polynomial() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let columns = vec![Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let coeffs = vec![FieldCtx::elem(&ring.field, 0), FieldCtx::elem(&ring.field, 0)];

        let row: P = decode_dense_row(&ring, &coeffs, &columns).unwrap();

        assert!(row.is_zero());
    }

    #[test]
    fn extract_skips_rows_with_existing_symbolic_heads() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let symbolic_rows: Vec<P> = vec![poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap()];

        let columns = vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let reduced_rows = vec![vec![FieldCtx::elem(&ring.field, 1), FieldCtx::elem(&ring.field, 0), FieldCtx::elem(&ring.field, 3)]];

        let extracted: Vec<P> = extract_new_rows_from_dense(&ring, &symbolic_rows, &reduced_rows, &columns).unwrap();

        assert!(extracted.is_empty());
    }

    #[test]
    fn extract_keeps_rows_with_new_heads() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let symbolic_rows: Vec<P> = vec![poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap()];

        let columns = vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let reduced_rows = vec![vec![FieldCtx::elem(&ring.field, 0), FieldCtx::elem(&ring.field, 1), FieldCtx::elem(&ring.field, 6)]];

        let extracted: Vec<P> = extract_new_rows_from_dense(&ring, &symbolic_rows, &reduced_rows, &columns).unwrap();

        assert_eq!(extracted.len(), 1);
        assert_eq!(
            extracted[0].leading_mono(),
            Some(&Monomial::from_slice(&[1, 0]))
        );
    }

    #[test]
    fn extract_skips_zero_reduced_rows() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let symbolic_rows: Vec<P> = Vec::new();

        let columns = vec![Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 0])];

        let reduced_rows = vec![vec![FieldCtx::elem(&ring.field, 0), FieldCtx::elem(&ring.field, 0)]];

        let extracted: Vec<P> = extract_new_rows_from_dense(&ring, &symbolic_rows, &reduced_rows, &columns).unwrap();

        assert!(extracted.is_empty());
    }
}
