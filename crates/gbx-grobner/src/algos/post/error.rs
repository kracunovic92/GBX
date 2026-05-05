use gbx_alg::DivByZero;
use gbx_poly::polynomial::{PolynomialError, ReduceError};
use thiserror::Error;

/// Errors that can occur during Gröbner basis postprocessing.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PostError {
    /// Error originating from polynomial/term/monomial operations.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// Error while reducing one basis element by the others.
    #[error(transparent)]
    Reduce(#[from] ReduceError),

    /// Error while trying to divide by zero.
    #[error("{0}")]
    DivByZero(DivByZero),

    /// Internal invariant broken during minimization/reduction.
    #[error("Gröbner basis postprocessing invariant violation")]
    InvariantViolation,
    /// Leading monomial changed
    #[error("Gröbner basis postprocessing invariant violation")]
    LeadingMonomialChangedDuringReduction,
    /// Something went wrong
    #[error("Zero polynomials in-reduce")]
    ZeroPolynomialInReduction,
}
