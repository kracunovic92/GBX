//! Sequential Roman/Pearce sparse-buffer echelon reduction.

use crate::algos::f4::error::Result;
use crate::linear::roman::buffer::DenseReductionBuffer;
use crate::linear::roman::row::{SparseMatrixRow, SparsePivotRow};

use gbx_poly::ring::FieldCtx;

/// Runs sequential sparse-buffer echelon reduction.
///
/// The returned pivot rows are normalized and sorted by leading column.
pub fn sparse_echelon_sequential<F, C>(field: &F, rows: &[SparseMatrixRow<C>], ncols: usize) -> Result<Vec<SparsePivotRow<C>>>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    let mut pivot_for_col = vec![None; ncols];
    let mut pivots = Vec::new();
    let mut buffer = DenseReductionBuffer::new(ncols);

    for row_idx in sorted_row_indices(rows) {
        reduce_one_row_sequential(
            field,
            &rows[row_idx],
            &mut pivot_for_col,
            &mut pivots,
            &mut buffer,
        )?;
    }

    pivots.sort_unstable_by_key(|pivot| pivot.lead_col);

    Ok(pivots)
}

fn sorted_row_indices<C>(rows: &[SparseMatrixRow<C>]) -> Vec<usize> {
    let mut indices: Vec<_> = (0..rows.len()).collect();

    indices.sort_unstable_by_key(|&idx| rows[idx].leading_col().unwrap_or(usize::MAX));

    indices
}

fn reduce_one_row_sequential<F, C>(field: &F, row: &SparseMatrixRow<C>, pivot_for_col: &mut [Option<usize>], pivots: &mut Vec<SparsePivotRow<C>>, buffer: &mut DenseReductionBuffer<C>) -> Result<()>
where
    F: FieldCtx<Elem = C>,
    C: Copy + Eq + Default,
{
    if row.is_empty() {
        return Ok(());
    }

    buffer.clear();
    buffer.load_sparse_row(row);

    loop {
        let Some(lead_col) = buffer.leading_col() else {
            return Ok(());
        };

        if let Some(pivot_idx) = pivot_for_col[lead_col] {
            let pivot = &pivots[pivot_idx];

            buffer.reduce_by_pivot(field, pivot)?;

            continue;
        }

        let pivot = buffer.to_normalized_pivot(field, lead_col)?;

        pivot_for_col[lead_col] = Some(pivots.len());
        pivots.push(pivot);

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use gbx_field::fp::Fp;

    fn field() -> Fp {
        Fp::prime(32003).unwrap()
    }

    #[test]
    fn sorted_row_indices_orders_by_leading_column() {
        let rows = vec![SparseMatrixRow::new(vec![(5, 1)]), SparseMatrixRow::new(vec![(2, 1)]), SparseMatrixRow::new(vec![]), SparseMatrixRow::new(vec![(0, 1)])];

        assert_eq!(sorted_row_indices(&rows), vec![3, 1, 0, 2]);
    }

    #[test]
    fn independent_rows_become_pivots() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.new(1)), (2, field.new(3))]), SparseMatrixRow::new(vec![(1, field.new(1)), (3, field.new(4))])];

        let pivots = sparse_echelon_sequential(&field, &rows, 4).unwrap();

        assert_eq!(pivots.len(), 2);
        assert_eq!(pivots[0].lead_col, 0);
        assert_eq!(pivots[1].lead_col, 1);
    }

    #[test]
    fn dependent_row_reduces_to_zero() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.new(1)), (2, field.new(3))]), SparseMatrixRow::new(vec![(0, field.new(1)), (2, field.new(3))])];

        let pivots = sparse_echelon_sequential(&field, &rows, 3).unwrap();

        assert_eq!(pivots.len(), 1);
        assert_eq!(pivots[0].lead_col, 0);
    }

    #[test]
    fn pivots_are_normalized() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![(0, field.new(2)), (1, field.new(4))])];

        let pivots = sparse_echelon_sequential(&field, &rows, 2).unwrap();

        assert_eq!(pivots.len(), 1);
        assert_eq!(pivots[0].lead_col, 0);
        assert_eq!(pivots[0].lead_coeff, field.one());
        assert_eq!(pivots[0].tail, vec![(1, field.new(2))]);
    }

    #[test]
    fn empty_rows_count_as_zero_reductions() {
        let field = field();

        let rows = vec![SparseMatrixRow::new(vec![]), SparseMatrixRow::new(vec![(1, field.new(1))])];

        let pivots = sparse_echelon_sequential(&field, &rows, 3).unwrap();

        assert_eq!(pivots.len(), 1);
    }
}
