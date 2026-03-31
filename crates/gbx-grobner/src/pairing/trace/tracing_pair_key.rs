use crate::pairing::keys::PairKey;
use crate::trace::{PairingTrace, TraceHandle};
use crate::GrobnerBasis;

/// Wraps any [`PairKey`] and traces missing keys.
///
/// This wrapper is purely mechanical and does not change key selection
/// behavior.
#[derive(Debug, Clone)]
pub struct TracingPairKey<K, T> {
    inner: K,
    tracer: TraceHandle<T>,
}

impl<K, T> TracingPairKey<K, T> {
    /// Wraps a pair key strategy with tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: K, tracer: TraceHandle<T>) -> Self {
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

impl<P, K, T> PairKey<P> for TracingPairKey<K, T>
where
    K: PairKey<P>,
    T: PairingTrace,
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
