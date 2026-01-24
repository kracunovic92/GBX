//! Sparse multivariate polynomials.
//!
//! This module is designed around three ideas:
//!
//! 1) **Concrete public types** follow the `Fixed*` / `Dynamic*` convention used
//!    throughout the crate:
//!    - [`FixedPolynomial`]: compile-time arity (`const N: usize`)
//!    - [`DynamicPolynomial`]: runtime arity
//!
//! 2) **Algorithms depend on small traits**, not concrete types:
//!    - [`PolynomialView`]: read-only interface (slice of terms + leading term)
//!    - [`PolynomialMut`]: minimal construction/mutation hooks used by algorithms
//!
//! 3) A single **generic engine type** powers all concrete polynomial types:
//!    - [`Polynomial`]: parameterized by term type, monomial order, and storage backend.
//!
//! The generic `Polynomial<...>` is intentionally not the primary user-facing type.
//! Most users should use `FixedPolynomial` or `DynamicPolynomial` type aliases.
//!
//! # Storage backends
//!
//! The internal term container is abstracted via `gbx-storage` (`TermStorage`).
//! This allows using different representations (e.g. `Vec`-based, arena-packed, etc.)
//! without changing polynomial algorithms.
//!
//! # Normalization invariants
//!
//! Concrete polynomial types are expected to maintain these invariants after
//! normalization (constructors/mutators must enforce them):
//! - no zero coefficients
//! - no duplicate monomials
//! - terms sorted in **descending** order w.r.t. the monomial order `O`
//!
//! Keeping terms sorted makes `leading_term()` an O(1) operation.

mod error;
mod normalize;
mod poly;
mod traits;
mod types;
