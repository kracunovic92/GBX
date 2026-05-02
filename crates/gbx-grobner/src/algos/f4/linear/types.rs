/// Dense coefficient matrix used by the F4 linear phase.
///
/// Rows correspond to symbolic products.
/// Columns correspond to monomials, sorted by monomial order.
#[derive(Debug, Clone)]
pub struct DenseMatrix<C> {
    pub rows: Vec<Vec<C>>,
    pub ncols: usize,
}

impl<C> DenseMatrix<C> {
    #[must_use]
    pub fn new(rows: Vec<Vec<C>>, ncols: usize) -> Self {
        Self { rows, ncols }
    }

    #[must_use]
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// Optional per-row metadata for tracing/debugging and future extraction logic.
#[derive(Debug, Clone)]
pub struct MatrixRowMeta<M> {
    /// Leading monomial of the original symbolic row before elimination, if known.
    pub leading_mono: Option<M>,

    /// Row index in the original symbolic batch.
    pub source_row: usize,
}

impl<M> MatrixRowMeta<M> {
    #[must_use]
    pub fn new(leading_mono: Option<M>, source_row: usize) -> Self {
        Self { leading_mono, source_row }
    }
}

/// Full F4 matrix object:
/// - dense coefficient rows,
/// - ordered monomial columns,
/// - per-row metadata.
#[derive(Debug, Clone)]
pub struct F4Matrix<M, C> {
    pub matrix: DenseMatrix<C>,
    pub columns: Vec<M>,
    pub metadata: Vec<MatrixRowMeta<M>>,
}

impl<M, C> F4Matrix<M, C> {
    #[must_use]
    pub fn new(matrix: DenseMatrix<C>, columns: Vec<M>, metadata: Vec<MatrixRowMeta<M>>) -> Self {
        Self { matrix, columns, metadata }
    }

    #[must_use]
    pub fn nrows(&self) -> usize {
        self.matrix.nrows()
    }

    #[must_use]
    pub fn ncols(&self) -> usize {
        self.matrix.ncols
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }
}
