//! Polynomial-level error types.

use crate::monomial::MonomialError;
use crate::ring::RingError;
use crate::term::TermError;
use thiserror::Error;

/// Errors produced by polynomial construction and arithmetic.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum PolynomialError {
    /// Ring mismatch or ring validation error.
    #[error("ring error: {0}")]
    Ring(RingError),

    /// Term-level error.
    #[error("term error: {0}")]
    Term(TermError),

    /// Monomial-level error.
    #[error("monomial error: {0}")]
    Monomial(MonomialError),

    /// Internal invariant was violated.
    #[error("polynomial invariant violation")]
    InvariantViolation,
}

impl From<RingError> for PolynomialError {
    #[inline]
    fn from(e: RingError) -> Self {
        Self::Ring(e)
    }
}

impl From<TermError> for PolynomialError {
    #[inline]
    fn from(e: TermError) -> Self {
        Self::Term(e)
    }
}

impl From<MonomialError> for PolynomialError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        Self::Monomial(e)
    }
}

/// Result type used by the polynomial module.
pub type PolynomialResult<T> = Result<T, PolynomialError>;
