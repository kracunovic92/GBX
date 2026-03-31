use crate::pairing::filters::PairSetView;
use crate::pairing::updates::PairUpdate;
use crate::pairing::updates::Result as PairUpdateResult;
use crate::trace::{PairingTrace, TraceHandle};
use crate::{GrobnerBasis, PairQueue};

/// Wraps any [`PairUpdate`] and traces net queue growth caused by update passes.
///
/// This wrapper is purely mechanical and does not change update behavior.
#[derive(Debug, Clone)]
pub struct TracingPairUpdate<U, T> {
    inner: U,
    tracer: TraceHandle<T>,
}

impl<U, T> TracingPairUpdate<U, T> {
    /// Wraps a pair-update strategy with tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: U, tracer: TraceHandle<T>) -> Self {
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

impl<P, U, T> PairUpdate<P> for TracingPairUpdate<U, T>
where
    U: PairUpdate<P>,
    T: PairingTrace,
{
    type Key = U::Key;

    #[inline]
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize) -> PairUpdateResult<()>
    where
        Q: PairQueue<Key = Self::Key> + PairSetView,
    {
        let before = pairs.len();
        self.inner.on_new_poly(gb, pairs, new_index)?;
        let after = pairs.len();

        if after > before {
            self.tracer.lock().on_pairs_added_by_update(after - before);
        }

        Ok(())
    }
}
