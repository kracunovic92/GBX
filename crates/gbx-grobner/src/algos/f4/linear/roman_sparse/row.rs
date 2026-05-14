//! Sparse matrix and pivot row types used by the Roman sparse-buffer reducer.
//!
//! Column convention:
//! - smaller column index means earlier/larger monomial in the current order,
//! - therefore the leading column is the minimum nonzero column.
//!
//! `SparseMatrixRow` is used for input matrix rows.
//! `SparsePivotRow` is used for normalized pivot rows created during reduction.

use gbx_poly::monomial::Monomial;

/// Sparse matrix produced from a batch of F4 polynomial rows.
///
/// `columns[col]` gives the monomial represented by column `col`.
///
/// `input_lead_cols[col] == true` means column `col` was the leading column
/// of at least one original input row. Pivots with those leading columns are
/// internal reducer rows and should not be emitted as new F4 rows.
#[derive(Debug, Clone)]
pub struct SparseMatrix<C> {
    pub columns: Vec<Monomial>,
    pub rows: Vec<SparseMatrixRow<C>>,
    pub input_lead_cols: Vec<bool>,
}

impl<C> SparseMatrix<C> {
    #[inline]
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    #[inline]
    pub fn nnz(&self) -> usize {
        self.rows.iter().map(SparseMatrixRow::nnz).sum()
    }

    #[inline]
    pub fn dense_cells(&self) -> usize {
        self.nrows().saturating_mul(self.ncols())
    }

    #[inline]
    pub fn density(&self) -> f64 {
        let dense_cells = self.dense_cells();

        if dense_cells == 0 { 0.0 } else { self.nnz() as f64 / dense_cells as f64 }
    }

    #[inline]
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
    pub entries: Vec<(usize, C)>,
}

impl<C> SparseMatrixRow<C> {
    #[inline]
    pub fn new(entries: Vec<(usize, C)>) -> Self {
        Self { entries }
    }

    #[inline]
    pub fn nnz(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[inline]
    pub fn leading_col(&self) -> Option<usize> {
        self.entries.first().map(|&(col, _)| col)
    }
}

/// A normalized sparse pivot row.
///
/// Invariants:
/// - `lead_coeff` should be one,
/// - `tail` does not contain `lead_col`,
/// - `tail` is sorted by increasing column index,
/// - `tail` contains no zero coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparsePivotRow<C> {
    pub lead_col: usize,
    pub lead_coeff: C,
    pub tail: Vec<(usize, C)>,
}

impl<C> SparsePivotRow<C> {
    #[inline]
    pub fn nnz(&self) -> usize {
        1 + self.tail.len()
    }

    #[inline]
    pub fn entries(&self) -> impl Iterator<Item = (usize, C)> + '_
    where
        C: Copy,
    {
        std::iter::once((self.lead_col, self.lead_coeff)).chain(self.tail.iter().copied())
    }
}
