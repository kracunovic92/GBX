//! Monomial orders used by Gröbner basis algorithms.
//!
//! A monomial order compares exponent vectors and determines which term is
//! considered larger.
//!
//! This module provides:
//! - [`Lex`] for lexicographic order,
//! - [`Grevlex`] for graded reverse lexicographic order,
//! - [`OrderSpec`] for runtime-selected orders,
//! - [`MonomialOrder`] as the shared comparison trait.
//!
//! Orders operate on exponent slices and on [`crate::monomial::MonomialView`].
//!
//! # Example
//!
//! ```
//! use gbx_poly::monomial::Monomial;
//! use gbx_poly::order::{LEX, MonomialOrder};
//!
//! let a = Monomial::from_slice(&[1, 0]);
//! let b = Monomial::from_slice(&[0, 5]);
//!
//! assert!(LEX.cmp(&a, &b).is_gt());
//! ```
//!
//! # Arity
//!
//! Comparisons assume both monomials have the same number of variables.
//! A mismatch is considered a programmer error and will panic.

mod grevlex;
mod lex;
mod spec;
pub(crate) mod traits;

pub use grevlex::{GREVLEX, Grevlex};
pub use lex::{LEX, Lex};
pub use spec::{OrderParseError, OrderSpec};
pub use traits::MonomialOrder;
