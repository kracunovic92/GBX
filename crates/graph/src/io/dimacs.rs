use crate::Graph;
use crate::graph::GraphError;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur while parsing a DIMACS graph file.
#[derive(Debug, Error)]
pub enum DimacsError {
    /// I/O error while reading the DIMACS stream.
    #[error("I/O error while reading DIMACS: {0}")]
    Io(#[from] io::Error),

    /// Failed to parse an integer where one was expected.
    #[error("failed to parse integer in DIMACS file: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    /// The file did not contain a problem line (`p edge n m` or `p col n m`).
    #[error("missing problem line ('p edge n m' or 'p col n m')")]
    MissingProblemLine,

    /// The problem line had an unexpected format.
    #[error("invalid problem line: {0}")]
    InvalidProblemLine(String),

    /// An edge line appeared before the problem line.
    #[error("edge line appeared before problem line")]
    EdgeBeforeProblemLine,

    /// An edge line had an unexpected number of tokens.
    #[error("invalid edge line: {0}")]
    BadEdgeLine(String),

    /// A vertex index was outside the declared range 1..=n.
    #[error("vertex index out of range in line: {0}")]
    VertexOutOfRange(String),

    /// The actual number of edges did not match the `m` declared in the problem line.
    #[error("edge count mismatch: expected {expected}, found {found}")]
    EdgeCountMismatch {
        /// Expected input
        expected: usize,
        /// What we actually found
        found: usize,
    },
    /// Add Graph error
    #[error(transparent)]
    Graph(#[from] GraphError),
}

/// Parse a DIMACS `edge` / `col` graph from any buffered reader
/// into an undirected [`Graph`].
///
/// Supported syntax:
///
/// - Comment lines:
///   `c this is a comment`
///
/// - Problem line (must appear exactly once):
///   `p edge <num_vertices> <num_edges>`
///   or
///   `p col  <num_vertices> <num_edges>`
///
/// - Edge lines:
///   `e <u> <v>`
///
/// Vertices are 1-based and must satisfy `1 <= u, v <= n`.
///
/// # Errors
///
/// This function is **strict**:
/// - Edge lines before the problem line are rejected.
/// - The number of parsed edges must exactly match `m`.
/// - Vertex indices must be within range.
/// # Panics
pub fn read_dimacs<R: BufRead>(reader: R) -> Result<Graph, DimacsError> {
    let mut graph: Option<Graph> = None;
    let mut expected_edges: Option<usize> = None;
    let mut seen_edges: usize = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match line.as_bytes()[0] {
            b'p' => {
                if graph.is_some() {
                    return Err(DimacsError::MissingProblemLine);
                }

                let parts: Vec<_> = line.split_whitespace().collect();

                if parts.len() != 4 {
                    return Err(DimacsError::InvalidProblemLine(line.to_string()));
                }

                let format = parts[1];
                if format != "edge" && format != "col" {
                    return Err(DimacsError::InvalidProblemLine(line.to_string()));
                }

                let n: usize = parts[2].parse()?;
                let m: usize = parts[3].parse()?;

                graph = Some(Graph::new(n));
                expected_edges = Some(m);
            }

            b'e' => {
                let g = graph.as_mut().ok_or(DimacsError::EdgeBeforeProblemLine)?;

                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(DimacsError::BadEdgeLine(line.to_string()));
                }

                let u: usize = parts[1].parse()?;
                let v: usize = parts[2].parse()?;

                let max_v = g.n;
                if u == 0 || v == 0 || u > max_v || v > max_v {
                    return Err(DimacsError::VertexOutOfRange(line.to_string()));
                }

                g.add_edge(u, v)?;
                seen_edges += 1;
            }
            _ => {}
        }
    }

    let expected = expected_edges.ok_or(DimacsError::MissingProblemLine)?;

    if seen_edges != expected {
        return Err(DimacsError::EdgeCountMismatch { expected, found: seen_edges });
    }

    graph.ok_or(DimacsError::MissingProblemLine)
}

/// Convenience helper to read a DIMACS file from a given path into a [`Graph`].
///
/// This is a thin wrapper over [`read_dimacs`].
/// # Errors
pub fn read_dimacs_file(path: impl AsRef<Path>) -> Result<Graph, DimacsError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    read_dimacs(reader)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_small_dimacs_graph() {
        let dimacs = b"
        c This is a tiny graph
        p edge 3 2
        e 1 2
        e 2 3
        ";

        let cursor = Cursor::new(&dimacs[..]);
        let g = read_dimacs(cursor).expect("failed to parse DIMACS");

        assert_eq!(g.n, 3);
        assert_eq!(g.m, 2);

        assert_eq!(g.adj[1], vec![2]);
        assert_eq!(g.adj[2], vec![1, 3]);
        assert_eq!(g.adj[3], vec![2]);
    }

    #[test]
    fn missing_problem_line_is_error() {
        let dimacs = b"e 1 2\n";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(err, DimacsError::EdgeBeforeProblemLine));
    }

    #[test]
    fn edge_before_problem_line_is_error() {
        let dimacs = b"e 1 2\np edge 3 1\n";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(err, DimacsError::EdgeBeforeProblemLine));
    }

    #[test]
    fn edge_count_mismatch_is_error() {
        let dimacs = b"
        p edge 3 2
        e 1 2
        ";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(
            err,
            DimacsError::EdgeCountMismatch { expected: 2, found: 1 }
        ));
    }
}
