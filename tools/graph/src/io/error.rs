use crate::graph::error::GraphError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DimacsError {
    #[error("I/O error while reading DIMACS: {0}")]
    Io(#[from] io::Error),

    // Optional “at line” variants (recommended)
    #[error("I/O error while reading DIMACS at line {line_no}: {source}")]
    IoAtLine { line_no: usize, source: io::Error },

    #[error("failed to parse integer in DIMACS: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("failed to parse integer in DIMACS at line {line_no}: {source}")]
    ParseIntAtLine { line_no: usize, source: std::num::ParseIntError },

    #[error("missing problem line ('p edge n m' or 'p col n m')")]
    MissingProblemLine,

    #[error("duplicate problem line at line {line_no}")]
    DuplicateProblemLine { line_no: usize },

    #[error("invalid problem line at line {line_no}: {line}")]
    InvalidProblemLine { line_no: usize, line: String },

    #[error("edge line appeared before problem line at line {line_no}")]
    EdgeBeforeProblemLine { line_no: usize },

    #[error("invalid edge line at line {line_no}: {line}")]
    BadEdgeLine { line_no: usize, line: String },

    #[error("edge count mismatch: expected {expected}, found {found}")]
    EdgeCountMismatch { expected: usize, found: usize },

    #[error(transparent)]
    Graph(#[from] GraphError),
}
