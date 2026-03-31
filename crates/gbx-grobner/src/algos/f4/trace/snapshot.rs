use crate::trace::MemorySnapshot;

/// Immutable F4 trace snapshot used for progress and summary reporting.
#[derive(Debug, Default, Clone, Copy)]
pub struct F4TraceSnapshot {
    pub gb_len: usize,
    pub queue_len: usize,
    pub memory: Option<MemorySnapshot>,
}
