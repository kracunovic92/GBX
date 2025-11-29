//! Core algebraic traits for `algebra_core`.
//!
//! This module groups all public algebraic structures:
//!
//! - [`identity`] – additive and multiplicative identity traits (`Zero`, `One`).
//! - [`additive`] – additive semigroup / monoid / group traits.
//! - [`multiplicative`] – multiplicative semigroup / monoid traits.
//! - [`semiring`] – semiring structure (additive + multiplicative).
//! - [`ring`] – ring structure.
//! - [`field`] – field structure.
//!
//! These traits are re-exported at the crate root and via [`crate::prelude`]
//! for ergonomic use across dependent crates.

pub mod additive;
pub mod field;
pub mod identity;
pub mod multiplicative;
pub mod ring;
pub mod semiring;
// If/when you add law test helpers as public utilities, they might live here:
// pub mod laws;

pub use additive::{AddGroup, AddMonoid, AddSemigroup, Additive};
pub use field::{CheckedDiv, DivByZero, Field, TryInverse};
pub use identity::{One, Zero};
pub use multiplicative::{MulMonoid, MulSemigroup, Multiplicative};
pub use ring::Ring;
pub use semiring::Semiring;
