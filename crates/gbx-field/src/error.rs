use thiserror::Error;

/// Errors produced by field construction and validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum FieldError {
    /// A prime field modulus must be at least 2.
    #[error("invalid modulus p={p} (must be >= 2)")]
    InvalidModulus {
        /// prime number
        p: u32,
    },

    /// The provided modulus is not prime.
    #[error("modulus is not prime: p={p}")]
    ModulusNotPrime {
        /// prime number
        p: u32,
    },
}

/// Result type used by `gbx-field`.
pub type FieldResult<T> = core::result::Result<T, FieldError>;
