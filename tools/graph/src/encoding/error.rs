use thiserror::Error;

/// Errors that can occur while building encodings.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum EncodingError {
    /// Number of colors must be at least 1.
    #[error("invalid number of colors k={k}; expected k >= 1")]
    InvalidK { k: usize },
}
