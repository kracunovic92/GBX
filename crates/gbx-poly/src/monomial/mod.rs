//! Dynamic monomials.
//!
//! A monomial is an exponent vector `α ∈ ℕ^n` representing:
//!
//! ```text
//! x^α = x₀^α₀ * x₁^α₁ * ... * xₙ₋₁^αₙ₋₁
//! ```
//!
//! In GBX, monomials are runtime-sized. The number of variables belongs to the
//! surrounding `RingCtx`, not to the type of the monomial.
//!
//! The main type is [`Monomial`].
//!
//! A monomial stores:
//! - an exponent vector,
//! - a cached total degree.
//!
//! Monomial operations are checked:
//! - multiplication checks exponent overflow,
//! - lcm/gcd/quotient check arity,
//! - degree computation checks overflow.
//!
//! # Example
//!
//! ```
//! use gbx_poly::monomial::{Monomial, MonomialView};
//!
//! let m = Monomial::from_slice(&[1, 0, 2]);
//!
//! assert_eq!(m.exponents(), &[1, 0, 2]);
//! assert_eq!(m.degree(), 3);
//! ```

mod algos;
mod display;
mod error;
mod macros;
mod monomial;
mod traits;

/// Prelude functions
pub mod prelude {
    pub use super::{checked_div_exact, checked_gcd, checked_lcm, checked_lcm_degree, checked_quotient, divides, gcd_is_one, Monomial, MonomialDisplay, MonomialError, MonomialStyle, MonomialView};
}

pub use algos::{checked_div_exact, checked_gcd, checked_lcm, checked_lcm_degree, checked_quotient, divides, gcd_is_one};

pub use display::{MonomialDisplay, MonomialStyle};
pub use error::{MonomialError, MonomialResult};
pub use monomial::Monomial;
pub use traits::MonomialView;
