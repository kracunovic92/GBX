#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # algebra_core
//!
//! Foundational algebraic traits and law helpers.
//!
//! This crate provides **interfaces only** (traits for additive, multiplicative,
//! semiring, ring, and field structures). It intentionally avoids implementing
//! those traits for foreign types to respect Rust’s orphan rules.
//!
//! Concrete implementations live in sibling crates such as `algebra_field`
//! (e.g., prime fields `Zp`) or `algebra_poly` (polynomials over a field).
//!
//! ## Design goals
//!
//! - Small, composable traits (e.g. `Additive`, `Multiplicative`, `Semiring`).
//! - Law-focused: traits are meant to be testable using *law helpers* in
//!   this crate’s test utilities (see the `tests` folder and the `laws` module).
//! - No global impls for standard types to avoid surprising semantics.
//!
//! ## Example
//!
//! ```rust
//! use algebra_core::prelude::*;
//! use std::ops::{Mul,Add};
//!
//! #[derive(Clone, Copy, Debug, PartialEq, Eq)]
//! struct MyMod2(u8);
//!
//! impl Zero for MyMod2 {
//!     const ZERO: Self = Self(0);
//!
//!     fn zero() -> Self {
//!         Self(0)
//!     }
//!
//!     fn is_zero(&self) -> bool {
//!         self.0 % 2 == 0
//!     }
//! }
//!
//! impl One for MyMod2 {
//!     const ONE: Self = Self(1);
//!
//!     fn one() -> Self {
//!         Self(1)
//!     }
//! }
//!
//! impl Add for MyMod2 {
//!     type Output = Self;
//!
//!     fn add(self, rhs: Self) -> Self::Output {
//!         Self((self.0 + rhs.0) % 2)
//!     }
//! }
//!
//! impl Mul for MyMod2 {
//!     type Output = Self;
//!
//!     fn mul(self, rhs: Self) -> Self::Output {
//!         Self((self.0 * rhs.0) % 2)
//!     }
//! }
//!
//! ```
//!
//! In practice, you’ll implement `Additive`, `Multiplicative`, and the identity
//! traits, then opt into higher structures like `Semiring`, `Ring`, or `Field`
//! when the laws are satisfied.

/// Implementation for basic primitives.
pub mod impls;
/// Core algebraic traits.
pub mod traits;

/// Commonly used traits re-exported for convenient glob imports.
///
/// ```rust
/// use algebra_core::prelude::*;
/// ```
pub mod prelude {
    pub use crate::traits::additive::{
        AddAbelianGroup, AddGroup, AddMonoid, AddSemigroup, Additive,
    };
    pub use crate::traits::field::{CheckedDiv, DivByZero, Field, TryInverse};
    pub use crate::traits::identity::{One, Zero};
    pub use crate::traits::multiplicative::{MulMonoid, MulSemigroup, Multiplicative};
    pub use crate::traits::ring::Ring;
    pub use crate::traits::semiring::Semiring;
}

/// Re-export the most commonly used traits at the crate root for convenience.
///
/// This allows:
/// ```rust
/// use algebra_core::{Additive, Multiplicative, Semiring};
/// ```
pub use traits::{
    additive::{AddAbelianGroup, AddGroup, AddMonoid, AddSemigroup, Additive},
    field::{CheckedDiv, Field, TryInverse},
    identity::{One, Scalar, Zero},
    multiplicative::{MulAbelianMonoid, MulMonoid, MulSemigroup, Multiplicative},
    ring::Ring,
    semiring::Semiring,
};

