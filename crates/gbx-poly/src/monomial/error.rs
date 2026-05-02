use thiserror::Error;

/// Errors produced by monomial construction and arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MonomialError {
    /// Overflow during component-wise exponent addition.
    #[error("exponent overflow at index {index}: {lhs} + {rhs}")]
    ExponentOverflow { index: usize, lhs: u32, rhs: u32 },

    /// Total degree computation overflowed `u32`.
    #[error("total degree overflow")]
    DegreeOverflow,

    /// The two monomials had different arities.
    #[error("mismatched arity: {lhs} vs {rhs}")]
    MismatchedArity { lhs: usize, rhs: usize },

    /// Constructor received an unexpected exponent count.
    #[error("wrong exponent length: expected {expected}, got {got}")]
    WrongLength { expected: usize, got: usize },

    /// Exact division was requested but the divisor does not divide the dividend.
    #[error("not divisible")]
    NotDivisible,
}

/// Result type used by the monomial module.
pub type MonomialResult<T> = Result<T, MonomialError>;
