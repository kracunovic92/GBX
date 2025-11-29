use crate::Graph;
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
}

/// Parse a DIMACS 'edge' / 'col' graph from any buffered reader
/// into an undirected [`Graph`].
///
/// Supported syntax (typical for graph coloring benchmarks):
///
/// - Comment lines:
///   `c this is a comment`
///
/// - Problem line:
///   `p edge <num_vertices> <num_edges>`
///   or
///   `p col  <num_vertices> <num_edges>`
///
/// - Edge lines:
///   `e <u> <v>`
///
/// Vertices are 1-based and must satisfy `1 <= u, v <= n`.
pub fn read_dimacs<R: BufRead>(reader: R) -> Result<Graph, DimacsError> {
    let mut n: Option<usize> = None;
    let mut m: Option<usize> = None;
    let mut edges: Vec<(u32, u32)> = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some(first) = line
            .chars()
            .next()
        {
            match first {
                'c' => {
                    continue;
                }
                'p' => {
                    let parts: Vec<_> = line
                        .split_whitespace()
                        .collect();
                    if parts.len() != 4 {
                        return Err(DimacsError::InvalidProblemLine(line.to_string()));
                    }

                    let fmt = parts[1];
                    if fmt != "edge" && fmt != "col" {
                        return Err(DimacsError::InvalidProblemLine(line.to_string()));
                    }

                    let num_vertices: usize = parts[2].parse()?;
                    let num_edges: usize = parts[3].parse()?;

                    n = Some(num_vertices);
                    m = Some(num_edges);
                }
                'e' => {
                    if n.is_none() || m.is_none() {
                        return Err(DimacsError::EdgeBeforeProblemLine);
                    }

                    let parts: Vec<_> = line
                        .split_whitespace()
                        .collect();
                    if parts.len() != 3 {
                        return Err(DimacsError::BadEdgeLine(line.to_string()));
                    }

                    let u: u32 = parts[1].parse()?;
                    let v: u32 = parts[2].parse()?;

                    let max_v = n.unwrap() as u32;
                    if u == 0 || v == 0 || u > max_v || v > max_v {
                        return Err(DimacsError::VertexOutOfRange(line.to_string()));
                    }

                    edges.push((u, v));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    let n = n.ok_or(DimacsError::MissingProblemLine)?;
    let expected_m = m.unwrap();

    if edges.len() != expected_m {
        return Err(DimacsError::EdgeCountMismatch { expected: expected_m, found: edges.len() });
    }

    let mut graph = Graph::new(n);

    for (u, v) in edges {
        graph
            .add_edge(u, v)
            .expect("validated vertex indices");
    }

    Ok(graph)
}

/// Convenience helper to read a DIMACS file from a given path into a [`Graph`].
///
/// This is a thin wrapper over [`read_dimacs`].
pub fn read_dimacs_file(path: impl AsRef<Path>) -> Result<Graph, DimacsError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    read_dimacs(reader)
}

#[cfg(test)]
mod tests {
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
}
