//! F4 Gröbner basis algorithm.
//!
//! This module contains a correctness-first, CPU-oriented F4 implementation.
//!
//! The implementation is context-driven:
//! - coefficient arithmetic comes from `RingCtx::field`,
//! - monomial order comes from `RingCtx::order`,
//! - variable count comes from `RingCtx::nvars`,
//! - ring identity is checked through polynomial `RingId` tags.
//!
//! Current pipeline:
//! - select a batch of critical pairs,
//! - build S-polynomial products,
//! - perform symbolic preprocessing,
//! - build a coefficient matrix,
//! - row-reduce the matrix,
//! - extract candidate basis elements,
//! - insert nonzero reductions into the basis.
//!
//! The module layout keeps each phase replaceable so symbolic preprocessing,
//! sparse matrix construction, and reduction reuse can be improved independently.

pub mod api;
pub mod engine;
pub mod error;
pub mod options;
pub mod state;
pub mod types;

pub mod extract;
pub mod linear;
pub mod pairs;
mod pipeline;
pub mod symbolic;

pub use api::f4;
pub use error::{F4Error, Result};
pub use options::F4Options;
