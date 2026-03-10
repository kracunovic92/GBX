use super::config::TraceCfg;
use super::counters::TraceCounters;
use super::memory::current_memory_snapshot;
use super::reporter::{print_progress_line, print_summary};
use super::snapshot::TraceSnapshot;
use super::timings::{PhaseTimes, WhileTimes};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

/// Shared Buchberger tracer handle.
pub type SharedTracer = Rc<RefCell<Tracer>>;

/// Buchberger tracing state.
///
/// This owns counters, timings, progress scheduling, and optional memory
/// sampling. Mechanical events may be reported through wrappers; semantic
/// algorithm events should be emitted directly by the engine.
#[derive(Debug)]
pub struct Tracer {
    pub(crate) cfg: TraceCfg,
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

    #[must_use]
    pub fn shared(cfg: TraceCfg) -> SharedTracer {
        Rc::new(RefCell::new(Self::new(cfg)))
    }

    #[inline]
    pub fn set_initial_basis_len(&mut self, n: usize) {
        self.counters.set_initial_basis_len(n);
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
    pub fn on_seeded_pair(&mut self, queue_len: usize) {
        self.counters.on_seeded_pair(queue_len);
    }

    #[inline]
    pub fn on_pop_with_len(&mut self, queue_len: usize) {
        self.counters.on_pop(queue_len);
    }

    #[inline]
    pub fn on_inserted_poly(&mut self, gb_len: usize, queue_len: usize) {
        self.counters.on_inserted_poly(gb_len, queue_len);

        if self.cfg.print_on_insert {
            let total = self.start.elapsed();
            eprintln!(
                "[trace] +poly gb_len={} queue_len={} max_queue_len={} total={:?}",
                gb_len, queue_len, self.counters.max_queue_len, total
            );
        }
    }

    #[inline]
    pub fn on_zero_reduction(&mut self) {
        self.counters.on_zero_reduction();
    }

    #[inline]
    pub fn on_unit_reduction(&mut self) {
        self.counters.on_unit_reduction();
    }

    #[inline]
    pub fn on_pair_rejected_by_criterion(&mut self) {
        self.counters.on_pair_rejected_by_criterion();
    }

    #[inline]
    pub fn on_pair_key_missing(&mut self) {
        self.counters.on_pair_key_missing();
    }

    #[must_use]
    pub fn make_snapshot(&self, gb_len: usize, queue_len: usize, basis_term_count: Option<usize>, max_poly_terms: Option<usize>, include_memory: bool) -> TraceSnapshot {
        TraceSnapshot { gb_len, queue_len, basis_term_count, max_poly_terms, memory: if include_memory && self.cfg.collect_memory { current_memory_snapshot() } else { None } }
    }

    pub fn maybe_print_progress(&mut self, snap: &TraceSnapshot) {
        let every = self.cfg.progress_every;
        if every == 0 || self.counters.pops as usize % every != 0 {
            return;
        }

        let now = Instant::now();
        let delta = now.duration_since(self.last_progress);
        let total = now.duration_since(self.start);
        self.last_progress = now;
        self.counters.on_progress_sample();

        print_progress_line(self, snap, delta, total);
    }

    pub fn print_summary(&self, snap: &TraceSnapshot) {
        print_summary(self, snap);
    }

    pub fn print_iteration(
        &self,
        i: usize,
        j: usize,
        gb_len: usize,
        queue_len: usize,
        s_poly_dt: std::time::Duration,
        nf_dt: std::time::Duration,
        rem_norm_dt: std::time::Duration,
        pair_update_dt: std::time::Duration,
        outcome: &str,
    ) {
        if !self.cfg.print_each_iteration {
            return;
        }

        if self.cfg.print_iteration_timings {
            eprintln!(
                "[trace] iter pop={} pair=({}, {}) gb_len={} queue_len={} outcome={} s_poly={:?} nf={:?} rem_norm={:?} pair_update={:?}",
                self.counters.pops, i, j, gb_len, queue_len, outcome, s_poly_dt, nf_dt, rem_norm_dt, pair_update_dt,
            );
        } else {
            eprintln!(
                "[trace] iter pop={} pair=({}, {}) gb_len={} queue_len={} outcome={}",
                self.counters.pops, i, j, gb_len, queue_len, outcome,
            );
        }
    }
}
