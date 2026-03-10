use super::tracer::SharedTracer;
use crate::criteria::{PairCriterion, PairKey};
use crate::{GrobnerBasis, Pair, PairQueue};

/// Wrap any [`PairQueue`] and trace push/pop activity.
pub struct TracingQueue<Q> {
    inner: Q,
    tracer: SharedTracer,
}

impl<Q> TracingQueue<Q> {
    #[must_use]
    pub fn wrap(inner: Q, tracer: SharedTracer) -> Self {
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
        self.tracer.borrow_mut().on_push_with_len(len);
    }

    fn pop(&mut self) -> Option<Pair> {
        let out = self.inner.pop();
        if out.is_some() {
            let len = self.inner.len();
            self.tracer.borrow_mut().on_pop_with_len(len);
        }
        out
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Wrap any [`PairCriterion`] and trace pair rejections.
pub struct TracingPairCriterion<C> {
    inner: C,
    tracer: SharedTracer,
}

impl<C> TracingPairCriterion<C> {
    #[must_use]
    pub fn wrap(inner: C, tracer: SharedTracer) -> Self {
        Self { inner, tracer }
    }

    #[must_use]
    pub fn inner(&self) -> &C {
        &self.inner
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut C {
        &mut self.inner
    }

    #[must_use]
    pub fn into_inner(self) -> C {
        self.inner
    }
}

impl<P, C> PairCriterion<P> for TracingPairCriterion<C>
where
    C: PairCriterion<P>,
{
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool {
        let keep = self.inner.keep_pair(gb, i, j);
        if !keep {
            self.tracer.borrow_mut().on_pair_rejected_by_criterion();
        }
        keep
    }
}

/// Wrap any [`PairKey`] and trace missing keys.
pub struct TracingPairKey<K> {
    inner: K,
    tracer: SharedTracer,
}

impl<K> TracingPairKey<K> {
    #[must_use]
    pub fn wrap(inner: K, tracer: SharedTracer) -> Self {
        Self { inner, tracer }
    }

    #[must_use]
    pub fn inner(&self) -> &K {
        &self.inner
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut K {
        &mut self.inner
    }

    #[must_use]
    pub fn into_inner(self) -> K {
        self.inner
    }
}

impl<P, K> PairKey<P> for TracingPairKey<K>
where
    K: PairKey<P>,
{
    fn key_for_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> Option<u32> {
        let key = self.inner.key_for_pair(gb, i, j);
        if key.is_none() {
            self.tracer.borrow_mut().on_pair_key_missing();
        }
        key
    }
}
