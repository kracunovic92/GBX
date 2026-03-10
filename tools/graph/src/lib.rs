#![deny(rustdoc::broken_intra_doc_links)]
#![allow(missing_docs)]
#![forbid(unsafe_code)]
//! Internal graph tooling for GBX.
//!
//! This crate is intentionally **not** part of the Gröbner core library.
//! It provides:
//! - simple graph data structures
//! - I/O helpers (DIMACS)
//! - encodings from graphs to constraint systems (e.g. k-coloring)

pub mod encoding;
pub mod graph;
pub mod io;

/// Common behavior of “graph-like” structures.
///
/// Convention:
/// - Vertices are 1-based: `1..=n`.
/// - `neighbors(v)` returns outgoing neighbors for directed graphs.
pub trait GraphLike {
    fn vertex_count(&self) -> usize;
    fn neighbors(&self, v: usize) -> &[usize];
    fn is_directed(&self) -> bool;
}

// “Public surface”
pub use encoding::color::{build_k_coloring_system, BoolPolyBuilder, ColoringEncoding, VarIndex};
pub use encoding::error::EncodingError;

pub use graph::error::GraphError;
pub use graph::simple::Graph;

#[cfg(feature = "dimacs")]
pub use io::dimacs::{read_dimacs, read_dimacs_file};
#[cfg(feature = "dimacs")]
pub use io::error::DimacsError;
