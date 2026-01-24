//! Polynomial primitives: terms (coefficient × monomial).
//!
//! A term is `c * x^α` where `c` is a coefficient and `α` is a monomial.
//! Terms are used as building blocks for sparse polynomials.

mod dynamic;
mod fixed;
mod tests;
pub(crate) mod traits;

pub use dynamic::DynamicTerm;
pub use fixed::FixedTerm;
pub use traits::{Term, TermOps, TermView};

use crate::monomial::MonomialError;

/// Errors that can occur when operating on terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TermError {
    /// Monomial arithmetic failed (overflow / mismatched variable counts, etc.).
    Monomial(MonomialError),
}

impl From<MonomialError> for TermError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        TermError::Monomial(e)
    }
}
