//! Sparse multivariate polynomials.
//!
//! A polynomial is stored as a sparse list of terms.
//!
//! In GBX, polynomial arithmetic is context-driven:
//! - field arithmetic comes from [`RingCtx::field`](crate::ring::RingCtx::field),
//! - monomial order comes from [`RingCtx::order`](crate::ring::RingCtx::order),
//! - variable count comes from [`RingCtx::nvars`](crate::ring::RingCtx::nvars).
//!
//! The polynomial itself stores:
//! - a [`RingId`](crate::ring::RingId) safety tag,
//! - a `Vec<Term<C>>`.
//!
//! It does not store modulus, order, or variable count.
//!
//! # Canonical form
//!
//! A normalized polynomial satisfies:
//! - no zero coefficients,
//! - no duplicate monomials,
//! - terms sorted descending by the ring order.
//!
//! Construction through [`Polynomial::from_terms_in`] normalizes terms.

mod display;
mod display_macros;
pub mod error;
mod macros;
mod normalize;
pub mod ops;
pub mod poly;
pub mod reduce;
pub mod traits;

pub use display::{PolyDisplay, PolyStyle};
pub use error::{PolynomialError, PolynomialResult};
pub use normalize::normalize_terms_in;
pub use ops::PolynomialOps;
pub use poly::Polynomial;
pub use reduce::{PolynomialReduce, ReduceError};
pub use traits::{PolynomialMut, PolynomialView};
