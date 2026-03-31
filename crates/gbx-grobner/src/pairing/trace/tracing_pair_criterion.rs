use crate::pairing::criteria::PairCriterion;
use crate::trace::{PairingTrace, TraceHandle};
use crate::GrobnerBasis;

/// Wraps any [`PairCriterion`] and traces local criterion rejections.
///
/// This wrapper is purely mechanical and does not change criterion behavior.
#[derive(Debug, Clone)]
pub struct TracingPairCriterion<C, T> {
    inner: C,
    tracer: TraceHandle<T>,
}

impl<C, T> TracingPairCriterion<C, T> {
    /// Wraps a local pair criterion with tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: C, tracer: TraceHandle<T>) -> Self {
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

impl<P, C, T> PairCriterion<P> for TracingPairCriterion<C, T>
where
    C: PairCriterion<P>,
    T: PairingTrace,
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
