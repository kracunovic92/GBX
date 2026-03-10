#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum GraphError {
    /// A vertex index was outside the valid range `1..=n`.
    #[error("vertex out of range: v={v}, expected 1..={n}")]
    VertexOutOfRange { v: usize, n: usize },
}
