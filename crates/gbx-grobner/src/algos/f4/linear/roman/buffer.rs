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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Dense temporary accumulator used to reduce one sparse matrix row.
///
/// This is still one dense row, not a dense matrix.
///
/// Important invariant:
/// - `touched` is for clearing only.
/// - `active_heap` is for leading-column lookup.
#[derive(Debug, Clone)]
pub struct DenseReductionBuffer<C> {
    values: Vec<C>,

    /// Columns that were ever written since the last clear.
    /// Used only to clear the row cheaply.
    touched: Vec<usize>,

    /// Whether a column is already in `touched`.
    touched_mark: Vec<bool>,

    /// Candidate nonzero columns.
    ///
    /// This is lazy: zero columns may remain in the heap until `leading_col()`
    /// removes them.
    active_heap: BinaryHeap<Reverse<usize>>,

    /// Whether a column is currently queued in `active_heap`.
    queued: Vec<bool>,
}

impl<C> DenseReductionBuffer<C>
where
    C: Copy + Eq + Default,
{
    #[must_use]
    pub fn new(ncols: usize) -> Self {
        Self { values: vec![C::default(); ncols], touched: Vec::new(), touched_mark: vec![false; ncols], active_heap: BinaryHeap::new(), queued: vec![false; ncols] }
    }

    #[inline]
    #[must_use]
    pub fn touched_len(&self) -> usize {
        self.touched.len()
    }

    #[inline]
    #[must_use]
    pub fn ncols(&self) -> usize {
        self.values.len()
    }

    #[inline]
    #[must_use]
    pub fn value_at(&self, col: usize) -> C {
        self.values[col]
    }

    #[inline]
    pub fn clear(&mut self) {
        let zero = C::default();

        for &col in &self.touched {
            self.values[col] = zero;
            self.touched_mark[col] = false;
            self.queued[col] = false;
        }

        self.touched.clear();
        self.active_heap.clear();
    }

    #[inline]
    fn mark_touched(&mut self, col: usize) {
        if !self.touched_mark[col] {
            self.touched_mark[col] = true;
            self.touched.push(col);
        }
    }

    #[inline]
    fn queue_active(&mut self, col: usize) {
        if !self.queued[col] {
            self.queued[col] = true;
            self.active_heap.push(Reverse(col));
        }
    }

    #[inline]
    fn write_value(&mut self, col: usize, value: C) {
        if value != C::default() {
            self.mark_touched(col);
            self.queue_active(col);
        }

        self.values[col] = value;
    }

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
    /// This mutates the heap by lazily removing stale zero columns.
    #[must_use]
    pub fn leading_col(&mut self) -> Option<usize> {
        let zero = C::default();

        while let Some(&Reverse(col)) = self.active_heap.peek() {
            if self.values[col] != zero {
                return Some(col);
            }

            self.active_heap.pop();
            self.queued[col] = false;
        }

        None
    }

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
        //
        // We do not remove it from the heap here.
        // `leading_col()` will lazily discard it.
        self.values[pivot.lead_col] = zero;

        for &(col, pivot_coeff) in &pivot.tail {
            let old = self.values[col];
            let sub = field.mul(factor, pivot_coeff);
            let new = field.sub(old, sub);

            if new != zero {
                self.mark_touched(col);
                self.queue_active(col);
            }

            self.values[col] = new;
        }

        Ok(())
    }

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
    #![allow(clippy::unwrap_used, clippy::expect_used)]
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
