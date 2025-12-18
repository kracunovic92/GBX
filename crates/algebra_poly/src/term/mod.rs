//! Polynomial primitives: terms (coefficient × monomial).
//!
//! This module exposes:
//! - [`Term`]: term with a compile-time number of variables.
//! - [`DynamicTerm`]: term with a runtime number of variables.
//! - [`TermLike`]: common interface used by polynomial algorithms.
//!
//! The monomial part is provided by `crate::monomial`.

mod dynamic;
mod term;

use crate::monomial::MonomialError;
use crate::monomial::MonomialLike;
use algebra_core::{CheckedDiv, Field, Zero};
use core::fmt::Debug;

/// Errors that can occur when operating on terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TermError {
    /// Error arising from monomial operations (e.g. exponent overflow,
    /// mismatched variable counts for dynamic monomials).
    Monomial(MonomialError),
}

impl From<MonomialError> for TermError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        TermError::Monomial(e)
    }
}

/// Common interface for polynomial terms (coefficient × monomial).
///
/// Implemented by:
/// - [`Term`] for fixed-size monomials.
/// - [`DynamicTerm<F>`] for dynamic monomials.
///
/// This trait is the natural bound for polynomial algorithms that should
/// work with either representation.
pub trait TermLike: Clone + PartialEq {
    /// The underlying field of coefficients.
    type Field: Field;

    /// The monomial representation used by this term.
    ///
    /// This is typically:
    /// - [`crate::monomial::Monomial`] for `Term<F, N>`.
    /// - [`crate::monomial::DynamicMonomial`] for `DynamicTerm<F>`.
    type Mono: MonomialLike<Error = MonomialError> + Clone;

    /// Error type for fallible term operations.
    ///
    /// For the built-in implementations this is [`TermError`].
    /// We require `From<TermError>` so default methods can use `?` ergonomically.
    type Error: From<TermError>;

    /// Constructs a term from its parts.
    ///
    /// This enables default implementations in the trait without knowing
    /// the concrete term representation.
    fn from_parts(coeff: Self::Field, mono: Self::Mono) -> Self;

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
    /// # Errors
    ///
    /// Returns an error if monomial multiplication fails
    /// (overflow or mismatched variable counts).
    #[inline]
    fn checked_mul_monomial(&self, m: &Self::Mono) -> Result<Self, Self::Error>
    where
        Self::Field: Clone,
        Self: Sized,
    {
        let mono = self
            .mono()
            .checked_mul(m)
            .map_err(TermError::Monomial)?;
        Ok(Self::from_parts(
            self.coeff()
                .clone(),
            mono,
        ))
    }
    /// Checked multiplication of two terms.
    ///
    /// Mathematically:
    /// `(a * x^α) · (b * x^β) = (a · b) * x^(α + β)`.
    ///
    /// # Errors
    ///
    /// Returns an error if monomial multiplication fails (overflow / mismatch).
    #[inline]
    fn checked_mul_term(&self, other: &Self) -> Result<Self, Self::Error>
    where
        Self: Sized,
        Self::Field: Clone,
        Self::Error: From<MonomialError>,
    {
        let coeff = self
            .coeff()
            .clone()
            * other
                .coeff()
                .clone();
        let mono = self
            .mono()
            .checked_mul(other.mono())
            .map_err(Into::into)?;
        Ok(Self::from_parts(coeff, mono))
    }

    /// Multiplies two terms.
    ///
    /// # Panics
    ///
    /// Panics if monomial multiplication fails (overflow / mismatch).
    #[inline]
    fn mul_term(&self, other: &Self) -> Self
    where
        Self: Sized,
        Self::Field: Clone,
        Self::Error: From<MonomialError> + Debug,
    {
        self.checked_mul_term(other)
            .expect("TermLike::mul_term: monomial multiplication failed")
    }

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
        Self: Sized,
    {
        self.checked_mul_monomial(m)
            .expect("TermLike::mul_monomial: monomial multiplication failed")
    }

    /// Returns `true` if this term is divisible by `divisor`:
    /// - monomial divisibility, and
    /// - divisor coefficient is nonzero.
    #[inline]
    fn is_divisible_by(&self, divisor: &Self) -> bool
    where
        Self::Field: Zero,
    {
        !divisor
            .coeff()
            .is_zero()
            && divisor
                .mono()
                .divides(self.mono())
    }

    /// Checked term division.
    ///
    /// Computes `self / divisor` if:
    /// - `divisor.coeff != 0`, and
    /// - `divisor.mono | self. Mono`.
    ///
    /// Returns `None` if not divisible.
    #[inline]
    fn checked_div_term(&self, divisor: &Self) -> Option<Self>
    where
        Self: Sized,
        Self::Field: Clone + Zero,
    {
        if divisor
            .coeff()
            .is_zero()
        {
            return None;
        }

        let monomial = self
            .mono()
            .div_by(divisor.mono())?;

        let coeff = self
            .coeff()
            .clone()
            .checked_div(
                divisor
                    .coeff()
                    .clone(),
            )
            .ok()?;

        Some(Self::from_parts(coeff, monomial))
    }
}

pub use dynamic::DynamicTerm;
pub use term::Term;
