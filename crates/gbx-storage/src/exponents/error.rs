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

    /// A signed exponent input was negative.
    NegativeValue {
        /// The offending value.
        value: i64,
    },
}

impl ExpsError {
    /// Convenience constructor for arity mismatch.
    #[inline]
    #[must_use]
    pub const fn wrong_length(expected: usize, got: usize) -> Self {
        Self::WrongLength { expected, got }
    }

    /// Convenience constructor for value overflow.
    #[inline]
    #[must_use]
    pub const fn value_overflow(value: u64) -> Self {
        Self::ValueOverflow { value }
    }

    /// Convenience constructor for negative signed exponent input.
    #[inline]
    #[must_use]
    pub const fn negative_value(value: i64) -> Self {
        Self::NegativeValue { value }
    }
}

impl fmt::Display for ExpsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongLength { expected, got } => {
                write!(f, "wrong exponent length: expected {expected}, got {got}")
            }
            Self::ValueOverflow { value } => {
                write!(
                    f,
                    "exponent value overflow: {value} does not fit in the chosen word size"
                )
            }
            Self::NegativeValue { value } => {
                write!(f, "negative exponent value: {value}")
            }
        }
    }
}
impl std::error::Error for ExpsError {}
