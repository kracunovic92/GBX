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
//! - [`buchberger`]
//! - [`buchberger_with`]
//!
//! Post-processing helpers can minimize and reduce a computed basis.

mod algos;
mod basis;
mod display;
pub mod error;
mod instrumentation;

#[cfg(test)]
pub mod test_utils;

pub use algos::*;
pub use basis::GrobnerBasis;
pub use display::{GbDisplay, GbStyle};
pub use error::BuchbergerError;
