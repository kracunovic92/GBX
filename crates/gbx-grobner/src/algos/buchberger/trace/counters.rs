use crate::trace::MechanicalTraceCounters;

/// Counters for Buchberger tracing.
///
/// These counters combine reusable mechanical events with Buchberger-specific
/// semantic outcomes.
#[derive(Debug, Default, Clone, Copy)]
pub struct BuchbergerTraceCounters {
    /// Reusable queue/pairing/basis mechanics.
    pub mech: MechanicalTraceCounters,

    /// Number of nonzero remainders inserted into the basis.
    pub inserted_polys: u64,

    /// Number of zero reductions.
    pub zero_reductions: u64,

    /// Number of unit reductions.
    pub unit_reductions: u64,

    /// Number of emitted progress samples.
    pub progress_samples: u64,
}

impl BuchbergerTraceCounters {
    #[inline]
    pub fn set_initial_basis_len(&mut self, n: usize) {
        self.mech.set_initial_basis_len(n);
    }

    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.mech.set_gb_len(n);
    }

    #[inline]
    pub fn record_push(&mut self, queue_len: usize) {
        self.mech.record_push(queue_len);
    }

    #[inline]
    pub fn record_seeded_pair(&mut self, queue_len: usize) {
        self.mech.record_seeded_pair(queue_len);
    }

    #[inline]
    pub fn record_pop(&mut self, queue_len: usize) {
        self.mech.record_pop(queue_len);
    }

    #[inline]
    pub fn record_inserted_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.inserted_polys += 1;
        self.mech.set_gb_len(gb_len);
        self.mech.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_zero_reduction(&mut self) {
        self.zero_reductions += 1;
    }

    #[inline]
    pub fn record_unit_reduction(&mut self) {
        self.unit_reductions += 1;
    }

    #[inline]
    pub fn record_pair_rejected_by_criterion(&mut self) {
        self.mech.record_pair_rejected_by_criterion();
    }

    #[inline]
    pub fn record_pair_rejected_by_filter(&mut self) {
        self.mech.record_pair_rejected_by_filter();
    }

    #[inline]
    pub fn record_pair_key_missing(&mut self) {
        self.mech.record_pair_key_missing();
    }

    #[inline]
    pub fn record_pairs_added_by_update(&mut self, n: usize) {
        self.mech.record_pairs_added_by_update(n);
    }

    #[inline]
    pub fn record_progress_sample(&mut self) {
        self.progress_samples += 1;
    }
}
