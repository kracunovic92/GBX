use crate::algos::f4::pairs::pending::PendingPairs;
use crate::algos::f4::symbolic::{SymbolicProduct, SymbolicRow};
use crate::algos::f4::types::PolyMono;

use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Per-batch data kept by the engine for later symbolic simplification.
///
/// This stores:
/// - the symbolic products used to build `F_j`
/// - the materialized symbolic rows `F_j`
/// - the reduced rows `\\tilde F_j`
#[derive(Debug, Clone)]
pub struct BatchHistory<P, M> {
    /// Symbolic products used to build `F_j`.
    pub f_j_products: Vec<SymbolicProduct<M>>,

    /// Materialized symbolic family `F_j`.
    pub f_j_rows: Vec<P>,

    /// Row-echelon reduction `\\tilde F_j`.
    pub f_j_tilde: Vec<P>,
}

impl<P, M> BatchHistory<P, M> {
    #[inline]
    #[must_use]
    pub fn new(f_j_products: Vec<SymbolicProduct<M>>, f_j_rows: Vec<P>, f_j_tilde: Vec<P>) -> Self {
        Self { f_j_products, f_j_rows, f_j_tilde }
    }
}

impl<P, M> BatchHistory<P, M>
where
    M: Clone,
{
    /// Build history directly from symbolic rows plus reduced rows.
    #[inline]
    #[must_use]
    pub fn from_symbolic_rows(symbolic_rows: Vec<SymbolicRow<P, M>>, f_j_tilde: Vec<P>) -> Self {
        let mut f_j_products = Vec::with_capacity(symbolic_rows.len());
        let mut f_j_rows = Vec::with_capacity(symbolic_rows.len());

        for row in symbolic_rows {
            f_j_products.push(row.product);
            f_j_rows.push(row.polynomial);
        }

        Self { f_j_products, f_j_rows, f_j_tilde }
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
    pub history: Vec<BatchHistory<P, PolyMono<P>>>,
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

    /// Store one completed batch history `(F_j, \\tilde F_j)`.
    pub fn push_history(&mut self, symbolic_rows: Vec<SymbolicRow<P, PolyMono<P>>>, reduced_rows: Vec<P>)
    where
        <<P as PolynomialView>::Term as TermView>::Mono: Clone,
    {
        self.history.push(BatchHistory::from_symbolic_rows(
            symbolic_rows,
            reduced_rows,
        ));
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
