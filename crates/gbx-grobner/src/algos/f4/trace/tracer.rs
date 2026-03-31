use super::config::F4TraceConfig;
use super::counters::F4TraceCounters;
use super::snapshot::F4TraceSnapshot;
use super::timings::{WhileKind, WhileTimes};
use crate::trace::{current_memory_snapshot, measure_duration, BasisTrace, CorePhaseKind, CorePhaseTimes, PairingTrace, QueueTrace, TraceHandle};
use std::time::{Duration, Instant};

pub type SharedF4Tracer = TraceHandle<F4Tracer>;

#[derive(Debug)]
pub struct F4Tracer {
    cfg: F4TraceConfig,
    start: Instant,
    last_progress: Instant,
    counters: F4TraceCounters,
    phases: CorePhaseTimes,
    while_times: WhileTimes,
}

impl F4Tracer {
    #[must_use]
    pub fn new(cfg: F4TraceConfig) -> Self {
        let now = Instant::now();
        Self { cfg, start: now, last_progress: now, counters: F4TraceCounters::default(), phases: CorePhaseTimes::default(), while_times: WhileTimes::default() }
    }

    #[must_use]
    pub fn shared(cfg: F4TraceConfig) -> SharedF4Tracer {
        TraceHandle::new(Self::new(cfg))
    }

    #[must_use]
    pub const fn cfg(&self) -> F4TraceConfig {
        self.cfg
    }

    #[must_use]
    pub const fn counters(&self) -> &F4TraceCounters {
        &self.counters
    }

    #[must_use]
    pub const fn phases(&self) -> &CorePhaseTimes {
        &self.phases
    }

    #[must_use]
    pub const fn while_times(&self) -> &WhileTimes {
        &self.while_times
    }

    #[inline]
    pub fn add_phase_time(&mut self, kind: CorePhaseKind, dt: Duration) {
        self.phases.add(kind, dt);
    }

    #[inline]
    pub fn add_while_time(&mut self, kind: WhileKind, dt: Duration) {
        self.while_times.add(kind, dt);
    }

    #[inline]
    pub fn on_batch_selected(&mut self, batch_len: usize) {
        self.counters.record_batch_selected(batch_len);
    }

    #[inline]
    pub fn on_seed_rows(&mut self, count: usize) {
        self.counters.record_seed_rows(count);
    }

    #[inline]
    pub fn on_symbolic(&mut self, reducer_rows: usize, total_rows: usize) {
        self.counters.record_symbolic(reducer_rows, total_rows);
    }

    #[inline]
    pub fn on_matrix_shape(&mut self, rows: usize, cols: usize) {
        self.counters.record_matrix_shape(rows, cols);
    }

    #[inline]
    pub fn on_extracted(&mut self, count: usize) {
        self.counters.record_extracted(count);
    }

    #[inline]
    pub fn on_skipped_zero_extracted(&mut self) {
        self.counters.record_skipped_zero_extracted();
    }

    #[must_use]
    pub fn should_emit_progress(&self) -> bool {
        let every = self.cfg.progress_every;
        every != 0 && self.counters.selected_batches as usize % every == 0
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
    pub fn snapshot(&self, gb_len: usize, queue_len: usize, include_memory: bool) -> F4TraceSnapshot {
        F4TraceSnapshot { gb_len, queue_len, memory: if include_memory && self.cfg.core.collects_memory() { current_memory_snapshot() } else { None } }
    }

    #[inline]
    pub fn measure_while<T, E, F>(&mut self, kind: WhileKind, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        let slot = match kind {
            WhileKind::BatchSelect => &mut self.while_times.batch_select,
            WhileKind::SeedRows => &mut self.while_times.seed_rows,
            WhileKind::Symbolic => &mut self.while_times.symbolic,
            WhileKind::MatrixBuild => &mut self.while_times.matrix_build,
            WhileKind::RowReduce => &mut self.while_times.row_reduce,
            WhileKind::Extract => &mut self.while_times.extract,
            WhileKind::PairUpdate => &mut self.while_times.pair_update,
        };

        measure_duration(slot, f)
    }
}

impl QueueTrace for F4Tracer {
    #[inline]
    fn on_push(&mut self, queue_len: usize) {
        self.counters.record_push(queue_len);
    }

    #[inline]
    fn on_pop(&mut self, queue_len: usize) {
        self.counters.record_pop(queue_len);
    }
}

impl PairingTrace for F4Tracer {
    #[inline]
    fn on_seeded_pair(&mut self, queue_len: usize) {
        self.counters.record_seeded_pair(queue_len);
    }

    #[inline]
    fn on_pair_rejected_by_criterion(&mut self) {
        self.counters.record_pair_rejected_by_criterion();
    }

    #[inline]
    fn on_pair_rejected_by_filter(&mut self) {
        self.counters.record_pair_rejected_by_filter();
    }

    #[inline]
    fn on_pair_key_missing(&mut self) {
        self.counters.record_pair_key_missing();
    }

    #[inline]
    fn on_pairs_added_by_update(&mut self, n: usize) {
        self.counters.record_pairs_added_by_update(n);
    }
}

impl BasisTrace for F4Tracer {
    #[inline]
    fn on_init_complete(&mut self, initial_basis_len: usize) {
        self.counters.set_initial_basis_len(initial_basis_len);
    }

    #[inline]
    fn set_gb_len(&mut self, n: usize) {
        self.counters.set_gb_len(n);
    }

    #[inline]
    fn on_inserted_poly(&mut self, gb_len: usize, _queue_len: usize) {
        self.counters.record_inserted_poly(gb_len);
    }
}
