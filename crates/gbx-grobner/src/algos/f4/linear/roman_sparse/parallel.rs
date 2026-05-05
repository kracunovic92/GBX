use std::sync::{
    atomic::{AtomicUsize, Ordering}, Arc,
    OnceLock,
};

use rayon::prelude::*;

use crate::algos::f4::error::Result;
use crate::linear::roman_sparse::buffer::DenseReductionBuffer;
use crate::linear::roman_sparse::row::{SparseMatrixRow, SparsePivotRow};
use crate::linear::roman_sparse::sequential::RomanReductionStats;
use gbx_poly::ring::FieldCtx;

/// Parallel Roman/Pearce-style sparse-buffer echelon reduction.
///
/// Each worker gets its own dense buffer.
/// Pivots are immutable after insertion.
/// Pivot insertion is insert-once using OnceLock.
pub fn sparse_echelon_parallel<F, C>(field: &F, rows: &[SparseMatrixRow<C>], ncols: usize) -> Result<(Vec<SparsePivotRow<C>>, RomanReductionStats)>
where
    F: FieldCtx<Elem = C> + Sync,
    C: Copy + Eq + Default + Send + Sync,
{
    let pivot_for_col: Vec<OnceLock<Arc<SparsePivotRow<C>>>> = std::iter::repeat_with(OnceLock::new).take(ncols).collect();

    let input_nnz: usize = rows.iter().map(|r| r.entries.len()).sum();

    let zero_reductions = AtomicUsize::new(0);
    let pivot_reductions = AtomicUsize::new(0);
    let pivot_insert_retries = AtomicUsize::new(0);
    let max_touched = AtomicUsize::new(0);

    rows.par_iter().try_for_each_init(
        || DenseReductionBuffer::new(ncols),
        |buffer, row| {
            reduce_one_row_parallel(
                field,
                row,
                &pivot_for_col,
                buffer,
                &zero_reductions,
                &pivot_reductions,
                &pivot_insert_retries,
                &max_touched,
            )
        },
    )?;

    let mut pivots_out = Vec::new();

    for cell in &pivot_for_col {
        if let Some(pivot) = cell.get() {
            pivots_out.push((**pivot).clone());
        }
    }

    pivots_out.sort_unstable_by_key(|p| p.lead_col);

    let stats = RomanReductionStats {
        input_rows: rows.len(),
        ncols,
        input_nnz,
        pivots_created: pivots_out.len(),
        zero_reductions: zero_reductions.load(Ordering::Relaxed),
        pivot_reductions: pivot_reductions.load(Ordering::Relaxed),
        max_touched: max_touched.load(Ordering::Relaxed),
    };

    // You may want to log this separately because RomanReductionStats
    // currently does not contain this field.
    let _retries = pivot_insert_retries.load(Ordering::Relaxed);

    Ok((pivots_out, stats))
}

fn reduce_one_row_parallel<F, C>(
    field: &F,
    row: &SparseMatrixRow<C>,
    pivot_for_col: &[OnceLock<Arc<SparsePivotRow<C>>>],
    buffer: &mut DenseReductionBuffer<C>,
    zero_reductions: &AtomicUsize,
    pivot_reductions: &AtomicUsize,
    pivot_insert_retries: &AtomicUsize,
    max_touched: &AtomicUsize,
) -> Result<()>
where
    F: FieldCtx<Elem = C> + Sync,
    C: Copy + Eq + Default + Send + Sync,
{
    buffer.clear();
    buffer.load_sparse_row(row);

    loop {
        update_max(max_touched, buffer.touched_len());

        let Some(lead_col) = buffer.leading_col() else {
            zero_reductions.fetch_add(1, Ordering::Relaxed);
            buffer.clear();
            return Ok(());
        };

        if let Some(pivot) = pivot_for_col[lead_col].get() {
            buffer.reduce_by_pivot(field, pivot)?;
            pivot_reductions.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        let candidate = Arc::new(buffer.to_normalized_pivot(field, lead_col)?);

        match pivot_for_col[lead_col].set(candidate.clone()) {
            Ok(()) => {
                buffer.clear();
                return Ok(());
            }

            Err(candidate) => {
                // Another thread inserted a pivot for this lead column first.
                // Our candidate must be reduced again by the now-existing pivot.
                pivot_insert_retries.fetch_add(1, Ordering::Relaxed);

                buffer.clear();
                buffer.load_pivot_row(&candidate);
            }
        }
    }
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
