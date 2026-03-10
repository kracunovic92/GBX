//! Buchberger Gröbner basis algorithm.
//!
//! This module provides a baseline implementation of Buchberger's algorithm for
//! computing Gröbner bases in a polynomial ring.
//!
//! The main entry points are:
//!
//! - [`buchberger`] for the default configuration
//! - [`buchberger_with`] for custom pair management
//!
//! # Behavior
//!
//! Starting from the input generators, the algorithm repeatedly:
//!
//! 1. selects a critical pair,
//! 2. forms its S-polynomial,
//! 3. reduces that S-polynomial modulo the current basis,
//! 4. inserts each nonzero remainder into the basis.
//!
//! The final basis can optionally be post-processed into a minimal or reduced
//! Gröbner basis; see [`BasisPost`].

mod api;
mod bounds;
mod engine;
mod init;
mod options;

pub use api::{buchberger, buchberger_with, buchberger_with_tracer};
pub use options::{BasisPost, BuchbergerOptions};
pub use trace::{SharedTracer, TraceCfg, Tracer, TracingPairCriterion, TracingPairKey, TracingQueue};

#[cfg(test)]
mod tests;
pub mod trace;
