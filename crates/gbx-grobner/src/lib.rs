//! Gröbner basis algorithms.
//!
//! Algorithms in this crate are context-driven through
//! [`gbx_poly::ring::RingCtx`].
//!
//! The ring context supplies:
//! - coefficient arithmetic,
//! - monomial order,
//! - variable count,
//! - ring identity checks.
//!
//! Main entry points:
//! - [`f4`]
//! - [`F4Options`]
//!
//! Post-processing helpers can minimize and reduce a computed basis.

mod algos;
mod basis;
mod display;
pub mod error;
mod instrumentation;

#[cfg(test)]
/// Test helpers for constructing small polynomial rings.
pub mod test_utils;

pub use algos::{BasisPostOptionsKind, F4Error, F4Options, PostError, Result, engine, extract, f4, linear, make_monic_in_place, minimize_in_place, pairs, reduce_in_place, symbolic, types};
pub use basis::GrobnerBasis;
pub use display::{GbDisplay, GbStyle};
pub use error::BuchbergerError;
