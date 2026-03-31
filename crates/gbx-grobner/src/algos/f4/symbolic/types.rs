//! Core symbolic-row data types for F4.

use gbx_poly::polynomial::PolynomialView;

/// Kind of symbolic row.
///
/// Pair rows come directly from the selected critical-pair batch.
/// Reducer rows are introduced by symbolic preprocessing closure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolicRowKind {
    /// Row corresponding to the left multiple of a selected pair.
    PairLeft,
    /// Row corresponding to the right multiple of a selected pair.
    PairRight,
    /// Row introduced from the current basis during symbolic preprocessing.
    Reducer,
}

/// Seed row for symbolic preprocessing.
///
/// A seed row identifies a basis polynomial together with the monomial by which
/// it must be multiplied before entering the symbolic system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRow<M> {
    /// Index of the source polynomial in the current Gröbner basis.
    pub source_basis_index: usize,
    /// Monomial multiplier applied to that basis polynomial.
    pub multiplier: M,
    /// Role of this row in the symbolic system.
    pub kind: SymbolicRowKind,
}

/// Materialized symbolic row.
///
/// This is the actual polynomial row inserted into the symbolic system, together
/// with metadata describing how it was obtained.
#[derive(Debug, Clone)]
pub struct SymbolicRow<P, M>
where
    P: PolynomialView,
{
    /// Materialized polynomial row.
    pub poly: P,
    /// Index of the source polynomial in the current Gröbner basis.
    pub source_basis_index: usize,
    /// Monomial multiplier applied to the source basis polynomial.
    pub multiplier: M,
    /// Role of this row in the symbolic system.
    pub kind: SymbolicRowKind,
}

/// Result of symbolic preprocessing.
///
/// This separates the initially selected rows from the reducer rows added by
/// closure, while also exposing the full row set for downstream matrix
/// construction.
#[derive(Debug, Clone)]
pub struct SymbolicPreprocessing<P, M>
where
    P: PolynomialView,
{
    /// Rows induced directly by the selected critical-pair batch.
    pub seed_rows: Vec<SymbolicRow<P, M>>,
    /// Rows added from the current basis during symbolic closure.
    pub reducer_rows: Vec<SymbolicRow<P, M>>,
    /// Complete row set used for matrix construction.
    pub all_rows: Vec<SymbolicRow<P, M>>,
}
