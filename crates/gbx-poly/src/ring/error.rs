use gbx_field::FieldError;
use thiserror::Error;

/// Errors related to ring construction and ring safety checks.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum RingError {
    /// Invalid number of variables. A polynomial ring must have at least 1 variable.
    #[error("invalid nvars: {nvars} (must be > 0)")]
    InvalidNvars { nvars: usize },

    /// Operation attempted with a polynomial created in a different ring context.
    ///
    /// This typically indicates user code mixed polynomials from different rings
    /// (for example different modulus, order, or arity) in one operation.
    #[error("mismatched ring context: expected ring_id={expected}, got ring_id={got}")]
    MismatchedRing { expected: u64, got: u64 },

    /// Error originating from field construction or field-level validation.
    #[error(transparent)]
    Field(#[from] FieldError),
}

/// Convenience alias for ring results.
pub type Result<T> = core::result::Result<T, RingError>;
