use crate::GraphLike;
use std::fmt;

/// A simple undirected graph with vertices labeled 1..=n.
///
/// Internally we use a 1-based adjacency list:
/// - 'adj\[0]' is unused
/// - 'adj\[v]' (for 1 <= v <= n) contains all neighbors of vertex 'v'.
#[derive(Debug, Clone)]
pub struct Graph {
    /// Number of vertices.
    pub n: usize,
    /// Number of edges.
    pub m: usize,

    /// Adjacency lists for each vertex.
    ///
    /// The length of this vector is 'n+1' and index 0 is always unused.
    pub adj: Vec<Vec<usize>>,
}

/// Errors that can occur when working with a `Graph`.
#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum GraphError {
    /// A vertex index was outside the valid range `1..=n`.
    #[error("vertex out of range: {0}")]
    VertexOutOfRange(usize),
}

impl Graph {
    /// Creates a graph with 'n' vertices and no edges.
    #[must_use]
    pub fn new(n: usize) -> Self {
        // Allocate adjacency list with n+1 empty vectors.
        // Index 0 is unused.
        let adj = vec![Vec::new(); n + 1];
        Self { n, m: 0, adj }
    }

    /// Adds an undirected edge {u, v}
    ///
    /// Returns an error if either vertex is outside 1..=n.
    /// # Errors
    pub fn add_edge(&mut self, u: usize, v: usize) -> Result<(), GraphError> {
        if u == 0 || (u) > self.n {
            return Err(GraphError::VertexOutOfRange(u));
        }
        if v == 0 || (v) > self.n {
            return Err(GraphError::VertexOutOfRange(v));
        }

        self.adj[u].push(v);
        self.adj[v].push(u);

        self.m += 1;

        Ok(())
    }

    /// Returns an iterator over all edges '(u,v)' with 'u < v'.
    ///
    /// This avoids listing each undirected edge twice
    pub fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adj
            .iter()
            .enumerate()
            .skip(1)
            .flat_map(|(u, neighbors)| {
                neighbors
                    .iter()
                    .copied()
                    .filter(move |&v| v > u)
                    .map(move |v| (u, v))
            })
    }

    /// Convenience inherent method - internal use mainly.
    fn neighbors(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }
}

impl GraphLike for Graph {
    fn vertex_count(&self) -> usize {
        self.n
    }
    fn neighbors(&self, v: usize) -> &[usize] {
        Self::neighbors(self, v)
    }
    fn is_directed(&self) -> bool {
        false
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph(n = {}, m = {})", self.n, self.m)?;

        for v in 1..=self.n {
            write!(f, "{v}: ")?;

            for &u in &self.adj[v] {
                write!(f, "{u} ")?;
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
    use crate::GraphLike;

    #[test]
    fn simple_graph_structure() {
        let mut g = Graph::new(3);
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();

        assert_eq!(g.n, 3);
        assert_eq!(g.m, 2);

        assert_eq!(g.vertex_count(), 3);
        assert_eq!(g.neighbors(2), &[1, 3]);

        let mut edges: Vec<_> = g.edges().collect();
        edges.sort_unstable();
        assert_eq!(edges, vec![(1, 2), (2, 3)]);
    }

    #[test]
    fn out_of_range_vertex() {
        let mut g = Graph::new(3);
        let err = g.add_edge(0, 1).unwrap_err();
        assert!(matches!(err, GraphError::VertexOutOfRange(0)));

        let err = g.add_edge(1, 4).unwrap_err();
        assert!(matches!(err, GraphError::VertexOutOfRange(4)));
    }

    #[test]
    fn display_format_is_reasonable() {
        let mut g = Graph::new(3);
        g.add_edge(1, 2).unwrap();
        g.add_edge(2, 3).unwrap();

        let printed = format!("{g}");

        println!("ACTUAL OUTPUT:\n{printed}");
        assert!(printed.contains("Graph(n = 3, m = 2)"));
        assert!(printed.contains("1: 2"));
        assert!(printed.contains("2: 1 3"));
        assert!(printed.contains("3: 2"));
    }
}
