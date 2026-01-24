//! Polynomial-level errors.
//!
//! This error type is intentionally small today, but is marked `#[non_exhaustive]`
//! so it can grow without breaking downstream code.

use crate::monomial::MonomialError;
use crate::term::TermError;

/// Errors that can occur when operating on polynomials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PolynomialError {
    /// Error originating from term-level operations.
    Term(TermError),

    /// Internal invariant broken (e.g. a non-zero polynomial has no leading term).
    InvariantViolation,
}

impl From<TermError> for PolynomialError {
    #[inline]
    fn from(err: TermError) -> Self {
        Self::Term(err)
    }
}

impl From<MonomialError> for PolynomialError {
    #[inline]
    fn from(err: MonomialError) -> Self {
        Self::Term(TermError::from(err))
    }
}
