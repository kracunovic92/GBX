use super::config::BuchbergerTraceConfig;
use super::counters::BuchbergerTraceCounters;
use super::memory::current_memory_snapshot;
use super::snapshot::BuchbergerTraceSnapshot;
use super::timings::{measure_duration, PhaseKind, PhaseTimes, WhileKind, WhileTimes};
use crate::trace::TraceHandle;
use std::time::{Duration, Instant};

pub type SharedBuchbergerTracer = TraceHandle<BuchbergerTracer>;

#[derive(Debug)]
pub struct BuchbergerTracer {
    cfg: BuchbergerTraceConfig,
    start: Instant,
    last_progress: Instant,
    counters: BuchbergerTraceCounters,
    phases: PhaseTimes,
    while_times: WhileTimes,
}

impl BuchbergerTracer {
    #[must_use]
    pub fn new(cfg: BuchbergerTraceConfig) -> Self {
        let now = Instant::now();
        Self { cfg, start: now, last_progress: now, counters: BuchbergerTraceCounters::default(), phases: PhaseTimes::default(), while_times: WhileTimes::default() }
    }

    #[must_use]
    pub fn shared(cfg: BuchbergerTraceConfig) -> SharedBuchbergerTracer {
        TraceHandle::new(Self::new(cfg))
    }

    #[must_use]
    pub const fn cfg(&self) -> BuchbergerTraceConfig {
        self.cfg
    }

    #[must_use]
    pub const fn counters(&self) -> &BuchbergerTraceCounters {
        &self.counters
    }

    #[must_use]
    pub const fn phases(&self) -> &PhaseTimes {
        &self.phases
    }

    #[must_use]
    pub const fn while_times(&self) -> &WhileTimes {
        &self.while_times
    }

    #[inline]
    pub fn add_phase_time(&mut self, kind: PhaseKind, dt: Duration) {
        self.phases.add(kind, dt);
    }

    #[inline]
    pub fn add_while_time(&mut self, kind: WhileKind, dt: Duration) {
        self.while_times.add(kind, dt);
    }

    #[inline]
    pub fn on_init_complete(&mut self, initial_basis_len: usize) {
        self.counters.set_initial_basis_len(initial_basis_len);
    }

    #[inline]
    pub fn set_gb_len(&mut self, n: usize) {
        self.counters.set_gb_len(n);
    }

    #[inline]
    pub fn on_push(&mut self, queue_len: usize) {
        self.counters.record_push(queue_len);
    }

    #[inline]
    pub fn on_seeded_pair(&mut self, queue_len: usize) {
        self.counters.record_seeded_pair(queue_len);
    }

    #[inline]
    pub fn on_pop(&mut self, queue_len: usize) {
        self.counters.record_pop(queue_len);
    }

    #[inline]
    pub fn on_inserted_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.counters.record_inserted_poly(gb_len, queue_len);
    }

    #[inline]
    pub fn on_zero_reduction(&mut self) {
        self.counters.record_zero_reduction();
    }

    #[inline]
    pub fn on_unit_reduction(&mut self) {
        self.counters.record_unit_reduction();
    }

    #[inline]
    pub fn on_pair_rejected_by_criterion(&mut self) {
        self.counters.record_pair_rejected_by_criterion();
    }
    #[inline]
    pub fn on_pair_rejected_by_filter(&mut self) {
        self.counters.record_pair_rejected_by_filter();
    }

    #[inline]
    pub fn on_pairs_added_by_update(&mut self, n: usize) {
        self.counters.record_pairs_added_by_update(n);
    }

    #[inline]
    pub fn on_pair_key_missing(&mut self) {
        self.counters.record_pair_key_missing();
    }

    #[must_use]
    pub fn should_emit_progress(&self) -> bool {
        let every = self.cfg.progress_every;
        every != 0 && self.counters.pops as usize % every == 0
    }

    pub fn mark_progress_sample(&mut self) -> (Duration, Duration) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_progress);
        let total = now.duration_since(self.start);
        self.last_progress = now;
        self.counters.record_progress_sample();
        (delta, total)
    }

    #[must_use]
    pub fn elapsed_total(&self) -> Duration {
        self.start.elapsed()
    }

    #[must_use]
    pub fn snapshot(&self, gb_len: usize, queue_len: usize, basis_term_count: Option<usize>, max_poly_terms: Option<usize>, include_memory: bool) -> BuchbergerTraceSnapshot {
        BuchbergerTraceSnapshot { gb_len, queue_len, basis_term_count, max_poly_terms, memory: if include_memory && self.cfg.core.collects_memory() { current_memory_snapshot() } else { None } }
    }

    #[inline]
    pub fn measure_while<T, E, F>(&mut self, kind: WhileKind, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let slot = match kind {
            WhileKind::SPolynomial => &mut self.while_times.s_poly,
            WhileKind::NormalForm => &mut self.while_times.normal_form,
            WhileKind::RemainderNormalize => &mut self.while_times.remainder_normalize,
            WhileKind::PairUpdate => &mut self.while_times.pair_update,
        };
        measure_duration(slot, f)
    }
}
