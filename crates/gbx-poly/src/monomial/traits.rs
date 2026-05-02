//! Monomial traits.
//!
//! This module contains read-only monomial traits used by orders, display,
//! sorting, and algorithms.
//!
//! Construction and arithmetic live directly on the concrete
//! [`crate::monomial::Monomial`] type.

/// Read-only access to a monomial exponent vector `α ∈ ℕ^n`.
pub trait MonomialView {
    /// Exponent slice.
    fn exponents(&self) -> &[u32];

    /// Number of variables.
    #[inline]
    fn n_vars(&self) -> usize {
        self.exponents().len()
    }

    /// Total degree.
    fn degree(&self) -> u32;

    /// Returns `true` if this monomial is the multiplicative identity.
    #[inline]
    fn is_one(&self) -> bool {
        self.degree() == 0
    }
}
