use std::fmt::Debug;

/// Sparse input matrix row.
///
/// Invariant:
/// - `entries` are sorted by increasing column index.
/// - column 0 is the largest monomial / leading column.
#[derive(Debug, Clone)]
pub struct SparseMatrixRow<C> {
    pub entries: Vec<(usize, C)>,
}

impl<C> SparseMatrixRow<C>
where
    C: Copy + Eq + Default,
{
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    pub fn lead_col(&self) -> Option<usize> {
        self.entries.first().map(|(col, _)| *col)
    }
}

/// Normalized sparse pivot row.
///
/// We store the leading coefficient explicitly, even though it should be one.
/// This avoids requiring `field.one()` from `FieldCtx`.
#[derive(Debug, Clone)]
pub struct SparsePivotRow<C> {
    pub lead_col: usize,
    pub lead_coeff: C,

    /// Tail entries only. Does not include `lead_col`.
    /// Sorted by increasing column index.
    pub tail: Vec<(usize, C)>,
}

impl<C> SparsePivotRow<C>
where
    C: Copy + Eq + Default,
{
    #[inline]
    pub fn nnz(&self) -> usize {
        1 + self.tail.len()
    }
}
