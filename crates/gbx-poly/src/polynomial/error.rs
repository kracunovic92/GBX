//! Polynomial-level error types.

use crate::monomial::MonomialError;
use crate::ring::RingError;
use crate::term::TermError;
use thiserror::Error;

/// Errors that can occur when operating on polynomials.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum PolynomialError {
    /// Ring mismatch / ring construction errors.
    #[error("ring error: {0}")]
    Ring(RingError),

    /// Error originating from term-level operations.
    #[error("term error: {0}")]
    Term(TermError),

    /// Error originating from monomial-level operations.
    #[error("monomial error: {0}")]
    Monomial(MonomialError),

    /// Internal invariant broken (should never happen if invariants are maintained).
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

/// Convenience alias.
pub type Result<T> = core::result::Result<T, PolynomialError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ring::RingError;

    #[test]
    fn display_includes_prefix() {
        let e = PolynomialError::Ring(RingError::InvalidNvars { nvars: 0 });
        let s = e.to_string();
        assert!(s.contains("ring error"));
    }
}
