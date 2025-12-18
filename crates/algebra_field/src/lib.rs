#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! `algebra_field`: concrete fields like `Zp` (prime fields) built on
//! the algebraic traits in `algebra_core`.
//!
//! Currently, this crate provides:
//! - [`Zp`]: finite field-like type for integers modulo `P`.
//! - convenient aliases [`F2`], [`F3`], [`F5`], [`F7`].

pub mod complex;
/// Public export of the mod
pub mod zp;

/// Prime field `GF(P)` with a **type-level** modulus `P`.
///
/// See [`zp::Zp`] for details and examples.
pub use zp::Zp;


pub use complex::{C32, C64};

/// Binary field GF(2).
pub type F2 = Zp<2>;
/// GF(3).
pub type F3 = Zp<3>;
/// GF(5).
pub type F5 = Zp<5>;
/// GF(7).
pub type F7 = Zp<7>;
