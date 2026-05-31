use crate::GraphLike;
use crate::graph::error::GraphError;
use std::fmt;

/// A simple undirected graph with vertices labeled `1..=n`.
///
/// Storage:
/// - 1-based adjacency list
/// - `adj[0]` is unused
///
/// Notes:
/// - This structure does **not** prevent parallel edges or self-loops.
/// - For strict/simple graphs, add a normalization step (sort + dedup).
#[derive(Debug, Clone)]
pub struct Graph {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    /// Create a graph with `n` vertices and no edges.
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self { n, m: 0, adj: vec![Vec::new(); n + 1] }
    }

    /// Number of vertices.
    #[must_use]
    pub const fn n(&self) -> usize {
        self.n
    }

    /// Number of edges (undirected, counted once).
    #[must_use]
    pub const fn m(&self) -> usize {
        self.m
    }

    /// 1-based adjacency list (index 0 unused).
    #[must_use]
    pub fn adj(&self) -> &[Vec<usize>] {
        &self.adj
    }

    /// Adds an undirected edge `{u, v}`.
    ///
    /// # Errors
    /// Returns [`GraphError::VertexOutOfRange`] if `u` or `v` is outside `1..=n`.
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), GraphError> {
        self.check_vertex(u)?;
        self.check_vertex(v)?;

        self.adj[u].push(v);
        self.adj[v].push(u);
        self.m += 1;
        Ok(())
    }

    /// Returns an iterator over all edges `(u, v)` with `u < v`.
    ///
    /// This avoids listing each undirected edge twice.
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adj.iter().enumerate().skip(1).flat_map(|(u, ns)| {
            ns.iter()
                .copied()
                .filter(move |&v| v > u)
                .map(move |v| (u, v))
        })
    }

    /// Neighbors of vertex `v` (checked).
    ///
    /// # Errors
    /// Returns [`GraphError::VertexOutOfRange`] if `v` is outside `1..=n`.
    pub fn neighbors_checked(&self, v: usize) -> Result<&[usize], GraphError> {
        self.check_vertex(v)?;
        Ok(&self.adj[v])
    }

    #[inline]
    fn neighbors_unchecked(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }

    #[inline]
    fn check_vertex(&self, v: usize) -> Result<(), GraphError> {
        if (1..=self.n).contains(&v) { Ok(()) } else { Err(GraphError::VertexOutOfRange { v, n: self.n }) }
    }

    /// Sorts adjacency lists and removes parallel edges (and duplicates).
    ///
    /// Note: this also removes duplicate self-loops.
    pub fn normalize_simple(&mut self) {
        for v in 1..=self.n {
            self.adj[v].sort_unstable();
            self.adj[v].dedup();
        }

        self.m = self.edges().count();
    }
}

impl GraphLike for Graph {
    fn vertex_count(&self) -> usize {
        self.n
    }

    fn neighbors(&self, v: usize) -> &[usize] {
        self.neighbors_unchecked(v)
    }

    fn is_directed(&self) -> bool {
        false
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph(n={}, m={})", self.n, self.m)?;
        for v in 1..=self.n {
            write!(f, "{v}:")?;
            for &u in &self.adj[v] {
                write!(f, " {u}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn simple_graph_structure() {
        let mut g = Graph::new(3);
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();

        assert_eq!(g.n(), 3);
        assert_eq!(g.m(), 2);

        assert_eq!(g.vertex_count(), 3);
        assert_eq!(g.neighbors_checked(2).unwrap(), &[1, 3]);

        let mut edges: Vec<_> = g.edges().collect();
        edges.sort_unstable();
        assert_eq!(edges, vec![(1, 2), (2, 3)]);
    }

    #[test]
    fn out_of_range_vertex() {
        let mut g = Graph::new(3);

        let err = g.add_edge(0, 1).unwrap_err();
        assert!(matches!(err, GraphError::VertexOutOfRange { v: 0, n: 3 }));

        let err = g.add_edge(1, 4).unwrap_err();
        assert!(matches!(err, GraphError::VertexOutOfRange { v: 4, n: 3 }));

        let err = g.neighbors_checked(9).unwrap_err();
        assert!(matches!(err, GraphError::VertexOutOfRange { v: 9, n: 3 }));
    }

    #[test]
    fn display_format_is_reasonable() {
        let mut g = Graph::new(3);
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();

        let printed = format!("{g}");
        assert!(printed.contains("Graph(n=3, m=2)"));
        assert!(printed.contains("1: 2"));
        assert!(printed.contains("2: 1 3"));
        assert!(printed.contains("3: 2"));
    }
}
