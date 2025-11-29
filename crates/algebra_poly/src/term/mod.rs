//! Polynomial primitives: terms and polynomials over abstract fields.
//!
//! This module exposes:
//! - [`Term<F, N>`]: term with a compile-time number of variables.
//! - [`DynTerm<F>`]: term with a runtime number of variables.
//! - [`TermLike`]: common interface used by polynomial algorithms.
//!
//! The monomial part is provided by `crate::monomial`.

mod dynamic;
mod term;
// (you likely also have `mod polynomial;` here separately for the full Polynomial type)

use crate::monomial::MonomialError;
use crate::monomial::MonomialLike;
use algebra_core::{Field, Zero};
use core::fmt::Debug;

/// Errors that can occur when operating on terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermError {
    /// Error arising from monomial operations (e.g. exponent overflow,
    /// mismatched variable counts for dynamic monomials).
    Monomial(MonomialError),
}

/// Common interface for polynomial terms (coefficient × monomial).
///
/// Implemented by:
/// - [`Term<F, N>`] for fixed-size monomials.
/// - [`DynTerm<F>`] for dynamic monomials.
///
/// This trait is the natural bound for polynomial algorithms that should
/// work with either representation.
pub trait TermLike: Clone + PartialEq {
    /// The underlying field of coefficients.
    type Field: Field;

    /// The monomial representation used by this term.
    ///
    /// This is typically:
    /// - [`crate::monomial::Monomial<N>`] for `Term<F, N>`.
    /// - [`crate::monomial::DynamicMonomial`] for `DynTerm<F>`.
    type Mono: MonomialLike<Error = MonomialError> + Clone;

    /// Error type for fallible term operations.
    ///
    /// For the built-in implementations this is [`TermError`].
    type Error;

    /// Returns a reference to the coefficient.
    fn coeff(&self) -> &Self::Field;

    /// Returns a reference to the monomial.
    fn mono(&self) -> &Self::Mono;

    /// Degree of the term = degree of its monomial.
    #[inline]
    fn degree(&self) -> u64 {
        self.mono()
            .degree()
    }

    /// Returns `true` if the coefficient is zero.
    #[inline]
    fn is_zero(&self) -> bool
    where
        Self::Field: Zero,
    {
        self.coeff()
            .is_zero()
    }

    /// Multiplies this term by a scalar `c` in the field.
    ///
    /// Mathematically: `(a * x^α) * c = (a * c) * x^α`.
    fn mul_scalar(&self, c: &Self::Field) -> Self
    where
        Self::Field: Clone;

    /// Checked multiplication by a monomial.
    ///
    /// Mathematically: `(a * x^α) * x^β = a * x^{α+β}`.
    ///
    /// Should return an error if:
    /// - exponent addition would overflow, or
    /// - the monomials have mismatched variable counts.
    fn checked_mul_monomial(&self, m: &Self::Mono) -> Result<Self, Self::Error>
    where
        Self::Field: Clone;

    /// Multiplies this term by a monomial.
    ///
    /// # Panics
    ///
    /// Panics if the underlying monomial multiplication fails
    /// (overflow / mismatched variable counts).
    #[inline]
    fn mul_monomial(&self, m: &Self::Mono) -> Self
    where
        Self::Field: Clone,
        Self::Error: Debug,
    {
        self.checked_mul_monomial(m)
            .expect("TermLike::mul_monomial: monomial multiplication failed")
    }
}

pub use dynamic::DynamicTerm;
pub use term::Term;
