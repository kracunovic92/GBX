use crate::algos::f4::pairs::pending::PendingPairs;
use crate::algos::f4::symbolic::{SymbolicProduct, SymbolicRow};
use crate::algos::f4::types::PolyMono;

use gbx_poly::polynomial::PolynomialView;

/// Per-batch data kept by the engine for later symbolic simplification.
///
/// This stores:
/// - symbolic products used to build `F_j`,
/// - materialized symbolic rows `F_j`,
/// - reduced rows `F_j_tilde`.
#[derive(Debug, Clone)]
pub struct BatchHistory<P> {
    pub f_j_products: Vec<SymbolicProduct<PolyMono>>,
    pub f_j_rows: Vec<P>,
    pub f_j_tilde: Vec<P>,
}

impl<P> BatchHistory<P> {
    #[inline]
    #[must_use]
    pub fn new(f_j_products: Vec<SymbolicProduct<PolyMono>>, f_j_rows: Vec<P>, f_j_tilde: Vec<P>) -> Self {
        Self { f_j_products, f_j_rows, f_j_tilde }
    }

    #[inline]
    #[must_use]
    pub fn from_symbolic_rows(symbolic_rows: Vec<SymbolicRow<P, PolyMono>>, f_j_tilde: Vec<P>) -> Self {
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
{
    pub basis: Vec<P>,
    pub pending: PendingPairs,
    pub history: Vec<BatchHistory<P>>,
    pub iteration: usize,
}

impl<P> F4State<P>
where
    P: PolynomialView,
{
    #[must_use]
    pub fn new() -> Self {
        Self { basis: Vec::new(), pending: PendingPairs::new(), history: Vec::new(), iteration: 0 }
    }

    #[must_use]
    pub fn is_done(&self) -> bool {
        self.pending.is_empty()
    }

    #[must_use]
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    pub fn advance_iteration(&mut self) {
        self.iteration += 1;
    }

    pub fn push_history(&mut self, symbolic_rows: Vec<SymbolicRow<P, PolyMono>>, reduced_rows: Vec<P>) {
        self.history.push(BatchHistory::from_symbolic_rows(
            symbolic_rows,
            reduced_rows,
        ));
    }
}

impl<P> Default for F4State<P>
where
    P: PolynomialView,
{
    fn default() -> Self {
        Self::new()
    }
}
