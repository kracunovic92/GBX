//! Sparse matrix construction for Roman/Pearce sparse-buffer reduction.
//!
//! This module builds the column space for an F4 batch and converts polynomial
//! rows into sparse `(column, coefficient)` entries.

use std::collections::{HashMap, HashSet};

use crate::linear::roman::row::{SparseMatrix, SparseMatrixRow};

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;

/// Builds a sparse matrix from polynomial rows.
///
/// Columns are sorted in descending monomial order, so smaller column indices
/// represent larger monomials.
pub fn build_sparse_matrix<P, O>(rows: &[P], order: &O) -> SparseMatrix<P::Coeff>
where
    P: PolynomialView,
    P::Coeff: Copy + Eq + Default,
    O: MonomialOrder + Clone,
{
    let columns = collect_distinct_columns(rows, order);
    let column_index = build_column_index(&columns);

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

                let Some(&col) = column_index.get(term.mono()) else {
                    continue;
                };

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

fn collect_distinct_columns<P, O>(rows: &[P], order: &O) -> Vec<Monomial>
where
    P: PolynomialView,
    O: MonomialOrder + Clone,
{
    let approx_terms = rows.iter().map(PolynomialView::len).sum();

    let mut seen = HashSet::with_capacity(approx_terms);
    let mut columns = Vec::new();

    for row in rows {
        for term in row.terms() {
            let mono = term.mono();

            if seen.insert(mono) {
                columns.push(mono.clone());
            }
        }
    }

    columns.sort_unstable_by(|a, b| order.cmp(b, a));
    columns.dedup();

    columns
}

fn build_column_index(columns: &[Monomial]) -> HashMap<&Monomial, usize> {
    columns
        .iter()
        .enumerate()
        .map(|(col, mono)| (mono, col))
        .collect()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use gbx_field::fp::{Fp, FpElem};
    use gbx_poly::monomial::Monomial;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::Polynomial;
    use gbx_poly::ring::{FieldCtx, Ring, RingCtx};
    use gbx_poly::term::Term;

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

    macro_rules! test_poly {
        ($ctx:expr; $(($c:expr, [$($e:expr),* $(,)?])),* $(,)?) => {
            poly!($ctx; $(($c, [$($e),*])),*).unwrap()
        };
    }

    #[test]
    fn columns_are_distinct_and_sorted_by_descending_order() {
        let ctx = test_ctx();

        let rows = vec![
            // x^2 + y
            test_poly!(&ctx; (1, [2, 0]), (1, [0, 1])),
            // x + 1
            test_poly!(&ctx; (1, [1, 0]), (1, [0, 0])),
        ];

        let sparse = build_sparse_matrix(&rows, &ctx.order);

        assert_eq!(
            sparse.columns,
            vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0]), Monomial::from_slice(&[0, 1]), Monomial::from_slice(&[0, 0]),]
        );
    }

    #[test]
    fn sparse_rows_use_sorted_column_indices() {
        let ctx = test_ctx();

        let rows = vec![
            // x^2 + y
            test_poly!(&ctx; (1, [2, 0]), (1, [0, 1])),
            // x + 1
            test_poly!(&ctx; (1, [1, 0]), (1, [0, 0])),
        ];

        let sparse = build_sparse_matrix(&rows, &ctx.order);

        assert_eq!(sparse.rows.len(), 2);

        assert_eq!(
            sparse.rows[0]
                .entries
                .iter()
                .map(|&(col, _)| col)
                .collect::<Vec<_>>(),
            vec![0, 2]
        );

        assert_eq!(
            sparse.rows[1]
                .entries
                .iter()
                .map(|&(col, _)| col)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
    }

    #[test]
    fn input_lead_cols_marks_original_leading_columns() {
        let ctx = test_ctx();

        let rows = vec![
            // x^2 + y
            test_poly!(&ctx; (1, [2, 0]), (1, [0, 1])),
            // x + 1
            test_poly!(&ctx; (1, [1, 0]), (1, [0, 0])),
        ];

        let sparse = build_sparse_matrix(&rows, &ctx.order);

        assert_eq!(sparse.input_lead_cols, vec![true, true, false, false]);
    }

    #[test]
    fn zero_coefficients_are_skipped() {
        let ctx = test_ctx();

        let zero = FieldCtx::elem(&ctx.field, 0);
        let one = FieldCtx::elem(&ctx.field, 1);

        let row = P::from_raw_parts(
            ctx.id(),
            vec![Term::new(zero, Monomial::from_slice(&[2, 0])), Term::new(one, Monomial::from_slice(&[1, 0]))],
        );

        let sparse = build_sparse_matrix(&[row], &ctx.order);

        assert_eq!(sparse.rows.len(), 1);
        assert_eq!(sparse.rows[0].entries.len(), 1);
        assert_eq!(
            sparse.columns,
            vec![Monomial::from_slice(&[2, 0]), Monomial::from_slice(&[1, 0])]
        );

        let nonzero_col = sparse.rows[0].entries[0].0;
        assert_eq!(sparse.columns[nonzero_col], Monomial::from_slice(&[1, 0]));
    }

    #[test]
    fn empty_input_builds_empty_sparse_matrix() {
        let ctx = test_ctx();

        let rows: Vec<P> = Vec::new();
        let sparse = build_sparse_matrix(&rows, &ctx.order);

        assert_eq!(sparse.nrows(), 0);
        assert_eq!(sparse.ncols(), 0);
        assert_eq!(sparse.nnz(), 0);
        assert!(sparse.density().abs() <= f64::EPSILON);
        assert!(sparse.input_lead_cols.is_empty());
    }
}
