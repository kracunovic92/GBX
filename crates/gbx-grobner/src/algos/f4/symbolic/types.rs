use gbx_poly::monomial::Monomial;

/// Source polynomial used by a symbolic product.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SymbolicSource {
    /// Current Gröbner basis polynomial `basis[index]`.
    Basis(usize),

    /// Previously reduced row from batch history.
    HistoryReducedRow { batch_index: usize, row_index: usize },
}

/// A non-evaluated product `m * f`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicProduct<M = Monomial> {
    pub source: SymbolicSource,
    pub multiplier: M,
}

impl<M> SymbolicProduct<M> {
    #[inline]
    #[must_use]
    pub fn new(source: SymbolicSource, multiplier: M) -> Self {
        Self { source, multiplier }
    }

    #[inline]
    #[must_use]
    pub fn from_basis(basis_index: usize, multiplier: M) -> Self {
        Self { source: SymbolicSource::Basis(basis_index), multiplier }
    }
}

/// Why a symbolic row was inserted into the current symbolic family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolicRowKind {
    /// Row came directly from the selected pair batch.
    InitialSeed,

    /// Row was added because some monomial in the current symbolic family was top-reducible.
    TopReducerClosure,

    /// Row was inserted after a non-trivial simplify step.
    Simplified,
}

/// A materialized symbolic product together with origin metadata.
#[derive(Debug, Clone)]
pub struct SymbolicRow<P, M = Monomial> {
    pub product: SymbolicProduct<M>,
    pub polynomial: P,
    pub kind: SymbolicRowKind,
}

impl<P, M> SymbolicRow<P, M> {
    #[inline]
    #[must_use]
    pub fn new(product: SymbolicProduct<M>, polynomial: P, kind: SymbolicRowKind) -> Self {
        Self { product, polynomial, kind }
    }
}

/// Result of symbolic preprocessing for one F4 batch.
#[derive(Debug, Clone)]
pub struct SymbolicPreprocessOutput<P, M = Monomial> {
    pub rows: Vec<SymbolicRow<P, M>>,
    pub symbolic_heads: Vec<M>,
}

impl<P, M> SymbolicPreprocessOutput<P, M> {
    #[inline]
    #[must_use]
    pub fn new(rows: Vec<SymbolicRow<P, M>>, symbolic_heads: Vec<M>) -> Self {
        Self { rows, symbolic_heads }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// Convenient concrete symbolic product type.
pub type PolyProduct = SymbolicProduct<Monomial>;
