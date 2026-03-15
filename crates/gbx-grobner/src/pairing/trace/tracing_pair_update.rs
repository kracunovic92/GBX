use crate::algos::trace::SharedBuchbergerTracer;
use crate::pairing::filters::PairSetView;
use crate::pairing::updates::PairUpdate;
use crate::{GrobnerBasis, PairQueue};

/// Wraps any [`PairUpdate`] and traces net queue growth caused by update passes.
#[derive(Debug, Clone)]
pub struct TracingPairUpdate<U> {
    inner: U,
    tracer: SharedBuchbergerTracer,
}

impl<U> TracingPairUpdate<U> {
    /// Wraps a pair update strategy with Buchberger tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: U, tracer: SharedBuchbergerTracer) -> Self {
        Self { inner, tracer }
    }

    /// Returns the wrapped updater.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &U {
        &self.inner
    }

    /// Returns mutable access to the wrapped updater.
    #[must_use]
    #[inline]
    pub fn inner_mut(&mut self) -> &mut U {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped updater.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> U {
        self.inner
    }
}

impl<P, U> PairUpdate<P> for TracingPairUpdate<U>
where
    U: PairUpdate<P>,
{
    #[inline]
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize)
    where
        Q: PairQueue + PairSetView,
    {
        let before = pairs.len();
        self.inner.on_new_poly(gb, pairs, new_index);
        let after = pairs.len();

        if after > before {
            self.tracer.lock().on_pairs_added_by_update(after - before);
        }
    }
}
