//! DIMACS parsing for undirected graphs.
//!
//! Supports:
//! - `p edge <n> <m>`
//! - `p col  <n> <m>`
//! - `e <u> <v>`
//! - comment lines starting with `c`

use crate::Graph;
use crate::io::error::DimacsError;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Parse a DIMACS `edge` / `col` graph from any buffered reader into an undirected [`Graph`].
///
/// Strict behavior:
/// - requires exactly one problem line
/// - rejects edges before the problem line
/// - requires parsed edge count == declared `m`
///
/// Unknown non-empty lines are ignored (tolerates minor DIMACS variants).
///
/// # Errors
/// Returns [`DimacsError`] on malformed input or I/O errors.
pub fn read_dimacs<R: BufRead>(reader: R) -> Result<Graph, DimacsError> {
    let mut graph: Option<Graph> = None;
    let mut expected_edges: Option<usize> = None;
    let mut seen_edges: usize = 0;

    for line_res in reader.lines() {
        let raw = line_res?;
        let line = raw.trim();

        if line.is_empty() {
            continue;
        }

        match line.as_bytes()[0] {
            b'p' => {
                if graph.is_some() {
                    return Err(DimacsError::DuplicateProblemLine { line_no: 0 });
                }

                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() != 4 {
                    return Err(DimacsError::InvalidProblemLine { line_no: 0, line: line.to_string() });
                }

                let fmt = parts[1];
                if fmt != "edge" && fmt != "col" {
                    return Err(DimacsError::InvalidProblemLine { line_no: 0, line: line.to_string() });
                }

                let n: usize = parts[2].parse()?;
                let m: usize = parts[3].parse()?;

                graph = Some(Graph::new(n));
                expected_edges = Some(m);
            }

            b'e' => {
                let g = graph
                    .as_mut()
                    .ok_or(DimacsError::EdgeBeforeProblemLine { line_no: 0 })?;

                let parts: Vec<_> = line.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(DimacsError::BadEdgeLine { line_no: 0, line: line.to_string() });
                }

                let u: usize = parts[1].parse()?;
                let v: usize = parts[2].parse()?;

                g.add_edge(u, v)?;
                seen_edges += 1;
            }

            _ => {
                // Ignore comments and unknown lines (DIMACS variants sometimes include extra info).
            }
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
/// Thin wrapper over [`read_dimacs`].
///
/// # Errors
/// Returns [`DimacsError`] if the file cannot be opened, read, or parsed.
pub fn read_dimacs_file(path: impl AsRef<Path>) -> Result<Graph, DimacsError> {
    let file = File::open(path)?;
    read_dimacs(BufReader::new(file))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use crate::io::error::DimacsError;
    use std::io::Cursor;

    #[test]
    fn parse_small_dimacs_edge_graph() {
        let dimacs = b"
        c This is a tiny graph
        p edge 3 2
        e 1 2
        e 2 3
        ";
        let g = read_dimacs(Cursor::new(&dimacs[..])).expect("failed to parse DIMACS");

        assert_eq!(g.n(), 3);
        assert_eq!(g.m(), 2);

        assert_eq!(g.adj()[1], vec![2]);
        assert_eq!(g.adj()[2], vec![1, 3]);
        assert_eq!(g.adj()[3], vec![2]);
    }

    #[test]
    fn parse_small_dimacs_col_graph() {
        let dimacs = b"
        p col 2 1
        e 1 2
        ";
        let g = read_dimacs(Cursor::new(&dimacs[..])).unwrap();
        assert_eq!(g.n(), 2);
        assert_eq!(g.m(), 1);
    }

    #[test]
    fn edge_before_problem_line_is_error() {
        let dimacs = b"e 1 2\np edge 3 1\n";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(
            err,
            DimacsError::EdgeBeforeProblemLine { line_no: _ }
        ));
    }

    #[test]
    fn missing_problem_line_is_error() {
        let dimacs = b"c comment only\n";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(err, DimacsError::MissingProblemLine));
    }

    #[test]
    fn duplicate_problem_line_is_error() {
        let dimacs = b"
        p edge 3 0
        p edge 3 0
        ";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(
            err,
            DimacsError::DuplicateProblemLine { line_no: _ }
        ));
    }

    #[test]
    fn bad_edge_line_is_error() {
        let dimacs = b"
        p edge 3 1
        e 1
        ";
        let err = read_dimacs(Cursor::new(&dimacs[..])).unwrap_err();
        assert!(matches!(err, DimacsError::BadEdgeLine { .. }));
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
