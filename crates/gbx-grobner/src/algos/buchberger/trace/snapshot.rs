use crate::MemorySnapshot;

/// Immutable Buchberger trace snapshot used for progress and summary reporting.
#[derive(Debug, Default, Clone, Copy)]
pub struct BuchbergerTraceSnapshot {
    pub gb_len: usize,
    pub queue_len: usize,
    pub basis_term_count: Option<usize>,
    pub max_poly_terms: Option<usize>,
    pub memory: Option<MemorySnapshot>,
}
