//! Graph input / output utilities.
//!
//! Currently supported formats:
//! - DIMACS (`p edge` / `p col`)

#[cfg(feature = "dimacs")]
pub mod dimacs;

pub mod error;

#[cfg(feature = "dimacs")]
pub use dimacs::{read_dimacs, read_dimacs_file};

pub use error::DimacsError;
