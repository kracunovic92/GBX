use crate::algos::trace::SharedBuchbergerTracer;
use crate::pairing::criteria::PairCriterion;
use crate::GrobnerBasis;

/// Wraps any [`PairCriterion`] and traces local criterion rejections.
#[derive(Debug, Clone)]
pub struct TracingPairCriterion<C> {
    inner: C,
    tracer: SharedBuchbergerTracer,
}

impl<C> TracingPairCriterion<C> {
    /// Wraps a local pair criterion with Buchberger tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: C, tracer: SharedBuchbergerTracer) -> Self {
        Self { inner, tracer }
    }

    /// Returns the wrapped criterion.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &C {
        &self.inner
    }

    /// Returns mutable access to the wrapped criterion.
    #[must_use]
    #[inline]
    pub fn inner_mut(&mut self) -> &mut C {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped criterion.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> C {
        self.inner
    }
}

impl<P, C> PairCriterion<P> for TracingPairCriterion<C>
where
    C: PairCriterion<P>,
{
    #[inline]
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool {
        let keep = self.inner.keep_pair(gb, i, j);
        if !keep {
            self.tracer.lock().on_pair_rejected_by_criterion();
        }
        keep
    }
}
