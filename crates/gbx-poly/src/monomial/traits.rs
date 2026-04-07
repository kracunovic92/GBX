//! Monomial traits.
//!
//! - [`MonomialView`]: read-only access to the exponent vector (small, ubiquitous).
//! - [`Monomial`]: construction + checked arithmetic (used by algorithms).
//!
//! This separation keeps generic code lightweight: term orders, sorting, and
//! read-only inspection only require [`MonomialView`]. Construction and
//! arithmetic require [`Monomial`].

use core::cmp::Ordering;

use super::error::{MonomialError, Result};

/// Read-only access to a monomial exponent vector `α ∈ ℕ^n`.
///
/// This trait is intentionally minimal so internal representations can vary
/// (arrays, boxed slices, smallvec, packed words, SIMD...) without impacting
/// downstream code.
///
/// # Contract
/// - [`MonomialView::exponents`] must return a slice whose length is the arity.
/// - `n_vars()` defaults to `exponents().len()` and should not be overridden
///   unless it remains exactly consistent with `exponents().len()`.
pub trait MonomialView {
    /// Exponent word type (typically `u32`).
    type Word: Copy + Eq;

    /// Exponent slice (`α₀, …, α_{n-1}`).
    fn exponents(&self) -> &[Self::Word];

    /// Number of variables (arity).
    #[inline]
    fn n_vars(&self) -> usize {
        self.exponents().len()
    }

    /// Optional fast-path for cached total degree (in `u32`).
    ///
    /// Implementations may return `Some(deg)` if they cache the total degree.
    /// If not available, return `None` (default).
    #[inline]
    fn degree_hint(&self) -> Option<u32> {
        None
    }
}

/// A constructive monomial type used by algorithms that build new monomials.
///
/// All fallible operations use the crate's unified [`MonomialError`] error type.
/// This keeps generic Gröbner-basis code ergonomic and makes error handling
/// consistent across built-in and third-party monomial implementations.
///
/// # Contract
/// - `try_from_exponents_iter` must error if the iterator yields the wrong length.
/// - `degree_checked` must check for overflow.
/// - `checked_mul` must check arity (for runtime-arity monomials) and overflow.
pub trait Monomial: MonomialView + Sized {
    /// Construct a monomial from an exponent iterator.
    ///
    /// `n_vars` is the expected variable count. For fixed-size monomials,
    /// `n_vars` must match the compile-time arity.
    ///
    /// Implementations must error if the iterator yields the wrong length.
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self>
    where
        I: IntoIterator<Item = Self::Word>;

    /// Checked total degree (sum of exponents).
    fn degree_checked(&self) -> Result<u32>;

    /// Checked multiplication (componentwise exponent addition).
    ///
    /// For runtime-arity monomials, implementations must error on mismatched arity.
    fn checked_mul(&self, other: &Self) -> Result<Self>;
}

/// Convenience methods available for any [`MonomialView`] with `u32` exponents.
pub trait MonomialViewExtU32: MonomialView<Word = u32> {
    /// Returns `true` if this monomial is the multiplicative identity `1`
    /// (all exponents are zero).
    #[inline]
    fn is_one(&self) -> bool {
        self.exponents().iter().all(|&e| e == 0)
    }

    /// Total degree as `u128` (infallible).
    ///
    /// Useful for heuristics/ordering when you don't want to thread a `Result`.
    #[inline]
    fn total_degree_u128(&self) -> u128 {
        self.exponents().iter().map(|&e| e as u128).sum()
    }

    /// Total degree as `u32` if it fits, otherwise `None`.
    #[inline]
    fn total_degree_u32(&self) -> Option<u32> {
        self.exponents()
            .iter()
            .try_fold(0u32, |acc, &e| acc.checked_add(e))
    }

    /// Lexicographic compare on exponent vectors.
    ///
    /// Returns `None` if arities differ.
    #[inline]
    fn cmp_lex_checked(&self, other: &Self) -> Option<Ordering> {
        if self.n_vars() != other.n_vars() {
            return None;
        }
        Some(self.exponents().iter().cmp(other.exponents().iter()))
    }
}

impl<T: MonomialView<Word = u32> + ?Sized> MonomialViewExtU32 for T {}

/// Convenience methods for constructive monomials (`Monomial`) with `u32` exponents.
pub trait MonomialExtU32: Monomial<Word = u32> {
    /// Construct the multiplicative identity monomial `1`
    /// in a ring with `n_vars` variables.
    #[inline]
    fn one(n_vars: usize) -> Result<Self> {
        Self::try_from_exponents_iter(n_vars, core::iter::repeat(0u32).take(n_vars))
    }
    /// Lexicographic compare on exponent vectors.
    ///
    /// Returns [`MonomialError::MismatchedArity`] if arities differ.
    #[inline]
    fn cmp_lex_result(&self, other: &Self) -> Result<Ordering> {
        if self.n_vars() != other.n_vars() {
            return Err(MonomialError::MismatchedArity { lhs: self.n_vars(), rhs: other.n_vars() });
        }
        Ok(self.exponents().iter().cmp(other.exponents().iter()))
    }
}

impl<T: Monomial<Word = u32> + ?Sized> MonomialExtU32 for T {}
