//! Sparse matrix and pivot row types used by the Roman sparse-buffer reducer.
//!
//! Column convention:
//! - smaller column index means earlier/larger monomial in the current order,
//! - therefore the leading column is the minimum nonzero column.

use gbx_poly::monomial::Monomial;

/// Sparse matrix produced from a batch of F4 polynomial rows.
///
/// `columns[col]` gives the monomial represented by column `col`.
#[derive(Debug, Clone)]
pub struct SparseMatrix<C> {
    /// Ordered monomial columns.
    pub columns: Vec<Monomial>,

    /// Sparse matrix rows indexed into `columns`.
    pub rows: Vec<SparseMatrixRow<C>>,

    /// Marks columns that were leading columns of original input rows.
    ///
    /// Pivots with these leading columns are internal reducer rows and should
    /// not be emitted as new F4 rows.
    pub input_lead_cols: Vec<bool>,
}

impl<C> SparseMatrix<C> {
    /// Returns the number of rows.
    #[inline]
    #[must_use]
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of columns.
    #[inline]
    #[must_use]
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    /// Returns the number of stored nonzero entries.
    #[inline]
    #[must_use]
    pub fn nnz(&self) -> usize {
        self.rows.iter().map(SparseMatrixRow::nnz).sum()
    }

    /// Returns the number of cells in the equivalent dense matrix.
    #[inline]
    #[must_use]
    pub fn dense_cells(&self) -> usize {
        self.nrows().saturating_mul(self.ncols())
    }

    /// Returns the stored-entry density relative to the equivalent dense matrix.
    #[inline]
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn density(&self) -> f64 {
        let dense_cells = self.dense_cells();

        if dense_cells == 0 { 0.0 } else { self.nnz() as f64 / dense_cells as f64 }
    }

    /// Consumes the matrix and returns its owned parts.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Vec<Monomial>, Vec<SparseMatrixRow<C>>, Vec<bool>) {
        (self.columns, self.rows, self.input_lead_cols)
    }
}

/// One sparse matrix row.
///
/// Invariant:
/// - `entries` should be sorted by increasing column index,
/// - zero coefficients should not be stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseMatrixRow<C> {
    /// Stored `(column, coefficient)` entries.
    pub entries: Vec<(usize, C)>,
}

impl<C> SparseMatrixRow<C> {
    /// Creates a sparse row from owned entries.
    #[inline]
    #[must_use]
    pub const fn new(entries: Vec<(usize, C)>) -> Self {
        Self { entries }
    }

    /// Returns the number of stored entries.
    #[inline]
    #[must_use]
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` when the row has no stored entries.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the leading column of the row.
    #[inline]
    #[must_use]
    pub fn leading_col(&self) -> Option<usize> {
        self.entries.first().map(|&(col, _)| col)
    }
}

/// Normalized sparse pivot row.
///
/// Invariants:
/// - `lead_coeff` should be one,
/// - `tail` does not contain `lead_col`,
/// - `tail` is sorted by increasing column index,
/// - `tail` contains no zero coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparsePivotRow<C> {
    /// Leading column of the pivot row.
    pub lead_col: usize,

    /// Leading coefficient of the pivot row.
    pub lead_coeff: C,

    /// Non-leading `(column, coefficient)` entries.
    pub tail: Vec<(usize, C)>,
}

impl<C> SparsePivotRow<C> {
    /// Returns the number of stored entries, including the leading entry.
    #[inline]
    #[must_use]
    pub fn nnz(&self) -> usize {
        1 + self.tail.len()
    }

    /// Iterates over all stored entries, including the leading entry first.
    #[inline]
    pub fn entries(&self) -> impl Iterator<Item = (usize, C)> + '_
    where
        C: Copy,
    {
        std::iter::once((self.lead_col, self.lead_coeff)).chain(self.tail.iter().copied())
    }
}
