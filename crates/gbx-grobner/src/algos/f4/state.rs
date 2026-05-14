//! Mutable state for the F4 engine.

use crate::algos::f4::pairs::pending::PendingPairs;
use crate::algos::f4::simplify::SimplifyIndex;
use crate::algos::f4::symbolic::UnevaluatedProduct;
use crate::algos::f4::types::PolyMono;

use gbx_poly::polynomial::PolynomialView;

/// Compact symbolic history for one processed F4 batch.
///
/// The history stores the data needed by symbolic simplification without
/// retaining the full materialized symbolic row set.
#[derive(Debug, Clone)]
pub struct BatchHistory<P> {
    /// Products used to build the symbolic row set `F_j`.
    pub f_j_products: Vec<UnevaluatedProduct<PolyMono>>,

    /// Leading monomials of the materialized symbolic rows.
    ///
    /// This vector is aligned with `f_j_products`.
    pub f_j_heads: Vec<PolyMono>,

    /// Reduced rows `F_j_tilde` produced by matrix reduction.
    pub f_j_tilde: Vec<P>,
}

impl<P> BatchHistory<P> {
    /// Creates a batch-history record.
    ///
    /// `f_j_products` and `f_j_heads` must have the same length.
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

    /// Returns the number of symbolic products stored for this batch.
    #[inline]
    #[must_use]
    pub fn product_len(&self) -> usize {
        self.f_j_products.len()
    }

    /// Returns the number of reduced rows stored for this batch.
    #[inline]
    #[must_use]
    pub fn reduced_len(&self) -> usize {
        self.f_j_tilde.len()
    }

    /// Returns `true` when this batch stores no products or reduced rows.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.f_j_products.is_empty() && self.f_j_tilde.is_empty()
    }
}

/// Mutable state of the F4 main loop.
#[derive(Debug, Clone)]
pub struct F4State<P>
where
    P: PolynomialView,
{
    /// Current basis.
    pub basis: Vec<P>,

    /// Critical pairs waiting to be processed.
    pub pending: PendingPairs,

    /// Per-batch symbolic history used by `Simplify`.
    pub history: Vec<BatchHistory<P>>,

    /// Incremental simplification index built from `history`.
    pub simplify_index: SimplifyIndex,

    iteration: usize,
}

impl<P> F4State<P>
where
    P: PolynomialView,
{
    /// Creates an empty F4 state.
    #[must_use]
    pub fn new() -> Self {
        Self { basis: Vec::new(), pending: PendingPairs::new(), history: Vec::new(), simplify_index: SimplifyIndex::new(), iteration: 0 }
    }

    /// Returns `true` when no critical pairs remain.
    #[inline]
    #[must_use]
    pub fn is_done(&self) -> bool {
        self.pending.is_empty()
    }

    /// Returns the number of iterations already started.
    #[inline]
    #[must_use]
    pub fn iteration(&self) -> usize {
        self.iteration
    }

    /// Advances the iteration counter.
    #[inline]
    pub fn advance_iteration(&mut self) {
        self.iteration += 1;
    }

    /// Stores one processed batch and extends the simplification index.
    pub fn push_history(&mut self, products: Vec<UnevaluatedProduct<PolyMono>>, heads: Vec<PolyMono>, reduced_rows: Vec<P>) {
        let batch_index = self.history.len();
        let batch = BatchHistory::new(products, heads, reduced_rows);

        self.simplify_index.extend_with_batch(batch_index, &batch);
        self.history.push(batch);
    }

    /// Returns compact counters useful for logs and tests.
    #[inline]
    #[must_use]
    pub fn debug_counts(&self) -> F4StateDebugCounts {
        F4StateDebugCounts { iteration: self.iteration, basis_len: self.basis.len(), history_len: self.history.len(), pending_len: self.pending.len() }
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

/// Compact debug counters for an [`F4State`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F4StateDebugCounts {
    /// Current iteration counter.
    pub iteration: usize,

    /// Number of basis elements.
    pub basis_len: usize,

    /// Number of stored batch-history records.
    pub history_len: usize,

    /// Number of pending critical pairs.
    pub pending_len: usize,
}
