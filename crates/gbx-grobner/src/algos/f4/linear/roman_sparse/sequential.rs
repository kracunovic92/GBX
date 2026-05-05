use std::sync::Arc;

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

/// Sequential Roman/Pearce-style sparse-buffer echelon reduction.
pub fn sparse_echelon_sequential<F, C>(field: &F, rows: &[SparseMatrixRow<C>], ncols: usize) -> Result<(Vec<SparsePivotRow<C>>, RomanReductionStats)>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    let mut stats = RomanReductionStats { input_rows: rows.len(), ncols, input_nnz: rows.iter().map(|r| r.entries.len()).sum(), ..Default::default() };

    let mut pivot_for_col: Vec<Option<Arc<SparsePivotRow<C>>>> = vec![None; ncols];
    let mut pivots_out = Vec::new();

    let mut buffer = DenseReductionBuffer::new(ncols);

    for row in rows {
        buffer.clear();
        buffer.load_sparse_row(row);

        loop {
            stats.max_touched = stats.max_touched.max(buffer.touched_len());

            let Some(lead_col) = buffer.leading_col() else {
                stats.zero_reductions += 1;
                break;
            };

            if let Some(pivot) = pivot_for_col[lead_col].as_ref() {
                buffer.reduce_by_pivot(field, pivot)?;
                stats.pivot_reductions += 1;
                continue;
            }

            let pivot = Arc::new(buffer.to_normalized_pivot(field, lead_col)?);

            pivot_for_col[lead_col] = Some(pivot.clone());
            pivots_out.push((*pivot).clone());
            stats.pivots_created += 1;

            break;
        }
    }

    pivots_out.sort_unstable_by_key(|p| p.lead_col);

    Ok((pivots_out, stats))
}
