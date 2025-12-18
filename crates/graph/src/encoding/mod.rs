//! Encodings from graphs into algebraic / boolean constraint systems.
//!
//! Currently supported:
//! - k-coloring encoding (boolean polynomial system)

/// Module for NP coloring problem
pub mod coloring;

pub use coloring::{build_k_coloring_system, BoolPolyBuilder, ColoringEncoding, VarIndex};
