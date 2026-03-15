use super::tracer::SharedBuchbergerTracer;
use crate::{Pair, PairQueue, PairSetView};

/// Wraps any [`PairQueue`] and traces push/pop activity.
#[derive(Debug)]
pub struct TracingQueue<Q> {
    inner: Q,
    tracer: SharedBuchbergerTracer,
}

impl<Q> TracingQueue<Q> {
    #[must_use]
    pub fn wrap(inner: Q, tracer: SharedBuchbergerTracer) -> Self {
        Self { inner, tracer }
    }

    #[must_use]
    pub fn inner(&self) -> &Q {
        &self.inner
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut Q {
        &mut self.inner
    }

    #[must_use]
    pub fn into_inner(self) -> Q {
        self.inner
    }
}

impl<Q> PairQueue for TracingQueue<Q>
where
    Q: PairQueue,
{
    fn new() -> Self
    where
        Self: Sized,
    {
        panic!("TracingQueue::new() is unsupported; use TracingQueue::wrap(inner, tracer)");
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn push(&mut self, pair: Pair) {
        self.inner.push(pair);
        let len = self.inner.len();
        self.tracer.lock().on_push(len);
    }

    fn pop(&mut self) -> Option<Pair> {
        let out = self.inner.pop();
        if out.is_some() {
            let len = self.inner.len();
            self.tracer.lock().on_pop(len);
        }
        out
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<Q> PairSetView for TracingQueue<Q>
where
    Q: PairSetView,
{
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        self.inner.contains_pair(i, j)
    }
}
