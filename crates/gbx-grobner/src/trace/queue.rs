use crate::trace::tracer::Tracer;
use crate::{Pair, PairQueue};
use std::cell::RefCell;
use std::rc::Rc;

/// Wrap any PairQueue and trace pushes/pops.
pub struct TracingQueue<Q> {
    inner: Q,
    tracer: Rc<RefCell<Tracer>>,
}

impl<Q> TracingQueue<Q> {
    pub fn wrap(inner: Q, tracer: Rc<RefCell<Tracer>>) -> Self {
        Self { inner, tracer }
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
        panic!("TracingQueue::new() is not supported; use TracingQueue::wrap(inner, tracer)");
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
            self.tracer.borrow_mut().on_pop(self);
        }
        out
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}
