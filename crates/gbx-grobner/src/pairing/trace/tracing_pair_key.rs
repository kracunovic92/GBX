use crate::algos::trace::SharedBuchbergerTracer;
use crate::pairing::keys::PairKey;
use crate::GrobnerBasis;

/// Wraps any [`PairKey`] and traces missing keys.
#[derive(Debug, Clone)]
pub struct TracingPairKey<K> {
    inner: K,
    tracer: SharedBuchbergerTracer,
}

impl<K> TracingPairKey<K> {
    /// Wraps a pair key strategy with Buchberger tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: K, tracer: SharedBuchbergerTracer) -> Self {
        Self { inner, tracer }
    }

    /// Returns the wrapped key strategy.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &K {
        &self.inner
    }

    /// Returns mutable access to the wrapped key strategy.
    #[must_use]
    #[inline]
    pub fn inner_mut(&mut self) -> &mut K {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped key strategy.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> K {
        self.inner
    }
}

impl<P, K> PairKey<P> for TracingPairKey<K>
where
    K: PairKey<P>,
{
    type Key = K::Key;

    #[inline]
    fn key_for_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<Self::Key> {
        let key = self.inner.key_for_pair(gb, i, j);
        if key.is_none() {
            self.tracer.lock().on_pair_key_missing();
        }
        key
    }
}
