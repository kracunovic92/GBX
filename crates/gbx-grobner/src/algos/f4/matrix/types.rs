//! Matrix data types for F4.

use crate::algos::f4::symbolic::SymbolicRowKind;

/// Ordered column basis used by an F4 coefficient matrix.
#[derive(Debug, Clone)]
pub struct ColumnBasis<M> {
    /// Matrix columns in descending monomial order
    /// (earlier columns correspond to larger monomials).
    pub monomials: Vec<M>,
}

impl<M> ColumnBasis<M> {
    #[must_use]
    pub fn len(&self) -> usize {
        self.monomials.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.monomials.is_empty()
    }
}

/// Metadata attached to one matrix row.
///
/// This preserves symbolic provenance through matrix construction.
#[derive(Debug, Clone)]
pub struct MatrixRowMeta<M> {
    /// Index of the source basis polynomial used to materialize this row.
    pub source_basis_index: usize,
    /// Monomial multiplier used to materialize the row.
    pub multiplier: M,
    /// Role of the symbolic row.
    pub kind: SymbolicRowKind,
}

/// Dense matrix representation for the current F4 implementation.
#[derive(Debug, Clone)]
pub struct DenseMatrixData<C, M> {
    /// Dense coefficient rows.
    pub rows: Vec<Vec<C>>,
    /// Ordered matrix columns.
    pub columns: ColumnBasis<M>,
    /// Per-row metadata aligned with `rows`.
    pub row_meta: Vec<MatrixRowMeta<M>>,
}

impl<C, M> DenseMatrixData<C, M> {
    #[must_use]
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() || self.columns.is_empty()
    }
}

/// Reduced dense matrix representation after row reduction.
#[derive(Debug, Clone)]
pub struct ReducedMatrixData<C, M> {
    /// Reduced dense coefficient rows.
    pub rows: Vec<Vec<C>>,
    /// Ordered matrix columns.
    pub columns: ColumnBasis<M>,
    /// Per-row metadata carried through row permutations.
    pub row_meta: Vec<MatrixRowMeta<M>>,
}

impl<C, M> ReducedMatrixData<C, M> {
    #[must_use]
    pub fn nrows(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() || self.columns.is_empty()
    }
}
