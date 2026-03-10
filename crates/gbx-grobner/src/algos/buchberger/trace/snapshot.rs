use super::memory::MemorySnapshot;

/// Snapshot of current Buchberger state used for progress and summary output.
#[derive(Debug, Default, Clone, Copy)]
pub struct TraceSnapshot {
    /// Current basis length.
    pub gb_len: usize,

    /// Current queue length.
    pub queue_len: usize,

    /// Total number of terms across the current basis, if sampled.
    pub basis_term_count: Option<usize>,

    /// Maximum number of terms in any current basis polynomial, if sampled.
    pub max_poly_terms: Option<usize>,

    /// Optional process memory snapshot.
    pub memory: Option<MemorySnapshot>,
}
