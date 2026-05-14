//! Extraction of new F4 rows from sparse pivot rows.
//!
//! Sparse elimination creates an internal pivot table. Not every pivot is a new
//! polynomial for F4.
//!
//! F4 extraction rule:
//! - if the pivot leading column was already a leading column of an input row,
//!   it corresponds to an existing reducer row and should be skipped;
//! - if the pivot leading column is new, it is a genuinely new reduced row.

use crate::algos::f4::error::Result;
use crate::linear::roman_sparse::row::SparsePivotRow;
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::Term;

/// Converts sparse pivot rows into F4 output rows.
///
/// `input_lead_cols[col] == true` means column `col` was already the leading
/// column of an input matrix row. Those pivots are internal and are skipped.
pub fn extract_new_rows_from_sparse_pivots<P, F, O>(ctx: &RingCtx<F, O>, pivots: &[SparsePivotRow<P::Coeff>], columns: &[Monomial], input_lead_cols: &[bool]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let zero = P::Coeff::default();

    let mut out = Vec::new();

    for pivot in pivots {
        // This is the key filter.
        //
        // If this pivot leading column was already the leading column
        // of a symbolic input row, it is not a new F4 row.
        // Skip before cloning monomials.
        if input_lead_cols
            .get(pivot.lead_col)
            .copied()
            .unwrap_or(false)
        {
            continue;
        }

        let mut terms = Vec::with_capacity(1 + pivot.tail.len());

        if pivot.lead_coeff != zero {
            terms.push(Term::new(pivot.lead_coeff, columns[pivot.lead_col].clone()));
        }

        for &(col, coeff) in &pivot.tail {
            if coeff == zero {
                continue;
            }

            terms.push(Term::new(coeff, columns[col].clone()));
        }

        let poly = P::from_terms_in(ctx, terms)?;

        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
}
/// Debug/helper conversion only.
///
/// This converts all sparse pivot rows into polynomial rows. Do not use this
/// as the default F4 sparse reducer output, because it returns internal reducer
/// pivots too.
pub fn sparse_pivots_to_polynomials<P, F, O>(ctx: &RingCtx<F, O>, pivots: &[SparsePivotRow<P::Coeff>], columns: &[Monomial]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq + Default,
{
    let zero = P::Coeff::default();
    let mut out = Vec::with_capacity(pivots.len());

    for pivot in pivots {
        let mut terms = Vec::with_capacity(1 + pivot.tail.len());

        if pivot.lead_coeff != zero {
            terms.push(Term::new(pivot.lead_coeff, columns[pivot.lead_col].clone()));
        }

        for &(col, coeff) in &pivot.tail {
            if coeff == zero {
                continue;
            }

            terms.push(Term::new(coeff, columns[col].clone()));
        }

        let poly = P::from_terms_in(ctx, terms)?;

        if !poly.is_zero() {
            out.push(poly);
        }
    }

    Ok(out)
}
