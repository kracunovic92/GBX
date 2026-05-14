//! Dense one-row accumulator for sparse-buffer row reduction.
//!
//! This is **not** a dense matrix.
//!
//! `DenseReductionBuffer` stores one active row of length `ncols` and keeps a
//! list of touched columns so clearing is proportional to the number of touched
//! entries, not the full number of columns.
//!
//! This matches the Roman/Pearce style:
//!
//! ```text
//! sparse row -> dense buffer -> reduce by sparse pivots -> sparse pivot row
//! ```

use crate::algos::f4::error::{F4Error, Result};
use crate::linear::roman::row::{SparseMatrixRow, SparsePivotRow};
use gbx_poly::ring::FieldCtx;

/// Dense temporary accumulator used to reduce one sparse matrix row.
#[derive(Debug, Clone)]
pub struct DenseReductionBuffer<C> {
    values: Vec<C>,
    touched: Vec<usize>,
    marked: Vec<bool>,
}

impl<C> DenseReductionBuffer<C>
where
    C: Copy + Eq + Default,
{
    /// Creates a new one-row buffer with `ncols` columns.
    ///
    /// Memory usage is `O(ncols)`, not `O(nrows * ncols)`.
    pub fn new(ncols: usize) -> Self {
        Self { values: vec![C::default(); ncols], touched: Vec::new(), marked: vec![false; ncols] }
    }

    /// Number of currently touched columns.
    #[inline]
    pub fn touched_len(&self) -> usize {
        self.touched.len()
    }

    /// Number of columns in the buffer.
    #[inline]
    pub fn ncols(&self) -> usize {
        self.values.len()
    }

    /// Reads a coefficient.
    #[inline]
    pub fn value_at(&self, col: usize) -> C {
        self.values[col]
    }

    /// Clears only columns that were touched since the last clear.
    #[inline]
    pub fn clear(&mut self) {
        let zero = C::default();

        for &col in &self.touched {
            self.values[col] = zero;
            self.marked[col] = false;
        }

        self.touched.clear();
    }

    #[inline]
    fn mark_touched(&mut self, col: usize) {
        if !self.marked[col] {
            self.marked[col] = true;
            self.touched.push(col);
        }
    }

    #[inline]
    fn write_value(&mut self, col: usize, value: C) {
        if value != C::default() {
            self.mark_touched(col);
        }

        self.values[col] = value;
    }

    /// Loads a sparse matrix row into the buffer.
    ///
    /// The buffer must be clear before this is called.
    pub fn load_sparse_row(&mut self, row: &SparseMatrixRow<C>) {
        debug_assert!(
            self.touched.is_empty(),
            "DenseReductionBuffer must be clear before loading a row"
        );

        let zero = C::default();

        for &(col, coeff) in &row.entries {
            debug_assert!(col < self.ncols(), "sparse row column index out of bounds");

            if coeff != zero {
                self.write_value(col, coeff);
            }
        }
    }

    /// Loads a sparse pivot row into the buffer.
    ///
    /// This is mostly useful for tests and debugging.
    pub fn load_pivot_row(&mut self, pivot: &SparsePivotRow<C>) {
        debug_assert!(
            self.touched.is_empty(),
            "DenseReductionBuffer must be clear before loading a pivot"
        );

        let zero = C::default();

        debug_assert!(
            pivot.lead_col < self.ncols(),
            "pivot leading column out of bounds"
        );

        if pivot.lead_coeff != zero {
            self.write_value(pivot.lead_col, pivot.lead_coeff);
        }

        for &(col, coeff) in &pivot.tail {
            debug_assert!(col < self.ncols(), "pivot tail column out of bounds");

            if coeff != zero {
                self.write_value(col, coeff);
            }
        }
    }

    /// Finds the current leading nonzero column.
    ///
    /// Current implementation scans touched columns and returns the minimum
    /// nonzero column. This is simple and correct. If profiling shows this is
    /// hot, the next improvement is to maintain touched columns in a structure
    /// that supports faster leading-column lookup.
    pub fn leading_col(&self) -> Option<usize> {
        let zero = C::default();

        self.touched
            .iter()
            .copied()
            .filter(|&col| self.values[col] != zero)
            .min()
    }

    /// Reduces this buffer by a normalized sparse pivot row.
    ///
    /// Assumes `pivot.lead_coeff == 1`.
    pub fn reduce_by_pivot<F>(&mut self, field: &F, pivot: &SparsePivotRow<C>) -> Result<()>
    where
        F: FieldCtx<Elem = C>,
    {
        let zero = C::default();
        let factor = self.values[pivot.lead_col];

        if factor == zero {
            return Ok(());
        }

        // Cancel leading column.
        self.values[pivot.lead_col] = zero;

        // Sparse AXPY:
        // buffer[col] -= factor * pivot[col]
        for &(col, pivot_coeff) in &pivot.tail {
            let old = self.values[col];
            let sub = field.mul(factor, pivot_coeff);
            let new = field.sub(old, sub);

            if new != zero {
                self.mark_touched(col);
            }

            self.values[col] = new;
        }

        Ok(())
    }

    /// Converts the current buffer into a normalized sparse pivot row.
    ///
    /// The returned pivot row has:
    /// - leading coefficient normalized to one,
    /// - tail sorted by increasing column index,
    /// - no zero entries.
    pub fn to_normalized_pivot<F>(&self, field: &F, lead_col: usize) -> Result<SparsePivotRow<C>>
    where
        F: FieldCtx<Elem = C>,
    {
        let zero = C::default();
        let lead_coeff = self.values[lead_col];

        let Some(inv) = field.try_inv(lead_coeff) else {
            return Err(F4Error::NonInvertibleLeadingCoefficient);
        };

        let normalized_lead = field.mul(lead_coeff, inv);

        let mut tail = Vec::with_capacity(self.touched.len().saturating_sub(1));

        for &col in &self.touched {
            if col == lead_col {
                continue;
            }

            let coeff = self.values[col];

            if coeff == zero {
                continue;
            }

            let normalized = field.mul(coeff, inv);

            if normalized != zero {
                tail.push((col, normalized));
            }
        }

        tail.sort_unstable_by_key(|&(col, _)| col);

        Ok(SparsePivotRow { lead_col, lead_coeff: normalized_lead, tail })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sparse_row_tracks_only_nonzero_entries() {
        let row = SparseMatrixRow { entries: vec![(0, 10), (2, 0), (4, 30)] };

        let mut buffer = DenseReductionBuffer::new(8);
        buffer.load_sparse_row(&row);

        assert_eq!(buffer.touched_len(), 2);
        assert_eq!(buffer.value_at(0), 10);
        assert_eq!(buffer.value_at(2), 0);
        assert_eq!(buffer.value_at(4), 30);
        assert_eq!(buffer.leading_col(), Some(0));
    }

    #[test]
    fn clear_only_resets_touched_entries() {
        let row = SparseMatrixRow { entries: vec![(3, 11), (6, 22)] };

        let mut buffer = DenseReductionBuffer::new(10);
        buffer.load_sparse_row(&row);

        assert_eq!(buffer.touched_len(), 2);

        buffer.clear();

        assert_eq!(buffer.touched_len(), 0);
        assert_eq!(buffer.value_at(3), 0);
        assert_eq!(buffer.value_at(6), 0);
        assert_eq!(buffer.leading_col(), None);
    }

    #[test]
    fn leading_col_ignores_zeroed_touched_entries() {
        let row = SparseMatrixRow { entries: vec![(5, 10), (2, 20)] };

        let mut buffer = DenseReductionBuffer::new(8);
        buffer.load_sparse_row(&row);

        assert_eq!(buffer.leading_col(), Some(2));
    }
}
