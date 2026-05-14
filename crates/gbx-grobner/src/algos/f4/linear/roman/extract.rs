//! Extraction of F4 output rows from sparse pivot rows.
//!
//! Sparse echelon reduction creates internal pivot rows. F4 only keeps pivots
//! whose leading column was not already the leading column of an input row.

use crate::algos::f4::error::Result;
use crate::linear::roman::row::SparsePivotRow;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::Term;

/// Converts new sparse pivot rows into polynomial rows.
///
/// Pivots whose leading columns were already input leading columns are skipped.
pub fn extract_new_rows_from_sparse_pivots<P, F, O>(ctx: &RingCtx<F, O>, pivots: &[SparsePivotRow<P::Coeff>], columns: &[Monomial], input_lead_cols: &[bool]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let mut out = Vec::new();

    for pivot in pivots {
        if input_lead_cols
            .get(pivot.lead_col)
            .copied()
            .unwrap_or(false)
        {
            continue;
        }

        let poly: P = pivot_to_polynomial(ctx, pivot, columns)?;

        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
}

/// Converts all sparse pivots into polynomial rows.
///
/// This is intended for debugging and tests. It should not be used as the F4
/// reducer output because it also returns internal reducer pivots.
pub fn sparse_pivots_to_polynomials<P, F, O>(ctx: &RingCtx<F, O>, pivots: &[SparsePivotRow<P::Coeff>], columns: &[Monomial]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let mut out = Vec::with_capacity(pivots.len());

    for pivot in pivots {
        let poly: P = pivot_to_polynomial(ctx, pivot, columns)?;

        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
}

fn pivot_to_polynomial<P, F, O>(ctx: &RingCtx<F, O>, pivot: &SparsePivotRow<P::Coeff>, columns: &[Monomial]) -> Result<P>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let zero = P::Coeff::default();
    let mut terms = Vec::with_capacity(pivot.nnz());

    if pivot.lead_coeff != zero {
        terms.push(Term::new(pivot.lead_coeff, columns[pivot.lead_col].clone()));
    }

    for &(col, coeff) in &pivot.tail {
        if coeff == zero {
            continue;
        }

        terms.push(Term::new(coeff, columns[col].clone()));
    }

    Ok(P::from_terms_in(ctx, terms)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use gbx_field::fp::{Fp, FpElem};
    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::Lex;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::{FieldCtx, Ring, RingCtx};

    type P = Polynomial<FpElem>;

    fn test_ctx() -> RingCtx<Fp, Lex> {
        let field = Fp::prime(32003).unwrap();

        Ring::builder()
            .field(field)
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap()
    }

    fn columns() -> Vec<Monomial> {
        vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 1]), Monomial::from_slice(&[0, 0])]
    }

    #[test]
    fn extract_skips_pivots_with_input_leading_columns() {
        let ctx = test_ctx();
        let columns = columns();

        let pivots = vec![
            SparsePivotRow { lead_col: 0, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(2, FieldCtx::new(&ctx.field, 3))] },
            SparsePivotRow { lead_col: 1, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(3, FieldCtx::new(&ctx.field, 4))] },
        ];

        let input_lead_cols = vec![true, false, false, false];

        let out: Vec<P> = extract_new_rows_from_sparse_pivots(&ctx, &pivots, &columns, &input_lead_cols).unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].leading_mono(), Some(&Monomial::from_slice(&[1, 0])));
    }

    #[test]
    fn extract_treats_missing_input_lead_marker_as_not_input_leading() {
        let ctx = test_ctx();
        let columns = columns();

        let pivots = vec![SparsePivotRow { lead_col: 2, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(3, FieldCtx::new(&ctx.field, 5))] }];

        let input_lead_cols = vec![false];

        let out: Vec<P> = extract_new_rows_from_sparse_pivots(&ctx, &pivots, &columns, &input_lead_cols).unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].leading_mono(), Some(&Monomial::from_slice(&[0, 1])));
    }

    #[test]
    fn sparse_pivots_to_polynomials_converts_all_nonzero_pivots() {
        let ctx = test_ctx();
        let columns = columns();

        let pivots = vec![
            SparsePivotRow { lead_col: 0, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(2, FieldCtx::new(&ctx.field, 3))] },
            SparsePivotRow { lead_col: 1, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(3, FieldCtx::new(&ctx.field, 4))] },
        ];

        let out: Vec<P> = sparse_pivots_to_polynomials(&ctx, &pivots, &columns).unwrap();

        assert_eq!(out.len(), 2);
        assert_eq!(out[0].leading_mono(), Some(&Monomial::from_slice(&[2, 0])));
        assert_eq!(out[1].leading_mono(), Some(&Monomial::from_slice(&[1, 0])));
    }

    #[test]
    fn zero_tail_entries_are_skipped() {
        let ctx = test_ctx();
        let columns = columns();

        let pivot = SparsePivotRow { lead_col: 1, lead_coeff: FieldCtx::new(&ctx.field, 1), tail: vec![(2, FieldCtx::new(&ctx.field, 0)), (3, FieldCtx::new(&ctx.field, 7))] };

        let out: Vec<P> = sparse_pivots_to_polynomials(&ctx, &[pivot], &columns).unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].len(), 2);
        assert_eq!(out[0].leading_mono(), Some(&Monomial::from_slice(&[1, 0])));
    }

    #[test]
    fn zero_leading_coeff_can_produce_tail_polynomial() {
        let ctx = test_ctx();
        let columns = columns();

        let pivot = SparsePivotRow { lead_col: 1, lead_coeff: FieldCtx::new(&ctx.field, 0), tail: vec![(2, FieldCtx::new(&ctx.field, 7))] };

        let out: Vec<P> = sparse_pivots_to_polynomials(&ctx, &[pivot], &columns).unwrap();

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].leading_mono(), Some(&Monomial::from_slice(&[0, 1])));
    }
}
