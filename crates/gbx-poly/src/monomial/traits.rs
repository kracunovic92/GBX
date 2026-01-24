//! Monomial traits.
//!
//! We intentionally split the API into two layers:
//!
//! - [`MonomialView`]: read-only access to the exponent vector (small, ubiquitous).
//! - [`Monomial`]: construction + checked arithmetic (used by algorithms).
//!
//! This separation keeps generic code lightweight: term orders, sorting, and
//! read-only inspection only require [`MonomialView`]. Construction and
//! arithmetic require [`Monomial`].

use core::cmp::Ordering;

/// A monomial in `n` variables is an exponent vector `α ∈ ℕ^n`, representing
/// `x^α = ∏ x_i^{α_i}`.
///
/// This trait is intentionally small so internal representations can change
/// (array, boxed slice, smallvec, packed bits, SIMD...) without changing
/// downstream code.
pub trait MonomialView: Clone + Eq {
    /// Number of variables in this monomial.
    fn n_vars(&self) -> usize;

    /// Exponent slice. Must have length `n_vars()`.
    fn exponents(&self) -> &[u32];
}

/// This is the trait algorithms use when they need to *build* new monomials
/// (e.g. multiplication, LCM, quotient) rather than just inspect them.
///
/// # Error model
/// Implementations should use their own error type. For built-in monomials,
/// this is typically `MonomialError`.
pub trait Monomial: MonomialView {
    /// Error type for fallible operations.
    type Error;

    /// Construct a monomial from an exponent iterator.
    ///
    /// `n_vars` is the expected variable count. For fixed-size monomials,
    /// `n_vars` must match the compile-time arity.
    ///
    /// Implementations should error if the iterator yields the wrong length.
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = u32>;

    /// Checked total degree (sum of exponents).
    ///
    /// Production note: for performance, implementations are encouraged to
    /// cache degree and validate it at construction time. This method exists
    /// to support representations that do not cache it.
    fn degree_checked(&self) -> Result<u32, Self::Error>;

    /// Checked multiplication (componentwise exponent addition).
    ///
    /// For dynamic monomials, implementations should error on mismatched
    /// variable counts. For fixed-size monomials, the type system guarantees
    /// equal arity.
    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error>;
}

/// Convenience methods available for any [`MonomialView`].
///
/// Keeping these as an *extension trait* avoids bloating the core traits and
/// avoids imposing extra bounds (like `Debug`) on implementers.
pub trait MonomialViewExt: MonomialView {
    /// Returns `true` if this monomial is the multiplicative identity `1`,
    /// i.e. all exponents are zero.
    #[inline]
    fn is_one(&self) -> bool {
        self.exponents().iter().all(|&e| e == 0)
    }

    /// This is infallible and useful in contexts where you need a degree-like
    /// quantity for ordering/heuristics and cannot return a `Result`.
    #[inline]
    fn total_degree_u128(&self) -> u128 {
        self.exponents().iter().map(|&e| e as u128).sum()
    }

    /// Compare two monomials lexicographically on exponent vectors.
    ///
    /// This is a tiny utility sometimes useful in tests; prefer the dedicated
    /// term order types for Gröbner algorithms.
    #[inline]
    fn cmp_lex(&self, other: &Self) -> Ordering {
        assert_eq!(
            self.n_vars(),
            other.n_vars(),
            "cmp_lex called with different variable counts"
        );
        for (&a, &b) in self.exponents().iter().zip(other.exponents().iter()) {
            match a.cmp(&b) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        Ordering::Equal
    }
}

impl<T: MonomialView> MonomialViewExt for T {}
