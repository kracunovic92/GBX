#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

//! # gbx-alg
//!
//! Foundational algebraic traits for the GBX ecosystem.
//!
//! ## What this crate contains
//! - **Capability traits** wrapping `core::ops` (e.g. [`Additive`], [`Multiplicative`]).
//! - **Identity traits** ([`Zero`], [`One`]).
//! - **Marker traits** for algebraic structures ([`Semiring`], [`Ring`], [`Field`]).
//! - Optional **law checking helpers** behind the `laws` feature.
//!
//! ## Design philosophy
//! The type system can enforce that operations *exist*, but cannot enforce algebraic *laws*
//! (associativity, commutativity, distributivity). Those are validated with tests via
//! `gbx_alg::laws` helpers.
//!
//! Concrete implementations live in other crates (e.g. `gbx-field`, `gbx-poly`).

pub mod prelude;
pub mod traits;

mod impls;

#[cfg(feature = "laws")]
pub mod laws;

pub use traits::{
    additive::{AddAbelianGroup, AddGroup, AddMonoid, AddSemigroup, Additive, AdditiveAssign},
    field::{CheckedDiv, DivByZero, Field, TryInverse},
    identity::{One, Scalar, Zero},
    multiplicative::{MulAbelianMonoid, MulMonoid, MulSemigroup, Multiplicative, MultiplicativeAssign},
    ring::Ring,
    semiring::Semiring,
};
