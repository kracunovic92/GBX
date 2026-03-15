/// Counters for Buchberger tracing.
///
/// These counters represent semantic or traced mechanical algorithm events.
#[derive(Debug, Default, Clone, Copy)]
pub struct BuchbergerTraceCounters {
    pub pops: u64,
    pub pushes: u64,
    pub seeded_pairs: u64,
    pub inserted_polys: u64,
    pub zero_reductions: u64,
    pub unit_reductions: u64,
    pub pairs_rejected_by_criterion: u64,
    pub pairs_skipped_missing_key: u64,
    pub initial_basis_len: usize,
    pub gb_len: usize,
    pub max_queue_len: usize,
    pub progress_samples: u64,
    pub pairs_rejected_by_filter: u64,
    pub pairs_added_by_update: u64,
}

impl BuchbergerTraceCounters {
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
    pub fn record_seeded_pair(&mut self, queue_len: usize) {
        self.seeded_pairs += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_pop(&mut self, queue_len: usize) {
        self.pops += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn record_inserted_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.inserted_polys += 1;
        self.gb_len = gb_len;
        self.observe_queue_len(queue_len);
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
        self.pairs_rejected_by_criterion += 1;
    }

    #[inline]
    pub fn record_pair_key_missing(&mut self) {
        self.pairs_skipped_missing_key += 1;
    }

    #[inline]
    pub fn record_progress_sample(&mut self) {
        self.progress_samples += 1;
    }

    #[inline]
    pub fn record_pair_rejected_by_filter(&mut self) {
        self.pairs_rejected_by_filter += 1;
    }

    #[inline]
    pub fn record_pairs_added_by_update(&mut self, n: usize) {
        self.pairs_added_by_update += n as u64;
    }
}
