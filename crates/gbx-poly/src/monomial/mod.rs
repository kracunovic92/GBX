//! Monomials (exponent vectors)
//!
//! A **monomial** in `n` variables is an exponent vector `α ∈ ℕ^n` representing
//!
//! ```text
//! x^α = x₀^{α₀} * x₁^{α₁} * ... * x_{n-1}^{α_{n-1}}.
//! ```
//!
//! ## Types
//! - [`FixedMonomial<N>`]: fixed arity known at compile time.
//! - [`DynamicMonomial`]: arity stored at runtime.
//!
//! Both implement [`MonomialView`] (read-only) and [`Monomial`] (constructive).
//!
//! ## Term orders
//! Gröbner basis algorithms require a fixed term order. We provide:
//! - [`Lex`]
//! - [`Grevlex`]
//!
//! Orders operate on exponent slices (`u32` by default), so they work with any monomial exposing
//! [`MonomialView::exponents`].
//!
//! ## Errors
//! Invariants such as matching arity and checked arithmetic are reported via
//! [`MonomialError`].

mod algos;
mod display;
mod dynamic;
mod error;
mod fixed;
mod macros;
mod traits;

/// This should represent public interface for other
pub mod prelude {
    pub use super::{checked_gcd, checked_lcm, checked_quotient, divides, DynamicMonomial, FixedMonomial, Monomial, MonomialAlgos, MonomialError, MonomialView, MonomialViewExtU32, Result};
}

pub use algos::{checked_gcd, checked_lcm, checked_lcm_degree, checked_quotient, divides, gcd_is_one, MonomialAlgos};
pub use display::MonomialDisplay;
pub use dynamic::DynamicMonomial;
pub use error::{MonomialError, Result};
pub use fixed::FixedMonomial;
pub use traits::{Monomial, MonomialExtU32, MonomialView, MonomialViewExtU32};
