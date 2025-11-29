//! Create that is used to  represent Graph-like structures

/// Graph mod
pub mod graph;

mod encoding;
/// I/O mod used for creating Grap from files
pub mod io;

/// This trait describes common behavior of "graph-like" structures.
/// Later, both `Graph` (undirected) and `Digraph` (directed) will implement it
pub trait GraphLike {
    /// Returns number of vertices in the graph.
    fn vertex_count(&self) -> usize;

    /// Returns all neighbors of vertex 'v'.
    ///
    ///  For an undirected graph: all vertices 'u' such that {u,v} is an edge.
    ///  For a directed graph: all vertices 'u' such that there is an edge  v -> u (i.e., outgoing neighbors)
    fn neighbors(&self, v: u32) -> &[u32];

    /// Whether this graph is directed.
    fn is_directed(&self) -> bool;
}

pub use crate::graph::Graph;
