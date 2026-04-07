use crate::algos::f4::types::PolyMono;

/// Source polynomial used by a symbolic product.
///
/// In the basic version this will usually be a basis polynomial.
/// The history variant is here so `Simplify` can later replace a basis product
/// with a previously reduced row, matching the improved F4 paper.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolicSource {
    /// Current Gröbner basis polynomial `basis[index]`.
    Basis(usize),

    /// Previously reduced row from batch history.
    HistoryReducedRow { batch_index: usize, row_index: usize },
}

/// A non-evaluated product `m * f`, represented symbolically.
///
/// This is the central object of symbolic preprocessing:
/// we first reason about products symbolically, and only materialize them
/// into actual polynomials when needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolicProduct<M> {
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

    /// Row was added because some monomial in the current symbolic family
    /// was top-reducible by the basis.
    TopReducerClosure,

    /// Row was inserted after a non-trivial simplify step.
    ///
    /// In the first pass this may be unused, but it is useful to keep now
    /// because the improved F4 algorithm relies on it.
    Simplified,
}

/// A materialized symbolic product together with its origin metadata.
#[derive(Debug, Clone)]
pub struct SymbolicRow<P, M> {
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
///
/// `rows` are the rows that should be sent to the linear/matrix reduction phase.
/// `symbolic_heads` are the leading monomials of the symbolic family before
/// row-echelon reduction; later extraction keeps only reduced rows whose heads
/// are new relative to this set.
#[derive(Debug, Clone)]
pub struct SymbolicPreprocessOutput<P, M> {
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

/// Convenient alias for symbolic products built from the monomial type of `P`.
pub type PolyProduct<P> = SymbolicProduct<PolyMono<P>>;
