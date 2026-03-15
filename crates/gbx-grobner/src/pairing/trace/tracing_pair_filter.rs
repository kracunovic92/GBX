use crate::algos::trace::SharedBuchbergerTracer;
use crate::pairing::filters::{PairFilter, PairSetView};
use crate::GrobnerBasis;

/// Wraps any [`PairFilter`] and traces state-aware filter rejections.
#[derive(Debug, Clone)]
pub struct TracingPairFilter<F> {
    inner: F,
    tracer: SharedBuchbergerTracer,
}

impl<F> TracingPairFilter<F> {
    /// Wraps a state-aware pair filter with Buchberger tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: F, tracer: SharedBuchbergerTracer) -> Self {
        Self { inner, tracer }
    }

    /// Returns the wrapped filter.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &F {
        &self.inner
    }

    /// Returns mutable access to the wrapped filter.
    #[must_use]
    #[inline]
    pub fn inner_mut(&mut self) -> &mut F {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped filter.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> F {
        self.inner
    }
}

impl<P, F> PairFilter<P> for TracingPairFilter<F>
where
    F: PairFilter<P>,
{
    #[inline]
    fn keep_pair<S: PairSetView>(&mut self, gb: &GrobnerBasis<P>, state: &S, i: usize, j: usize) -> bool {
        let keep = self.inner.keep_pair(gb, state, i, j);
        if !keep {
            self.tracer.lock().on_pair_rejected_by_filter();
        }
        keep
    }
}
