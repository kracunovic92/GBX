//! F4 Gröbner basis algorithm.
//!
//! This module contains a correctness-first, CPU-oriented F4 implementation.
//! The current pipeline is intentionally simple:
//!
//! - select a batch of critical pairs,
//! - build batch input from those pairs,
//! - perform symbolic preprocessing,
//! - construct the coefficient matrix,
//! - row-reduce it,
//! - extract candidate basis elements.
//!
//! The module layout is designed so that individual stages can later be replaced
//! by more faithful or more optimized variants, including improved F4-style
//! symbolic preprocessing and reduction reuse.

pub mod api;
pub mod engine;
pub mod error;
pub mod options;
pub mod state;
pub mod types;

pub mod extract;
pub mod linear;
pub mod pairs;
pub mod symbolic;
pub mod trace;

pub use api::{f4, f4_traced};
pub use error::{F4Error, Result};
pub use options::F4Options;
pub use trace::*;
