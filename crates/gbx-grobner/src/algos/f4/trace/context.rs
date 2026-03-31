use super::{
    reporter::{print_progress_line, print_summary},
    snapshot::F4TraceSnapshot,
    timings::WhileKind,
    tracer::SharedF4Tracer,
};
use crate::trace::{BasisTrace, CorePhaseKind};
use std::time::Duration;

/// Thin facade around an optional shared tracer.
///
/// This removes `Option` and locking noise from the F4 engine.
#[derive(Clone)]
pub struct F4TraceCtx {
    tracer: Option<SharedF4Tracer>,
}

impl F4TraceCtx {
    #[must_use]
    pub fn new(tracer: Option<SharedF4Tracer>) -> Self {
        Self { tracer }
    }

    #[must_use]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.tracer.is_some()
    }

    #[inline]
    pub fn on_init_complete(&self, initial_basis_len: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_init_complete(initial_basis_len);
        }
    }

    #[inline]
    pub fn set_gb_len(&self, n: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().set_gb_len(n);
        }
    }

    #[inline]
    pub fn add_phase_time(&self, kind: CorePhaseKind, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_phase_time(kind, dt);
        }
    }

    #[inline]
    pub fn add_while_time(&self, kind: WhileKind, dt: Duration) {
        if let Some(tr) = &self.tracer {
            tr.lock().add_while_time(kind, dt);
        }
    }

    #[inline]
    pub fn on_batch_selected(&self, batch_len: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_batch_selected(batch_len);
        }
    }

    #[inline]
    pub fn on_seed_rows(&self, count: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_seed_rows(count);
        }
    }

    #[inline]
    pub fn on_symbolic(&self, reducer_rows: usize, total_rows: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_symbolic(reducer_rows, total_rows);
        }
    }

    #[inline]
    pub fn on_matrix_shape(&self, rows: usize, cols: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_matrix_shape(rows, cols);
        }
    }

    #[inline]
    pub fn on_extracted(&self, count: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_extracted(count);
        }
    }

    #[inline]
    pub fn on_skipped_zero_extracted(&self) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_skipped_zero_extracted();
        }
    }

    #[inline]
    pub fn on_inserted_poly(&self, gb_len: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_inserted_poly(gb_len, 0);
        }
    }

    #[must_use]
    #[inline]
    pub fn snapshot(&self, gb_len: usize, queue_len: usize) -> Option<F4TraceSnapshot> {
        self.tracer.as_ref().map(|tr| {
            let tr = tr.lock();
            tr.snapshot(gb_len, queue_len, true)
        })
    }

    pub fn maybe_emit_progress(&self, gb_len: usize, queue_len: usize) {
        let Some(tr) = &self.tracer else {
            return;
        };

        let mut tr = tr.lock();

        if !tr.should_emit_progress() {
            return;
        }

        let snap = tr.snapshot(gb_len, queue_len, true);
        let (delta, total) = tr.mark_progress_sample();
        print_progress_line(&tr, &snap, delta, total);
    }

    pub fn print_summary(&self, gb_len: usize, queue_len: usize) {
        let Some(tr) = &self.tracer else {
            return;
        };

        let tr = tr.lock();
        let snap = tr.snapshot(gb_len, queue_len, true);
        print_summary(&tr, &snap);
    }
}
