//! Error types for Gröbner basis construction.

use crate::{PairUpdateError, PostError, SPolyError};
use gbx_alg::DivByZero;
use gbx_poly::polynomial::{PolynomialError, ReduceError};
use thiserror::Error;

/// Errors that can occur during Buchberger Gröbner basis computation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BuchbergerError {
    /// Error originating from polynomial/term/monomial operations.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// Error while constructing an S-polynomial.
    #[error(transparent)]
    SPoly(#[from] SPolyError),

    /// Error while reducing a polynomial w.r.t. the current basis.
    #[error(transparent)]
    Reduce(#[from] ReduceError),

    /// Internal invariant broken (e.g. pair indexes out of range).
    #[error("internal invariant violation")]
    InvariantViolation,

    /// Error while trying to division with 0
    #[error("{0}")]
    DivByZero(DivByZero),
    #[error(transparent)]
    Post(#[from] PostError),
    #[error("pair update failed: {0}")]
    PairUpdate(#[from] PairUpdateError),
}
