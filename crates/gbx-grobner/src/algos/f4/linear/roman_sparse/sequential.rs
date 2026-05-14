//! Sequential Roman/Pearce-style sparse-buffer echelon reduction.
//!
//! This computes sparse Gaussian elimination using:
//! - sparse input rows,
//! - sparse pivot rows,
//! - one dense active-row buffer.
//!
//! Rows are processed by increasing leading column, matching the Roman/Pearce
//! sparse-buffer strategy.

use crate::algos::f4::error::Result;
use crate::linear::roman_sparse::buffer::DenseReductionBuffer;
use crate::linear::roman_sparse::row::{SparseMatrixRow, SparsePivotRow};
use gbx_poly::ring::FieldCtx;

#[derive(Debug, Default, Clone)]
pub struct RomanReductionStats {
    pub input_rows: usize,
    pub ncols: usize,
    pub input_nnz: usize,
    pub pivots_created: usize,
    pub zero_reductions: usize,
    pub pivot_reductions: usize,
    pub max_touched: usize,
}

impl RomanReductionStats {
    pub fn input_density(&self) -> f64 {
        let cells = self.input_rows.saturating_mul(self.ncols);

        if cells == 0 { 0.0 } else { self.input_nnz as f64 / cells as f64 }
    }
}

/// Sequential sparse-buffer echelon reduction.
///
/// Returns all normalized nonzero pivot rows sorted by leading column.
pub fn sparse_echelon_sequential<F, C>(field: &F, rows: &[SparseMatrixRow<C>], ncols: usize) -> Result<(Vec<SparsePivotRow<C>>, RomanReductionStats)>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    let mut stats = RomanReductionStats { input_rows: rows.len(), ncols, input_nnz: rows.iter().map(|r| r.entries.len()).sum(), ..Default::default() };

    let mut row_order: Vec<usize> = (0..rows.len()).collect();

    row_order.sort_unstable_by_key(|&idx| {
        rows[idx]
            .entries
            .first()
            .map(|&(col, _)| col)
            .unwrap_or(usize::MAX)
    });

    let mut pivot_for_col: Vec<Option<usize>> = vec![None; ncols];
    let mut pivots_out: Vec<SparsePivotRow<C>> = Vec::new();
    let mut buffer = DenseReductionBuffer::new(ncols);

    for row_idx in row_order {
        let row = &rows[row_idx];

        if row.entries.is_empty() {
            stats.zero_reductions += 1;
            continue;
        }

        buffer.clear();
        buffer.load_sparse_row(row);

        loop {
            stats.max_touched = stats.max_touched.max(buffer.touched_len());

            let Some(lead_col) = buffer.leading_col() else {
                stats.zero_reductions += 1;
                break;
            };

            if let Some(pivot_idx) = pivot_for_col[lead_col] {
                let pivot = &pivots_out[pivot_idx];

                buffer.reduce_by_pivot(field, pivot)?;
                stats.pivot_reductions += 1;

                continue;
            }

            let pivot = buffer.to_normalized_pivot(field, lead_col)?;

            pivot_for_col[lead_col] = Some(pivots_out.len());
            pivots_out.push(pivot);
            stats.pivots_created += 1;

            break;
        }
    }

    pivots_out.sort_unstable_by_key(|pivot| pivot.lead_col);

    Ok((pivots_out, stats))
}
