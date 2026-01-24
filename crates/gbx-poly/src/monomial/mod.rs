//! Monomials (exponent vectors) and term orders.
//!
//! A **monomial** in `n` variables is an exponent vector `α ∈ ℕ^n` representing
//!
//! ```text
//! x^α = x₀^{α₀} * x₁^{α₁} * ... * x_{n-1}^{α_{n-1}}.
//! ```
//!
//! This module provides two concrete monomial types:
//! - ['FixedMonomial<{ N }>']: fixed arity known at compile time.
//! - [`DynamicMonomial`]: arity stored at runtime.
//!
//! Both types implement [`MonomialView`] (read-only) and [`Monomial`] (constructive).
//!
//! ## Term orders
//! Gröbner basis algorithms require a fixed term order. We provide:
//! - [`Lex`]
//! - [`Grevlex`]
//!
//! Orders are defined on exponent slices, so they work with any monomial type that
//! exposes `exponents()`.
//!
//! ## Errors
//! Invariants such as matching arity and checked arithmetic are reported via
//! [`MonomialError`].

mod algos;
mod dynamic;
mod fixed;
mod order;
pub(crate) mod traits;

pub use algos::{checked_gcd, checked_lcm, checked_quotient, divides, MonomialAlgos};
pub use dynamic::DynamicMonomial;
pub use fixed::FixedMonomial;
pub use order::{Grevlex, Lex, MonomialOrder};
pub use traits::{Monomial, MonomialView, MonomialViewExt};

/// Errors that can occur when operating on monomials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum MonomialError {
    /// Overflow during component-wise exponent addition.
    ExponentOverflow { index: usize, lhs: u32, rhs: u32 },

    /// Total degree computation overflowed `u32`.
    DegreeOverflow,

    /// The two monomials had different numbers of variables.
    MismatchedVariableCount { lhs: usize, rhs: usize },

    /// Constructor received an unexpected exponent count.
    WrongLength { expected: usize, got: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_surface_compiles() {
        fn _takes_monomial<M: Monomial>(_m: &M) {}

        let m2 = FixedMonomial::<2>::from_exponents([1, 0]);
        _takes_monomial(&m2);

        let md = DynamicMonomial::from_slice(&[1, 0, 3]);
        _takes_monomial(&md);

        // Orders are callable
        let _ = Lex::cmp(&m2, &m2);
        let _ = Grevlex::cmp(&md, &md);

        // Algorithms are callable
        assert!(divides(&m2, &FixedMonomial::<2>::from_exponents([2, 0])));
        let _ = checked_lcm(&m2, &FixedMonomial::<2>::from_exponents([2, 3])).unwrap();
    }
}
