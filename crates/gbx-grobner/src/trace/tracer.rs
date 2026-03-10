use crate::trace::conf::TraceCfg;
use crate::trace::counters::TraceCounters;
use crate::trace::timings::{PhaseTimes, WhileTimes};
use crate::PairQueue;
use std::time::Instant;

#[derive(Debug)]
pub struct Tracer {
    cfg: TraceCfg,
    start: Instant,
    last_progress: Instant,

    pub counters: TraceCounters,
    pub phases: PhaseTimes,
    pub while_times: WhileTimes,
}

impl Tracer {
    #[must_use]
    pub fn new(cfg: TraceCfg) -> Self {
        let now = Instant::now();
        Self { cfg, start: now, last_progress: now, counters: TraceCounters::default(), phases: PhaseTimes::default(), while_times: WhileTimes::default() }
    }

    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.counters.set_gb_len(n);
    }

    #[inline]
    pub fn on_push_with_len(&mut self, queue_len: usize) {
        self.counters.on_push(queue_len);
    }

    #[inline]
    pub fn on_pop<Q: PairQueue>(&mut self, q: &Q) {
        self.counters.on_pop(q.len());

        let every = self.cfg.progress_every;
        if every != 0 && (self.counters.pops as usize % every == 0) {
            let now = Instant::now();
            let dt = now.duration_since(self.last_progress);
            let total = now.duration_since(self.start);
            self.last_progress = now;

            eprintln!(
                "[trace] pops={} gb_len={} queue_len={} max_queue_len={} pushes={} new_polys={} inserted={} zero_reductions={} rejected_by_criterion={} skipped_missing_key={} unit_reductions={} Δt={:?} total={:?}",
                self.counters.pops,
                self.counters.gb_len,
                q.len(),
                self.counters.max_queue_len,
                self.counters.pushes,
                self.counters.new_polys,
                self.counters.inserted_polys,
                self.counters.zero_reductions,
                self.counters.pairs_rejected_by_criterion,
                self.counters.pairs_skipped_missing_key,
                self.counters.unit_reductions,
                dt,
                total
            );
        }
    }

    #[inline]
    pub fn on_new_poly<Q: PairQueue>(&mut self, new_idx: usize, gb_len: usize, q: &Q) {
        self.counters.on_new_poly(gb_len, q.len());

        if self.cfg.on_new_poly {
            let total = Instant::now().duration_since(self.start);
            eprintln!(
                "[trace] +poly idx={} gb_len={} queue_len={} max_queue_len={} total={:?}",
                new_idx,
                gb_len,
                q.len(),
                self.counters.max_queue_len,
                total
            );
        }
    }

    #[inline]
    pub fn on_inserted_poly(&mut self, gb_len: usize) {
        self.counters.on_inserted_poly(gb_len);
    }

    #[inline]
    pub fn on_zero_reduction(&mut self) {
        self.counters.on_zero_reduction();
    }

    #[inline]
    pub fn on_unit_reduction(&mut self) {
        self.counters.on_unit_reduction();
    }

    /// Records that a candidate pair was rejected by the active pair criterion.
    #[inline]
    pub fn on_pair_rejected_by_criterion(&mut self) {
        self.counters.on_pair_rejected_by_criterion();
    }

    /// Records that a candidate pair was skipped because no queue key
    /// could be computed for it.
    #[inline]
    pub fn on_pair_key_missing(&mut self) {
        self.counters.on_pair_key_missing();
    }

    /// Backward-compatible alias for older tracing code.
    ///
    /// Prefer [`Self::on_pair_rejected_by_criterion`] in new code.
    #[inline]
    pub fn on_product_skip(&mut self) {
        self.on_pair_rejected_by_criterion();
    }

    pub fn print_summary(&self) {
        if self.cfg.phases {
            eprintln!(
                "[trace] phases: init={}ms seed={}ms while={}ms post={}ms",
                self.phases.init_ms, self.phases.seed_ms, self.phases.while_ms, self.phases.post_ms
            );
        }

        if self.cfg.breakdown {
            eprintln!(
                "[trace] while breakdown: s_poly={}ms normal_form={}ms rem_norm={}ms",
                self.while_times.s_poly_ms, self.while_times.nf_ms, self.while_times.rem_norm_ms
            );
        }

        eprintln!(
            "[trace] totals: pops={} pushes={} new_polys={} inserted_polys={} zero_reductions={} unit_reductions={} rejected_by_criterion={} skipped_missing_key={} final_gb_len={} max_queue_len={}",
            self.counters.pops,
            self.counters.pushes,
            self.counters.new_polys,
            self.counters.inserted_polys,
            self.counters.zero_reductions,
            self.counters.unit_reductions,
            self.counters.pairs_rejected_by_criterion,
            self.counters.pairs_skipped_missing_key,
            self.counters.gb_len,
            self.counters.max_queue_len
        );
    }
}
