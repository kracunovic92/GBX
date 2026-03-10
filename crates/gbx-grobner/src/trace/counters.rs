#[derive(Debug, Default)]
pub struct TraceCounters {
    pub pops: u64,
    pub pushes: u64,
    pub new_polys: u64,
    pub inserted_polys: u64,
    pub zero_reductions: u64,
    pub unit_reductions: u64,

    /// Number of candidate pairs rejected by the active pair criterion.
    pub pairs_rejected_by_criterion: u64,

    /// Number of candidate pairs skipped because no queue key could be computed.
    pub pairs_skipped_missing_key: u64,

    pub gb_len: usize,
    pub max_queue_len: usize,
}

impl TraceCounters {
    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.gb_len = n;
    }

    #[inline]
    pub fn observe_queue_len(&mut self, len: usize) {
        self.max_queue_len = self.max_queue_len.max(len);
    }

    #[inline]
    pub fn on_push(&mut self, queue_len: usize) {
        self.pushes += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_pop(&mut self, queue_len: usize) {
        self.pops += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_new_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.new_polys += 1;
        self.gb_len = gb_len;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_inserted_poly(&mut self, gb_len: usize) {
        self.inserted_polys += 1;
        self.gb_len = gb_len;
    }

    #[inline]
    pub fn on_zero_reduction(&mut self) {
        self.zero_reductions += 1;
    }

    #[inline]
    pub fn on_unit_reduction(&mut self) {
        self.unit_reductions += 1;
    }

    #[inline]
    pub fn on_pair_rejected_by_criterion(&mut self) {
        self.pairs_rejected_by_criterion += 1;
    }

    #[inline]
    pub fn on_pair_key_missing(&mut self) {
        self.pairs_skipped_missing_key += 1;
    }

    /// Backward-compatible alias for older tracing code.
    ///
    /// Prefer [`Self::on_pair_rejected_by_criterion`] in new code.
    #[inline]
    pub fn on_product_skip(&mut self) {
        self.on_pair_rejected_by_criterion();
    }
}
