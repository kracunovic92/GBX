#![deny(rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]
//! Graph data structures, parsers (DIMACS), and encodings used in GBX.
//!
//! This crate provides a simple undirected [`Graph`], I/O helpers (e.g. DIMACS),
//! and utilities to build polynomial encodings (e.g. k-coloring constraints).

/// Core graph data structures.
pub mod graph;

/// Encodings from graphs into algebraic systems (e.g., k-coloring).
pub mod encoding;

/// Graph I/O (e.g., DIMACS parsing).
pub mod io;

/// Common behavior of "graph-like" structures.
///
/// This allows algorithms to work over both undirected graphs (`Graph`)
/// and future directed graphs (`Digraph`) using the same interface.
///
/// Convention:
/// - Vertices are 1-based: `1..=n`.
/// - `neighbors(v)` returns outgoing neighbors for directed graphs.
pub trait GraphLike {
    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize;

    /// Returns the neighbors of vertex `v`.
    fn neighbors(&self, v: usize) -> &[usize];

    /// Whether this graph is directed.
    fn is_directed(&self) -> bool;
}

pub use graph::Graph;
pub use io::{DimacsError, read_dimacs, read_dimacs_file};
