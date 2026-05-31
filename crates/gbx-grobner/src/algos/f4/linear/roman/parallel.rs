//! Parallel sparse-buffer echelon reduction.
//!
//! Each worker reduces one input row using a private dense buffer. Pivot rows
//! are inserted once per leading column and become immutable after insertion.

use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::algos::f4::error::Result;
use crate::linear::roman::buffer::DenseReductionBuffer;
use crate::linear::roman::row::{SparseMatrixRow, SparsePivotRow};

use gbx_poly::ring::FieldCtx;

/// Runs parallel sparse-buffer echelon reduction.
///
/// The returned pivot rows are normalized and sorted by leading column.
///
/// # Errors
///
/// Returns an error if a pivot row cannot be normalized because its leading
/// coefficient is not invertible.
pub fn sparse_echelon_parallel<F, C>(field: &F, rows: &[SparseMatrixRow<C>], ncols: usize) -> Result<Vec<SparsePivotRow<C>>>
where
    F: FieldCtx<Elem = C> + Sync,
    C: Copy + Eq + Default + Send + Sync,
{
    let pivot_for_col: Vec<OnceLock<Arc<SparsePivotRow<C>>>> = std::iter::repeat_with(OnceLock::new).take(ncols).collect();

    let counters = ParallelReductionCounters::default();

    rows.par_iter().try_for_each_init(
        || DenseReductionBuffer::new(ncols),
        |buffer, row| reduce_one_row_parallel(field, row, &pivot_for_col, buffer, &counters),
    )?;

    let mut pivots = collect_pivots(&pivot_for_col);

    pivots.sort_unstable_by_key(|pivot| pivot.lead_col);

    Ok(pivots)
}

#[derive(Default)]
struct ParallelReductionCounters {
    zero_reductions: AtomicUsize,
    pivot_reductions: AtomicUsize,
    pivot_insert_retries: AtomicUsize,
    max_touched: AtomicUsize,
}

fn reduce_one_row_parallel<F, C>(
    field: &F,
    row: &SparseMatrixRow<C>,
    pivot_for_col: &[OnceLock<Arc<SparsePivotRow<C>>>],
    buffer: &mut DenseReductionBuffer<C>,
    counters: &ParallelReductionCounters,
) -> Result<()>
where
    F: FieldCtx<Elem = C> + Sync,
    C: Copy + Eq + Default + Send + Sync,
{
    if row.is_empty() {
        counters.zero_reductions.fetch_add(1, Ordering::Relaxed);
        return Ok(());
    }

    buffer.clear();
    buffer.load_sparse_row(row);

    loop {
        update_max(&counters.max_touched, buffer.touched_len());

        let Some(lead_col) = buffer.leading_col() else {
            counters.zero_reductions.fetch_add(1, Ordering::Relaxed);
            return Ok(());
        };

        if let Some(pivot) = pivot_for_col[lead_col].get() {
            buffer.reduce_by_pivot(field, pivot)?;
            counters.pivot_reductions.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        let candidate = Arc::new(buffer.to_normalized_pivot(field, lead_col)?);

        match pivot_for_col[lead_col].set(candidate.clone()) {
            Ok(()) => return Ok(()),

            Err(candidate) => {
                counters
                    .pivot_insert_retries
                    .fetch_add(1, Ordering::Relaxed);

                buffer.clear();
                buffer.load_pivot_row(&candidate);
            }
        }
    }
}

fn collect_pivots<C>(pivot_for_col: &[OnceLock<Arc<SparsePivotRow<C>>>]) -> Vec<SparsePivotRow<C>>
where
    C: Clone,
{
    pivot_for_col
        .iter()
        .filter_map(|cell| cell.get())
        .map(|pivot| (**pivot).clone())
        .collect()
}

fn update_max(max: &AtomicUsize, value: usize) {
    let mut current = max.load(Ordering::Relaxed);

    while value > current {
        match max.compare_exchange_weak(current, value, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(next) => current = next,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use crate::linear::roman::sequential::sparse_echelon_sequential;

    use gbx_field::fp::Fp;

    fn field() -> Fp {
        Fp::prime(32003).unwrap()
    }

    #[test]
    fn independent_rows_become_pivots() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.elem(1)), (2, field.elem(3))]), SparseMatrixRow::new(vec![(1, field.elem(1)), (3, field.elem(4))])];

        let pivots = sparse_echelon_parallel(&field, &rows, 4).unwrap();

        assert_eq!(pivots.len(), 2);
        assert_eq!(pivots[0].lead_col, 0);
        assert_eq!(pivots[1].lead_col, 1);
    }

    #[test]
    fn dependent_row_reduces_to_zero() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.elem(1)), (2, field.elem(3))]), SparseMatrixRow::new(vec![(0, field.elem(1)), (2, field.elem(3))])];

        let pivots = sparse_echelon_parallel(&field, &rows, 3).unwrap();

        assert_eq!(pivots.len(), 1);
        assert_eq!(pivots[0].lead_col, 0);
    }

    #[test]
    fn pivots_are_normalized() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.elem(2)), (1, field.elem(4))])];

        let pivots = sparse_echelon_parallel(&field, &rows, 2).unwrap();

        assert_eq!(pivots.len(), 1);
        assert_eq!(pivots[0].lead_col, 0);
        assert_eq!(pivots[0].lead_coeff, field.elem(1));
        assert_eq!(pivots[0].tail, vec![(1, field.elem(2))]);
    }

    #[test]
    fn empty_rows_count_as_zero_reductions() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![]), SparseMatrixRow::new(vec![(1, field.elem(1))])];

        let pivots = sparse_echelon_parallel(&field, &rows, 3).unwrap();

        assert_eq!(pivots.len(), 1);
    }

    #[test]
    fn parallel_matches_sequential_on_simple_input() {
        let field = field();

        let rows = vec![
            SparseMatrixRow::new(vec![(0, field.elem(1)), (2, field.elem(3))]),
            SparseMatrixRow::new(vec![(1, field.elem(2)), (2, field.elem(5))]),
            SparseMatrixRow::new(vec![(0, field.elem(4)), (3, field.elem(7))]),
            SparseMatrixRow::new(vec![(2, field.elem(1))]),
        ];

        let seq = sparse_echelon_sequential(&field, &rows, 4).unwrap();
        let par = sparse_echelon_parallel(&field, &rows, 4).unwrap();

        assert_eq!(par, seq);
    }
}
