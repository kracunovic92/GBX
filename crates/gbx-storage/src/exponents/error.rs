//! Errors produced by exponent storage helpers.
//!
//! Most exponent storage containers are infallible. Errors typically arise from:
//! - validating arity (length) when converting from external inputs
//! - converting between exponent word sizes (e.g. `u64` → `u32`)

use core::fmt;

/// Errors that can occur when constructing or converting exponent vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpsError {
    /// Input length did not match an expected arity.
    WrongLength {
        /// Expected number of exponents.
        expected: usize,
        /// Actual number of exponents provided.
        got: usize,
    },

    /// A value did not fit into the chosen exponent word size.
    ///
    /// Example: converting from `u64` to `u32` where the input exceeds `u32::MAX`.
    ValueOverflow {
        /// The offending value.
        value: u64,
    },
}

impl ExpsError {
    /// Convenience constructor for arity mismatch.
    #[inline]
    pub const fn wrong_length(expected: usize, got: usize) -> Self {
        Self::WrongLength { expected, got }
    }

    /// Convenience constructor for value overflow.
    #[inline]
    pub const fn value_overflow(value: u64) -> Self {
        Self::ValueOverflow { value }
    }
}

impl fmt::Display for ExpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpsError::WrongLength { expected, got } => {
                write!(f, "wrong exponent length: expected {expected}, got {got}")
            }
            ExpsError::ValueOverflow { value } => {
                write!(
                    f,
                    "exponent value overflow: {value} does not fit in the chosen word size"
                )
            }
        }
    }
}
impl std::error::Error for ExpsError {}
