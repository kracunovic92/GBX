//! Graph input / output utilities.
//!
//! Currently supported formats:
//! - DIMACS (edge / col)

/// Handling basic dimacs structure
pub mod dimacs;

// Re-export the most commonly used DIMACS helpers at `graph::io::*`
pub use dimacs::{DimacsError, read_dimacs, read_dimacs_file};
