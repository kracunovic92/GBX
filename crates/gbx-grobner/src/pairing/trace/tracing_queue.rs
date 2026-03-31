use crate::trace::{QueueTrace, TraceHandle};
use crate::{Pair, PairQueue, PairSetView};

/// Wraps any [`PairQueue`] and traces push/pop activity.
///
/// This wrapper is purely mechanical and does not change queue behavior.
#[derive(Debug, Clone)]
pub struct TracingQueue<Q, T> {
    inner: Q,
    tracer: TraceHandle<T>,
}

impl<Q, T> TracingQueue<Q, T> {
    /// Wraps a queue with tracing.
    #[must_use]
    #[inline]
    pub fn wrap(inner: Q, tracer: TraceHandle<T>) -> Self {
        Self { inner, tracer }
    }

    /// Returns the wrapped queue.
    #[must_use]
    #[inline]
    pub fn inner(&self) -> &Q {
        &self.inner
    }

    /// Returns mutable access to the wrapped queue.
    #[must_use]
    #[inline]
    pub fn inner_mut(&mut self) -> &mut Q {
        &mut self.inner
    }

    /// Consumes the wrapper and returns the wrapped queue.
    #[must_use]
    #[inline]
    pub fn into_inner(self) -> Q {
        self.inner
    }
}

impl<Q, T> PairQueue for TracingQueue<Q, T>
where
    Q: PairQueue,
    T: QueueTrace,
{
    type Key = Q::Key;

    fn new() -> Self
    where
        Self: Sized,
    {
        panic!("TracingQueue::new() is unsupported; use TracingQueue::wrap(inner, tracer)");
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    fn push(&mut self, pair: Pair<Self::Key>) {
        self.inner.push(pair);
        let len = self.inner.len();
        self.tracer.lock().on_push(len);
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair<Self::Key>> {
        let out = self.inner.pop();
        if out.is_some() {
            let len = self.inner.len();
            self.tracer.lock().on_pop(len);
        }
        out
    }
    #[inline]
    fn peek(&self) -> Option<&Pair<Self::Key>> {
        let out = self.inner.peek();
        out
    }
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<Q, T> PairSetView for TracingQueue<Q, T>
where
    Q: PairSetView,
{
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        self.inner.contains_pair(i, j)
    }
}
