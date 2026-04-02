use crate::algos::f4::pairs::pending::PendingPairs;
use crate::algos::f4::types::PolyMono;

use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Per-batch data optionally kept by the engine.
///
/// For now this stores both the symbolic input rows and the reduced rows.
/// That keeps compatibility with the current engine shape and leaves room
/// for later simplification or tracing logic.
#[derive(Debug, Clone)]
pub struct BatchHistory<P> {
    pub symbolic_rows: Vec<P>,
    pub reduced_rows: Vec<P>,
}

impl<P> BatchHistory<P> {
    #[must_use]
    pub fn new(symbolic_rows: Vec<P>, reduced_rows: Vec<P>) -> Self {
        Self { symbolic_rows, reduced_rows }
    }
}

/// Mutable state of the F4 main loop.
pub struct F4State<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    pub basis: Vec<P>,
    pub pending: PendingPairs<PolyMono<P>>,
    pub history: Vec<BatchHistory<P>>,
    pub iteration: usize,
}

impl<P> F4State<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    #[must_use]
    pub fn new() -> Self {
        Self { basis: Vec::new(), pending: PendingPairs::new(), history: Vec::new(), iteration: 0 }
    }

    #[must_use]
    pub fn is_done(&self) -> bool {
        self.pending.is_empty()
    }

    pub fn advance_iteration(&mut self) {
        self.iteration += 1;
    }

    pub fn push_history(&mut self, symbolic_rows: Vec<P>, reduced_rows: Vec<P>) {
        self.history
            .push(BatchHistory::new(symbolic_rows, reduced_rows));
    }
}

impl<P> Default for F4State<P>
where
    P: PolynomialView,
    P::Term: TermView,
{
    fn default() -> Self {
        Self::new()
    }
}
