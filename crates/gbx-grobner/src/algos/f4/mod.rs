//! F4 Gröbner basis algorithm.
//!
//! This module contains a correctness-first, CPU-only F4 architecture.
//! The initial implementation is intentionally simple:
//! - select a batch of critical pairs
//! - build their S-polynomials
//! - perform naive symbolic preprocessing
//! - construct a matrix
//! - row-reduce it
//! - extract candidate new basis elements

pub mod api;
pub mod batch;
pub mod engine;
pub mod error;
pub mod extract;
pub mod matrix;
pub mod options;
pub mod symbolic;
mod trace;

pub use api::{f4, f4_traced};
pub use error::{F4Error, Result};
pub use options::F4Options;
pub use trace::*;
