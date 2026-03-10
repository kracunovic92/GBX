/// Counters for Buchberger tracing.
///
/// These counters are intended to represent semantic algorithm events.
/// In particular:
///
/// - `inserted_polys` means actual nonzero remainders appended to the working
///   basis during Buchberger's main loop,
/// - `zero_reductions` means S-polynomials whose normal form was zero,
/// - `unit_reductions` means nonzero constant remainders that terminate early.
#[derive(Debug, Default, Clone)]
pub struct TraceCounters {
    /// Number of successful queue pops.
    pub pops: u64,

    /// Number of queue pushes.
    pub pushes: u64,

    /// Number of seeded initial pairs.
    pub seeded_pairs: u64,

    /// Number of nonzero remainders inserted into the working basis.
    pub inserted_polys: u64,

    /// Number of remainders equal to zero after normal form.
    pub zero_reductions: u64,

    /// Number of nonzero constant remainders.
    pub unit_reductions: u64,

    /// Number of candidate pairs rejected by the active pair criterion.
    pub pairs_rejected_by_criterion: u64,

    /// Number of candidate pairs skipped because no key could be computed.
    pub pairs_skipped_missing_key: u64,

    /// Initial working basis length after input preprocessing.
    pub initial_basis_len: usize,

    /// Current or final working basis length.
    pub gb_len: usize,

    /// Maximum observed queue length.
    pub max_queue_len: usize,

    /// Number of periodic progress samples printed.
    pub progress_samples: u64,
}

impl TraceCounters {
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
    pub fn on_push(&mut self, queue_len: usize) {
        self.pushes += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_seeded_pair(&mut self, queue_len: usize) {
        self.seeded_pairs += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_pop(&mut self, queue_len: usize) {
        self.pops += 1;
        self.observe_queue_len(queue_len);
    }

    #[inline]
    pub fn on_inserted_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.inserted_polys += 1;
        self.gb_len = gb_len;
        self.observe_queue_len(queue_len);
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

    #[inline]
    pub fn on_progress_sample(&mut self) {
        self.progress_samples += 1;
    }
}
