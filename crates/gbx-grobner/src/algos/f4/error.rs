use crate::{PostError, SPolyError};
use gbx_alg::DivByZero;
use gbx_poly::monomial::MonomialError;
use gbx_poly::polynomial::{PolynomialError, ReduceError};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, F4Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum F4Error {
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    #[error(transparent)]
    SPoly(#[from] SPolyError),

    #[error(transparent)]
    Mono(#[from] MonomialError),

    #[error(transparent)]
    Reduce(#[from] ReduceError),

    #[error("{0}")]
    DivByZero(DivByZero),

    #[error("F4 received no input generators")]
    EmptyInput,

    #[error("invalid F4 batch size: must be greater than zero")]
    InvalidBatchSize,

    #[error("missing basis polynomial at index {index}")]
    MissingBasisPolynomial { index: usize },

    #[error("invalid critical pair")]
    InvalidCriticalPair,

    #[error("F4 matrix build invariant violation")]
    MatrixBuildInvariant,

    #[error("F4 row reduction invariant violation")]
    RowReductionInvariant,

    #[error("non-invertible leading coefficient")]
    NonInvertibleLeadingCoefficient,

    #[error("F4 extraction invariant violation")]
    ExtractionInvariant,

    #[error("F4 symbolic preprocessing invariant violation")]
    SymbolicInvariant,
    /// Referenced reduced row in batch history does not exist.
    #[error("missing history reduced row: batch {batch_index}, row {row_index}")]
    MissingHistoryRow { batch_index: usize, row_index: usize },

    #[error(transparent)]
    Post(#[from] PostError),
}
