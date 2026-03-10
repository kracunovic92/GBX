use crate::criteria::{PairCriterion, PairKey, PairUpdate};
use crate::trace::tracer::Tracer;
use crate::{GrobnerBasis, PairQueue};
use gbx_field::fp::{FpDyn, FpDynElem};
use gbx_poly::monomial::DynamicMonomial;
use gbx_poly::order::Grevlex;
use gbx_poly::polynomial::PolyDyn;
use gbx_poly::pretty_str;
use gbx_poly::ring::RingCtx;
use gbx_poly::term::Term;
use gbx_storage::polynomial::VecTerms;
use std::cell::RefCell;
use std::rc::Rc;

pub type TraceTerm = Term<FpDynElem, DynamicMonomial>;
pub type TracePoly = PolyDyn<FpDynElem, VecTerms<TraceTerm>>;
pub type TraceRing = RingCtx<FpDyn, Grevlex>;

/// Shared tracing handle.
pub type SharedTracer = Rc<RefCell<Tracer>>;

/// Generic wrapper for any `PairUpdate<P>` that traces basis growth
/// after a new polynomial is appended.
pub struct TracingPairUpdate<U> {
    inner: U,
    tracer: SharedTracer,
}

impl<U> TracingPairUpdate<U> {
    #[must_use]
    pub fn wrap(inner: U, tracer: SharedTracer) -> Self {
        Self { inner, tracer }
    }

    #[must_use]
    pub fn inner(&self) -> &U {
        &self.inner
    }

    #[must_use]
    pub fn inner_mut(&mut self) -> &mut U {
        &mut self.inner
    }

    #[must_use]
    pub fn into_inner(self) -> U {
        self.inner
    }
}

impl<P, U> PairUpdate<P> for TracingPairUpdate<U>
where
    U: PairUpdate<P>,
{
    fn on_new_poly<Q>(&mut self, gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize)
    where
        Q: PairQueue,
    {
        self.inner.on_new_poly(gb, pairs, new_index);
        self.tracer
            .borrow_mut()
            .on_new_poly(new_index, gb.len(), pairs);
    }
}

/// Generic wrapper for any `PairCriterion<P>` that traces rejected pairs.
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

/// Generic wrapper for any `PairKey<P>` that traces missing keys.
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

/// Helper: pretty polynomial string for the concrete GBX dynamic setup.
#[must_use]
pub fn pretty_poly_str(ring: &TraceRing, vars: &[String], p: &TracePoly) -> String {
    pretty_str!(ring, p, vars)
}
