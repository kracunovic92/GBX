use super::{
    reporter::{print_progress_line, print_summary},
    snapshot::BuchbergerTraceSnapshot,
    timings::{PhaseKind, WhileKind},
    tracer::SharedBuchbergerTracer,
};
use crate::{GrobnerBasis, PairQueue};
use gbx_poly::polynomial::PolynomialView;
use std::time::Duration;

/// Thin facade around an optional shared tracer.
///
/// This removes `Option` and locking noise from the Buchberger engine.
#[derive(Clone)]
pub struct BuchbergerTraceCtx {
    tracer: Option<SharedBuchbergerTracer>,
}

impl BuchbergerTraceCtx {
    #[must_use]
    pub fn new(tracer: Option<SharedBuchbergerTracer>) -> Self {
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
    pub fn add_phase_time(&self, kind: PhaseKind, dt: Duration) {
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
    pub fn on_zero_reduction(&self) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_zero_reduction();
        }
    }

    #[inline]
    pub fn on_unit_reduction(&self) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_unit_reduction();
        }
    }

    #[inline]
    pub fn on_inserted_poly(&self, gb_len: usize, queue_len: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_inserted_poly(gb_len, queue_len);
        }
    }

    #[inline]
    pub fn on_seeded_pair(&self, queue_len: usize) {
        if let Some(tr) = &self.tracer {
            tr.lock().on_seeded_pair(queue_len);
        }
    }

    #[must_use]
    #[inline]
    pub fn snapshot<P, Q>(&self, gb: &GrobnerBasis<P>, pairs: &Q, basis_terms: Option<usize>, max_poly_terms: Option<usize>) -> Option<BuchbergerTraceSnapshot>
    where
        Q: PairQueue,
    {
        self.tracer.as_ref().map(|tr| {
            let tr = tr.lock();
            tr.snapshot(gb.len(), pairs.len(), basis_terms, max_poly_terms, true)
        })
    }

    pub fn maybe_emit_progress<P, Q>(&self, gb: &GrobnerBasis<P>, pairs: &Q)
    where
        Q: PairQueue,
        P: PolynomialView,
    {
        let Some(tr) = &self.tracer else {
            return;
        };

        let mut tr = tr.lock();

        if !tr.should_emit_progress() {
            return;
        }

        let basis_term_count = gb.as_slice().iter().map(|p| p.len()).sum::<usize>();
        let max_poly_terms = gb.as_slice().iter().map(|p| p.len()).max().unwrap_or(0);

        let snap = tr.snapshot(
            gb.len(),
            pairs.len(),
            Some(basis_term_count),
            Some(max_poly_terms),
            true,
        );

        let (delta, total) = tr.mark_progress_sample();

        print_progress_line(&tr, &snap, delta, total);
    }

    pub fn print_summary<P, Q>(&self, gb: &GrobnerBasis<P>, pairs: &Q)
    where
        Q: PairQueue,
        P: PolynomialView,
    {
        let Some(tr) = &self.tracer else {
            return;
        };

        let tr = tr.lock();

        let basis_term_count = gb.as_slice().iter().map(|p| p.len()).sum::<usize>();
        let max_poly_terms = gb.as_slice().iter().map(|p| p.len()).max().unwrap_or(0);

        let snap = tr.snapshot(
            gb.len(),
            pairs.len(),
            Some(basis_term_count),
            Some(max_poly_terms),
            true,
        );

        print_summary(&tr, &snap);
    }

    pub fn print_empty_summary<Q>(&self, pairs: &Q)
    where
        Q: PairQueue,
    {
        let Some(tr) = &self.tracer else {
            return;
        };

        let tr = tr.lock();
        let snap = tr.snapshot(0, pairs.len(), Some(0), Some(0), true);
        print_summary(&tr, &snap);
    }

    pub fn print_unit_summary<Q>(&self, pairs: &Q)
    where
        Q: PairQueue,
    {
        let Some(tr) = &self.tracer else {
            return;
        };

        let tr = tr.lock();

        // Early termination with a nonzero constant means the basis is effectively [1].
        let snap = tr.snapshot(1, pairs.len(), None, None, true);
        print_summary(&tr, &snap);
    }
}
