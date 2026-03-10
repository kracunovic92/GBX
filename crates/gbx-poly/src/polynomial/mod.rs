//! Sparse multivariate polynomials (context-driven).
//!
//! This module provides a generic sparse polynomial container and the traits
//! used by algorithms (reduction, Gröbner basis, etc.).
//!
//! Polynomials are evaluated inside a [`RingCtx`](crate::ring::RingCtx):
//! - **field arithmetic** comes from `ctx.field` (static or dynamic)
//! - **monomial order** comes from `ctx.order`
//! - **ring arity** comes from `ctx.nvars`
//!
//! The polynomial value itself is intentionally small:
//! - it stores only a [`RingId`](crate::ring::RingId) tag and a term storage backend
//! - it does *not* store modulus/order/arity
//!
//! ## Normalized (canonical) form
//! Normalized form means:
//! - no zero coefficients (`ctx.field.is_zero`)
//! - no duplicate monomials (merged with `ctx.field.add`)
//! - terms sorted descending with respect to `ctx.order`
//!
//! Construction via [`PolynomialMut::from_terms_in`] and [`PolynomialMut::normalize_in_place`]
//! enforces these invariants.
//!
//! ## Construction macros
//! The `poly!` and `terms!` macros help build test polynomials quickly.
//! They are storage-agnostic: they build `Vec<Term<..>>` then call
//! [`PolynomialMut::from_terms_in`].
//!
//! ## Common combinations (examples)
//!
//! ### Static field + fixed arity
//! ```
//! use gbx_poly::order::Lex;
//! use gbx_poly::polynomial::{PolyFixed, PolynomialView};
//! use gbx_poly::ring::{Ring, StaticFpCtx};
//! use gbx_field::fp::Fp;
//! use gbx_poly::poly;
//!
//! let ring = Ring::builder()
//!     .field(StaticFpCtx::<7>::new())
//!     .order(Lex)
//!     .nvars(2)
//!     .build()
//!     .unwrap();
//!
//! type P2 = PolyFixed<Fp<7>, 2>;
//! let p: P2 = poly![&ring; (3,[1,0]), (5,[1,0]), (1,[0,0])].unwrap();
//! assert_eq!(p.len(), 2);
//! ```
//!
//! ### Dynamic field + dynamic arity
//! ```
//! use gbx_poly::order::Lex;
//! use gbx_poly::polynomial::{PolyDyn, PolynomialView};
//! use gbx_field::fp::{FpDyn, FpDynElem};
//! use gbx_poly::poly;
//! use gbx_poly::ring::Ring;
//!
//! let ring = Ring::builder()
//!     .field(FpDyn::prime(7).unwrap())
//!     .order(Lex)
//!     .nvars(3)
//!     .build()
//!     .unwrap();
//!
//! type P = PolyDyn<FpDynElem>;
//! let p: P = poly![&ring; (3,[1,0,2]), (5,[1,0,2]), (4,[0,7,0])].unwrap();
//! assert_eq!(p.len(), 2);
//! ```
//!
//! ## Errors
//! Many operations validate ring identity and will return [`PolynomialError::Ring`]
//! when mixing polynomials from different rings.

mod display;
mod display_macros;
pub mod error;
mod macros;
mod normalize;
pub mod ops;
pub mod poly;
pub mod reduce;
pub mod traits;
mod types;

pub use display::{PolyDisplay, PolyStyle};
pub use error::{PolynomialError, Result};
pub use normalize::normalize_terms_in;
pub use ops::PolynomialOps;
pub use poly::Polynomial;
pub use reduce::{PolynomialReduce, ReduceError};
pub use traits::{PolynomialMut, PolynomialView};
pub use types::*;
