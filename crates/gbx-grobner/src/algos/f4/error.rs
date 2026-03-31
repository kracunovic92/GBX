//! Error types for F4 Gröbner basis computation.

use crate::{PairUpdateError, PostError, SPolyError};
use gbx_alg::DivByZero;
use gbx_poly::polynomial::{PolynomialError, ReduceError};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, F4Error>;

/// Errors that can occur during F4 Gröbner basis computation.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum F4Error {
    /// Error originating from polynomial/term/monomial operations.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// Error while constructing an S-polynomial.
    #[error(transparent)]
    SPoly(#[from] SPolyError),

    /// Error while reducing a polynomial w.r.t. the current basis.
    ///
    /// This is mainly useful while the initial F4 implementation still uses
    /// classical safety-reduction for extracted rows.
    #[error(transparent)]
    Reduce(#[from] ReduceError),

    /// Error while trying to divide by zero.
    #[error("{0}")]
    DivByZero(DivByZero),

    /// F4 received no generators.
    #[error("F4 received no input generators")]
    EmptyInput,

    /// F4 requires a strictly positive batch size.
    #[error("invalid F4 batch size: must be greater than zero")]
    InvalidBatchSize,

    /// Internal invariant broken: selected pair refers to a missing basis element.
    #[error("missing basis polynomial at index {index}")]
    MissingBasisPolynomial { index: usize },

    /// Internal matrix construction invariant broken.
    #[error("F4 matrix build invariant violation")]
    MatrixBuildInvariant,

    /// Internal row-reduction invariant broken.
    #[error("F4 row reduction invariant violation")]
    RowReductionInvariant,

    /// Internal extraction invariant broken.
    #[error("F4 extraction invariant violation")]
    ExtractionInvariant,

    #[error(transparent)]
    PairUpdate(#[from] PairUpdateError),
    #[error("F4 symbolic preprocessing invariant violation")]
    SymbolicInvariant,
    #[error(transparent)]
    Post(#[from] PostError),
}
