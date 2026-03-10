//! Encodings from graphs into algebraic / boolean constraint systems.
//!
//! Currently supported:
//! - k-coloring encoding (boolean polynomial system)

pub mod color;
pub mod error;

pub use color::{build_k_coloring_system, BoolPolyBuilder, ColoringEncoding, VarIndex};
pub use error::EncodingError;
