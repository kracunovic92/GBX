//! Errors produced by exponent storage helpers.
//!
//! Most storage containers are infallible. Errors typically come from conversions
//! (e.g., downcasting `u64` → `u32`).

/// Errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpsError {
    /// Input length did not match an expected arity.
    WrongLength {
        /// Expected size in runtime
        expected: usize,
        /// Actual size in runtime
        got: usize,
    },

    /// A value did not fit into the chosen exponent word size.
    ValueOverflow {
        /// What value ?
        value: u64,
    },
}
