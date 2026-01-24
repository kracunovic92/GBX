//! Law checkers for algebraic structures.
//!
//! These helpers validate algebraic laws (associativity, distributivity, etc.)
//! on a finite test set (a slice of elements).
//!
//! They are intended for *tests* of concrete implementations.
//!
//! Design:
//! - [`ops`] contains atomic laws (assoc/comm/identity/inverse/distrib/absorbing).
//! - other modules compose them into structure checkers (ring/field/etc.).

/// Primitive `+/*` law checks (associativity, commutativity, identities, distributivity...).
pub mod ops;

/// Additive-structure checkers built from [`ops`].
pub mod additive;

/// Field checkers (inverses, division laws) built from [`ops`] and additive/multiplicative checks.
pub mod field;

/// Multiplicative-structure checkers built from [`ops`].
pub mod multiplicative;

mod ops_generic;

/// Ring checkers built from additive + multiplicative + distributivity.
pub mod ring;

/// Semiring checkers built from additive monoid + multiplicative monoid + distributivity.
pub mod semiring;

mod tests;

pub use additive::*;
pub use field::*;
pub use multiplicative::*;
pub use ring::*;
pub use semiring::*;
