use crate::algos::f4::error::Result;

use crate::linear::build::build_dense_matrix;
use crate::linear::extract::extract_new_rows_from_dense;
use crate::linear::roman_sparse::parallel::sparse_echelon_parallel;
use crate::linear::roman_sparse::row::{SparseMatrixRow, SparsePivotRow};
use crate::linear::roman_sparse::sequential::sparse_echelon_sequential;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Sequential Roman reducer using current dense builder/extractor as bridge.
pub fn roman_sparse_buffer_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let dense = build_dense_matrix(rows, &ctx.order);

    let ncols = dense.columns.len();
    let sparse_rows = dense_rows_to_sparse_rows(&dense.matrix.rows);

    let (pivots, _stats) = sparse_echelon_sequential(&ctx.field, &sparse_rows, ncols)?;

    let dense_pivot_rows = sparse_pivots_to_dense_rows(&pivots, ncols);

    extract_new_rows_from_dense::<P, F, O>(ctx, rows, &dense_pivot_rows, &dense.columns)
}

/// Parallel Roman reducer using current dense builder/extractor as bridge.
pub fn roman_sparse_buffer_reduce_parallel<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = P::Coeff> + Sync,
    O: MonomialOrder + Clone + Sync,
    P: PolynomialMut + PolynomialView + Clone + Send + Sync,
    P::Coeff: Copy + Eq + Default + Send + Sync,
{
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let dense = build_dense_matrix(rows, &ctx.order);

    let ncols = dense.columns.len();
    let sparse_rows = dense_rows_to_sparse_rows(&dense.matrix.rows);

    let (pivots, _stats) = sparse_echelon_parallel(&ctx.field, &sparse_rows, ncols)?;

    let dense_pivot_rows = sparse_pivots_to_dense_rows(&pivots, ncols);

    extract_new_rows_from_dense::<P, F, O>(ctx, rows, &dense_pivot_rows, &dense.columns)
}

fn dense_rows_to_sparse_rows<C>(dense_rows: &[Vec<C>]) -> Vec<SparseMatrixRow<C>>
where
    C: Copy + Eq + Default,
{
    let zero = C::default();

    dense_rows
        .iter()
        .map(|row| {
            let entries = row
                .iter()
                .copied()
                .enumerate()
                .filter_map(
                    |(col, coeff)| {
                        if coeff != zero { Some((col, coeff)) } else { None }
                    },
                )
                .collect();

            SparseMatrixRow { entries }
        })
        .collect()
}

fn sparse_pivots_to_dense_rows<C>(pivots: &[SparsePivotRow<C>], ncols: usize) -> Vec<Vec<C>>
where
    C: Copy + Eq + Default,
{
    let zero = C::default();

    pivots
        .iter()
        .map(|pivot| {
            let mut row = vec![zero; ncols];

            row[pivot.lead_col] = pivot.lead_coeff;

            for &(col, coeff) in &pivot.tail {
                row[col] = coeff;
            }

            row
        })
        .collect()
}
