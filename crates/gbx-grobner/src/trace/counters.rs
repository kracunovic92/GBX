/// Reusable mechanical trace counters shared across Gröbner algorithms.
///
/// These counters track generic queue/pairing/basis mechanics rather than
/// algorithm-specific semantic events.
#[derive(Debug, Default, Clone, Copy)]
pub struct MechanicalTraceCounters {
    /// Number of successful queue pops.
    pub pops: u64,

    /// Number of queue pushes.
    pub pushes: u64,

    /// Number of seeded pairs inserted into the queue.
    pub seeded_pairs: u64,

    /// Number of pairs rejected by a local criterion.
    pub pairs_rejected_by_criterion: u64,

    /// Number of pairs rejected by a state-aware filter.
    pub pairs_rejected_by_filter: u64,

    /// Number of pairs skipped because no queue key could be produced.
    pub pairs_skipped_missing_key: u64,

    /// Net number of pairs added by pair-update passes.
    pub pairs_added_by_update: u64,

    /// Initial basis length after preprocessing/normalization.
    pub initial_basis_len: usize,

    /// Current or final basis length.
    pub gb_len: usize,

    /// Maximum observed queue length.
    pub max_queue_len: usize,
}

impl MechanicalTraceCounters {
    #[inline]
    pub fn set_initial_basis_len(&mut self, n: usize) {
        self.initial_basis_len = n;
        self.gb_len = n;
    }

    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.gb_len = n;
    }

    #[inline]
    pub fn observe_queue_len(&mut self, len: usize) {
        self.max_queue_len = self.max_queue_len.max(len);
    }

    #[inline]
    pub fn record_push(&mut self, queue_len: usize) {
        self.pushes += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_pop(&mut self, queue_len: usize) {
        self.pops += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_seeded_pair(&mut self, queue_len: usize) {
        self.seeded_pairs += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_pair_rejected_by_criterion(&mut self) {
        self.pairs_rejected_by_criterion += 1;
    }

    #[inline]
    pub fn record_pair_rejected_by_filter(&mut self) {
        self.pairs_rejected_by_filter += 1;
    }

    #[inline]
    pub fn record_pair_key_missing(&mut self) {
        self.pairs_skipped_missing_key += 1;
    }

    #[inline]
    pub fn record_pairs_added_by_update(&mut self, n: usize) {
        self.pairs_added_by_update += n as u64;
    }
}
