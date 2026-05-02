use gbx_field::FieldError;
use thiserror::Error;

/// Errors produced while constructing or checking ring contexts.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RingError {
    /// A polynomial ring must have at least one variable.
    #[error("invalid nvars: {nvars} (must be > 0)")]
    InvalidNvars { nvars: usize },

    /// An operation tried to combine values from different ring contexts.
    #[error("mismatched ring context: expected ring_id={expected}, got ring_id={got}")]
    MismatchedRing { expected: u64, got: u64 },

    /// Field construction or validation error.
    #[error(transparent)]
    Field(#[from] FieldError),
}

/// Result type used by the ring module.
pub type RingResult<T> = Result<T, RingError>;
