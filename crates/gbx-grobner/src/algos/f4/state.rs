use crate::algos::f4::pairs::pending::PendingPairs;
use crate::algos::f4::simplify::SimplifyIndex;
use crate::algos::f4::symbolic::UnevaluatedProduct;
use crate::algos::f4::types::PolyMono;

use gbx_poly::polynomial::PolynomialView;

/// Per-batch data kept by the engine for later symbolic simplification.
///
/// This stores only the compact symbolic history needed by `Simplify`:
/// - products used to build `F_j`,
/// - leading monomials `HT(F_j)` of those products after materialization,
/// - reduced rows `F_j_tilde`.
///
/// It intentionally does not store the full materialized rows `F_j`.
#[derive(Debug, Clone)]
pub struct BatchHistory<P> {
    pub f_j_products: Vec<UnevaluatedProduct<PolyMono>>,
    pub f_j_heads: Vec<PolyMono>,
    pub f_j_tilde: Vec<P>,
}

impl<P> BatchHistory<P> {
    #[inline]
    #[must_use]
    pub fn new(f_j_products: Vec<UnevaluatedProduct<PolyMono>>, f_j_heads: Vec<PolyMono>, f_j_tilde: Vec<P>) -> Self {
        debug_assert_eq!(
            f_j_products.len(),
            f_j_heads.len(),
            "F4 history products and heads must stay aligned"
        );

        Self { f_j_products, f_j_heads, f_j_tilde }
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
    pub simplify_index: SimplifyIndex,
    pub iteration: usize,
}

impl<P> F4State<P>
where
    P: PolynomialView,
{
    #[must_use]
    pub fn new() -> Self {
        Self { basis: Vec::new(), pending: PendingPairs::new(), history: Vec::new(), simplify_index: SimplifyIndex::new(), iteration: 0 }
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

    pub fn push_history(&mut self, products: Vec<UnevaluatedProduct<PolyMono>>, heads: Vec<PolyMono>, reduced_rows: Vec<P>) {
        let batch_index = self.history.len();

        let batch = BatchHistory::new(products, heads, reduced_rows);

        self.simplify_index.extend_with_batch(batch_index, &batch);

        self.history.push(batch);
    }
    pub fn debug_counts(&self) -> F4StateDebugCounts {
        F4StateDebugCounts { iteration: self.iteration, basis_len: self.basis.len(), history_len: self.history.len(), pending_len: self.pending.len() }
    }
}
#[derive(Debug, Clone)]
pub struct F4StateDebugCounts {
    pub iteration: usize,
    pub basis_len: usize,
    pub history_len: usize,
    pub pending_len: usize,
}

impl<P> Default for F4State<P>
where
    P: PolynomialView,
{
    fn default() -> Self {
        Self::new()
    }
}
