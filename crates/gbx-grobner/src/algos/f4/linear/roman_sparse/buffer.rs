use crate::algos::f4::error::{F4Error, Result};
use crate::linear::roman_sparse::row::{SparseMatrixRow, SparsePivotRow};
use gbx_poly::ring::FieldCtx;

/// Dense temporary accumulator used to reduce one sparse matrix row.
///
/// Important:
/// - `values` has length ncols.
/// - `touched` stores only columns that have been written.
/// - `marked[col]` prevents duplicate entries in `touched`.
///
/// Clearing is O(number of touched columns), not O(number of columns).
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
    pub fn new(ncols: usize) -> Self {
        Self { values: vec![C::default(); ncols], touched: Vec::new(), marked: vec![false; ncols] }
    }

    #[inline]
    pub fn touched_len(&self) -> usize {
        self.touched.len()
    }

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

    pub fn load_sparse_row(&mut self, row: &SparseMatrixRow<C>) {
        debug_assert!(
            self.touched.is_empty(),
            "DenseReductionBuffer must be clear before loading a row"
        );

        let zero = C::default();

        for &(col, coeff) in &row.entries {
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

        if pivot.lead_coeff != zero {
            self.write_value(pivot.lead_col, pivot.lead_coeff);
        }

        for &(col, coeff) in &pivot.tail {
            if coeff != zero {
                self.write_value(col, coeff);
            }
        }
    }

    /// Finds the current leading nonzero column.
    ///
    /// Simple first version: scan touched columns and take min column index.
    /// Later we can optimize this with sorted touched columns, a heap, or bitset.
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
    /// Assumes pivot lead coefficient is 1.
    /// Since we store normalized pivots, factor = buffer[pivot.lead_col].
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

        // Sparse AXPY over pivot tail:
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

    /// Converts current buffer into a normalized pivot row.
    ///
    /// The returned pivot row has:
    /// - lead coefficient normalized to 1,
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

        let mut tail = Vec::new();

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
