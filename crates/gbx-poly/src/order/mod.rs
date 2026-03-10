//! Monomial orders (term orders) for Gröbner basis computations.
//!
//! This module defines:
//! - [`MonomialOrder`] – a trait for total orders on exponent vectors.
//! - [`Lex`]           – lexicographic order.
//! - [`Grevlex`]       – graded reverse lexicographic order.
//! - [`OrderSpec`]     – runtime-selectable order parsed from strings.
//!
//! Orders are defined over exponent slices (`&[u32]`) rather than a specific
//! monomial type. Any monomial representation can use these orders as long as it
//! can expose its exponent vector (e.g. via [`crate::monomial::MonomialView::exponents`]).
//!
//! # Static vs runtime selection
//!
//! **Static (compile-time)**: use the ZST markers [`Lex`] / [`Grevlex`] (fast, monomorphized).
//!
//! ```
//! use gbx_poly::order::{LEX, MonomialOrder};
//! use gbx_poly::monomial::FixedMonomial;
//!
//! let a = FixedMonomial::<2>::from_exponents([1, 0]);
//! let b = FixedMonomial::<2>::from_exponents([0, 5]);
//!
//! // Compare with lex order
//! let _ = LEX.cmp(&a, &b);
//! ```
//!
//! **Runtime (config/CLI)**: parse into [`OrderSpec`].
//!
//! ```
//! use core::str::FromStr;
//! use gbx_poly::order::{MonomialOrder, OrderSpec};
//! use gbx_poly::monomial::FixedMonomial;
//!
//! let o = OrderSpec::from_str("grevlex").unwrap();
//! let a = FixedMonomial::<3>::from_exponents([1, 0, 0]);
//! let b = FixedMonomial::<3>::from_exponents([0, 2, 0]);
//! let _ = o.cmp(&a, &b);
//! ```
//!
//! # Panics
//! All comparisons assume both exponent vectors have the same length.
//! A mismatch indicates a programmer error (mixing rings / arities) and will panic.

mod grevlex;
mod lex;
mod spec;
mod traits;

pub use grevlex::{Grevlex, GREVLEX};
pub use lex::{Lex, LEX};
pub use spec::{OrderParseError, OrderSpec};
pub use traits::MonomialOrder;
