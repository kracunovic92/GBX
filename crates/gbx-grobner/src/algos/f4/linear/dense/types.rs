//! Dense matrix types for the F4 dense reducer.

/// Dense coefficient matrix used by the F4 linear phase.
///
/// Rows correspond to symbolic products. Columns correspond to monomials sorted
/// by the active monomial order.
#[derive(Debug, Clone)]
pub struct DenseMatrix<C> {
    /// Dense coefficient rows.
    pub rows: Vec<Vec<C>>,

    /// Number of columns in each row.
    pub ncols: usize,
}

impl<C> DenseMatrix<C> {
    /// Creates a dense matrix from owned rows and column count.
    #[inline]
    #[must_use]
    pub fn new(rows: Vec<Vec<C>>, ncols: usize) -> Self {
        Self { rows, ncols }
    }

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
        self.ncols
    }

    /// Returns `true` when the matrix has no rows.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Borrows one row.
    #[inline]
    #[must_use]
    pub fn row(&self, index: usize) -> &[C] {
        &self.rows[index]
    }

    /// Mutably borrows one row.
    #[inline]
    #[must_use]
    pub fn row_mut(&mut self, index: usize) -> &mut [C] {
        &mut self.rows[index]
    }
}

/// Metadata attached to one dense matrix row.
#[derive(Debug, Clone)]
pub struct MatrixRowMeta<M> {
    /// Leading monomial of the original symbolic row before elimination.
    pub leading_mono: Option<M>,

    /// Row index in the original symbolic batch.
    pub source_row: usize,
}

impl<M> MatrixRowMeta<M> {
    /// Creates row metadata.
    #[inline]
    #[must_use]
    pub fn new(leading_mono: Option<M>, source_row: usize) -> Self {
        Self { leading_mono, source_row }
    }
}

/// Full dense F4 matrix.
///
/// This stores coefficient rows, ordered monomial columns, and metadata for the
/// original symbolic rows.
#[derive(Debug, Clone)]
pub struct F4Matrix<M, C> {
    /// Dense coefficient matrix.
    pub matrix: DenseMatrix<C>,

    /// Ordered monomial columns.
    pub columns: Vec<M>,

    /// Per-row metadata.
    pub metadata: Vec<MatrixRowMeta<M>>,
}

impl<M, C> F4Matrix<M, C> {
    /// Creates a full F4 matrix from owned parts.
    #[inline]
    #[must_use]
    pub fn new(matrix: DenseMatrix<C>, columns: Vec<M>, metadata: Vec<MatrixRowMeta<M>>) -> Self {
        Self { matrix, columns, metadata }
    }

    /// Returns the number of rows.
    #[inline]
    #[must_use]
    pub fn nrows(&self) -> usize {
        self.matrix.nrows()
    }

    /// Returns the number of columns.
    #[inline]
    #[must_use]
    pub fn ncols(&self) -> usize {
        self.matrix.ncols()
    }

    /// Borrows one coefficient row.
    #[inline]
    #[must_use]
    pub fn row(&self, index: usize) -> &[C] {
        self.matrix.row(index)
    }

    /// Returns `true` when the matrix has no rows.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_matrix_reports_dimensions() {
        let matrix = DenseMatrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]], 3);

        assert_eq!(matrix.nrows(), 2);
        assert_eq!(matrix.ncols(), 3);
        assert!(!matrix.is_empty());
    }

    #[test]
    fn dense_matrix_row_accessors_borrow_rows() {
        let mut matrix = DenseMatrix::new(vec![vec![1, 2], vec![3, 4]], 2);

        assert_eq!(matrix.row(0), &[1, 2]);

        matrix.row_mut(1)[0] = 9;

        assert_eq!(matrix.row(1), &[9, 4]);
    }

    #[test]
    fn matrix_row_meta_stores_values() {
        let meta = MatrixRowMeta::new(Some("x"), 7);

        assert_eq!(meta.leading_mono, Some("x"));
        assert_eq!(meta.source_row, 7);
    }

    #[test]
    fn f4_matrix_reports_dimensions_and_rows() {
        let matrix = DenseMatrix::new(vec![vec![1, 0], vec![0, 1]], 2);
        let columns = vec!["x", "y"];
        let metadata = vec![MatrixRowMeta::new(Some("x"), 0), MatrixRowMeta::new(Some("y"), 1)];

        let f4 = F4Matrix::new(matrix, columns, metadata);

        assert_eq!(f4.nrows(), 2);
        assert_eq!(f4.ncols(), 2);
        assert_eq!(f4.row(0), &[1, 0]);
        assert!(!f4.is_empty());
    }
}
