//! Monomial representations for polynomials.
//!
//! This module provides two monomial types:
//! - [`Monomial<N>`]: fixed number of variables chosen at compile time.
//! - [`DynamicMonomial`]: runtime-sized monomials, where the number of variables
//!   is stored as the length of an exponent slice.
//!
//! Algorithms that should work with either representation can depend on
//! the [`MonomialLike`] trait.

mod dynamic;
mod fixed;
mod order;
mod traits;

/// Errors that can occur when operating on monomials.
///
/// This error type is shared by both the fixed-size [`Monomial<N>`] and
/// the dynamic [`DynamicMonomial`] representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonomialError {
    /// Overflow during component-wise exponent addition.
    ///
    /// This error occurs in operations like `a.mul(b)` or `a.checked_mul(b)`
    /// when adding exponents component-wise:
    ///
    /// ```text
    /// out[i] = lhs[i] + rhs[i]
    /// ```
    ///
    /// If the sum cannot be represented in `u32`, an overflow is detected.
    ExponentOverflow {
        /// The variable index where `lhs[index] + rhs[index]` overflowed.
        index: usize,

        /// The exponent from the left monomial at this index.
        lhs: u32,

        /// The exponent from the right monomial at this index.
        rhs: u32,
    },

    /// Total degree computation overflowed `u64`.
    ///
    /// Happens when summing all exponents exceeds `u64::MAX`.
    DegreeOverflow,

    /// The two monomials had different numbers of variables, so the requested
    /// operation is undefined.
    ///
    /// This mainly applies to the dynamic representation; for fixed-size
    /// monomials, the type system guarantees matching numbers of variables.
    MismatchedVariableCount {
        /// Number of variables in the left operand.
        lhs: usize,
        /// Number of variables in the right operand.
        rhs: usize,
    },
}

pub use dynamic::DynamicMonomial;
pub use fixed::Monomial;
pub use order::{Grevlex, Lex, MonomialOrder};
pub use traits::MonomialLike;
