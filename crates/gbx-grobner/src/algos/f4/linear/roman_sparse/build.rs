//! Sparse matrix builder for the Roman sparse-buffer reducer.
//!
//! This replaces the dense matrix bridge.
//!
//! It builds:
//! - a sorted global column list,
//! - sparse rows as `(column_index, coefficient)` pairs,
//! - a bitmap of input leading columns.
//!
//! Important memory rule:
//! Do not clone every monomial occurrence. Clone only distinct monomials once.

use crate::linear::roman_sparse::row::{SparseMatrix, SparseMatrixRow};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use std::collections::HashSet;

/// Builds a sparse matrix from polynomial rows.
///
/// The returned `SparseMatrix` owns:
/// - `columns`: all distinct monomials appearing in the batch,
/// - `rows`: sparse coefficient rows indexed into `columns`,
/// - `input_lead_cols`: bitmap of columns that were input leading columns.
pub fn build_sparse_matrix<P, O>(rows: &[P], order: &O) -> SparseMatrix<P::Coeff>
where
    P: PolynomialView,
    P::Coeff: Copy + Eq + Default,
    O: MonomialOrder + Clone,
{
    let mut columns = collect_distinct_columns(rows, order);
    let zero = P::Coeff::default();

    let mut input_lead_cols = vec![false; columns.len()];

    let sparse_rows = rows
        .iter()
        .map(|poly| {
            let mut entries = Vec::with_capacity(poly.len());

            for term in poly.terms() {
                let coeff = *term.coeff();

                if coeff == zero {
                    continue;
                }

                let col = find_column(&columns, order, term.mono());

                entries.push((col, coeff));
            }

            entries.sort_unstable_by_key(|&(col, _)| col);
            entries.dedup_by_key(|entry| entry.0);

            if let Some(&(lead_col, _)) = entries.first() {
                input_lead_cols[lead_col] = true;
            }

            SparseMatrixRow { entries }
        })
        .collect();

    SparseMatrix { columns, rows: sparse_rows, input_lead_cols }
}

/// Collects distinct monomials while cloning each distinct monomial only once.
///
/// We first deduplicate by borrowed monomial references. This avoids the bad
/// pattern of cloning every term monomial and then deduplicating afterward.
fn collect_distinct_columns<P, O>(rows: &[P], order: &O) -> Vec<Monomial>
where
    P: PolynomialView,
    O: MonomialOrder + Clone,
{
    let approx_terms = rows.iter().map(PolynomialView::len).sum();

    let mut seen: HashSet<&Monomial> = HashSet::with_capacity(approx_terms);
    let mut columns = Vec::new();

    for row in rows {
        for term in row.terms() {
            let mono = term.mono();

            if seen.insert(mono) {
                columns.push(mono.clone());
            }
        }
    }

    // Dense F4 matrices use columns in descending monomial order:
    // leading/largest monomial first.
    //
    // Sparse rows rely on this invariant:
    // smaller column index == larger monomial.
    columns.sort_unstable_by(|a, b| order.cmp(b, a));
    columns.dedup();

    columns
}

/// Finds the column index for a monomial.
///
/// `columns` is sorted by descending monomial order:
/// `columns.sort_unstable_by(|a, b| order.cmp(b, a))`.
#[inline]
fn find_column<O>(columns: &[Monomial], order: &O, target: &Monomial) -> usize
where
    O: MonomialOrder + Clone,
{
    columns
        .binary_search_by(|probe| order.cmp(target, probe))
        .expect("term monomial must exist in sparse matrix columns")
}
