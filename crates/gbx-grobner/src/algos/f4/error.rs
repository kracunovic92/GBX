//! Error types for the F4 implementation.

use crate::PostError;

use gbx_alg::DivByZero;
use gbx_poly::monomial::MonomialError;
use gbx_poly::polynomial::{PolynomialError, ReduceError};

use thiserror::Error;

/// Result type used by the F4 implementation.
pub type Result<T> = core::result::Result<T, F4Error>;

/// Errors that can occur while running the F4 algorithm.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum F4Error {
    /// Polynomial construction, normalization, or mutation failed.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// Monomial arithmetic or arity validation failed.
    #[error(transparent)]
    Mono(#[from] MonomialError),

    /// Polynomial reduction failed.
    #[error(transparent)]
    Reduce(#[from] ReduceError),

    /// Division by zero occurred in field or coefficient arithmetic.
    #[error(transparent)]
    DivByZero(#[from] DivByZero),

    /// F4 was called without input generators.
    #[error("F4 received no input generators")]
    EmptyInput,

    /// The configured F4 batch size is invalid.
    #[error("invalid F4 batch size: must be greater than zero")]
    InvalidBatchSize,

    /// A symbolic product references a missing basis polynomial.
    #[error("missing basis polynomial at index {index}")]
    MissingBasisPolynomial {
        /// Missing basis index.
        index: usize,
    },

    /// Critical-pair construction or validation failed.
    #[error("invalid F4 critical pair")]
    InvalidCriticalPair,

    /// Dense or sparse matrix construction violated an internal invariant.
    #[error("F4 matrix build invariant violation")]
    MatrixBuildInvariant,

    /// Row reduction violated an internal invariant.
    #[error("F4 row reduction invariant violation")]
    RowReductionInvariant,

    /// A pivot leading coefficient could not be inverted.
    #[error("non-invertible leading coefficient")]
    NonInvertibleLeadingCoefficient,

    /// Reduced-row extraction violated an internal invariant.
    #[error("F4 extraction invariant violation")]
    ExtractionInvariant,

    /// Symbolic preprocessing violated an internal invariant.
    #[error("F4 symbolic preprocessing invariant violation")]
    SymbolicInvariant,

    /// A symbolic product references a missing reduced row from batch history.
    #[error("missing history reduced row: batch {batch_index}, row {row_index}")]
    MissingHistoryRow {
        /// Missing history batch index.
        batch_index: usize,

        /// Missing reduced-row index inside the history batch.
        row_index: usize,
    },

    /// Final basis post-processing failed.
    #[error(transparent)]
    Post(#[from] PostError),
}
