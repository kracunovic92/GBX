use std::collections::HashMap;

use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;

use crate::algos::f4::simplify::rules::SourceSimplifyRules;
use crate::algos::f4::simplify::target::RewriteTarget;
use crate::symbolic::{ProductSource, UnevaluatedProduct};
use gbx_poly::monomial::{checked_div_exact, divides, Monomial};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Lookup structure for the F4 `Simplify` step.
///
/// `Simplify(t, f, history)` searches previous batches for a product
/// `u * f` with `u | t`. If found, it rewrites:
///
/// ```text
/// t * f -> (t / u) * p
/// ```
///
/// where `p` is the corresponding reduced historical row.
#[derive(Debug, Clone)]
pub struct SimplifyIndex {
    /// Rewrite rules grouped by product source.
    by_source: HashMap<ProductSource, SourceSimplifyRules>,
}

impl SimplifyIndex {
    #[must_use]
    pub fn new() -> Self {
        Self { by_source: HashMap::new() }
    }
    pub fn extend_with_batch<P>(&mut self, batch_index: usize, batch: &BatchHistory<P>)
    where
        P: PolynomialView,
    {
        let head_to_reduced_row = build_head_to_reduced_row_map(&batch.f_j_tilde);

        debug_assert_eq!(
            batch.f_j_products.len(),
            batch.f_j_heads.len(),
            "F4 history products and heads must stay aligned"
        );

        for (historical_product, head) in batch.f_j_products.iter().zip(batch.f_j_heads.iter()) {
            let Some(&row_index_in_f_j_tilde) = head_to_reduced_row.get(head) else {
                continue;
            };

            let target = RewriteTarget { source: ProductSource::HistoryReducedRow { batch_index, row_index: row_index_in_f_j_tilde } };

            let rules = self
                .by_source
                .entry(historical_product.source)
                .or_insert_with(SourceSimplifyRules::new);

            rules.insert_exact(historical_product.multiplier.clone(), target.clone());

            rules.insert_divisor_rewrite(historical_product.multiplier.clone(), target);
        }
    }
    /// Compiles simplify rewrite rules from previous batch history.
    ///
    /// Newer batches are processed first, so when duplicate exact rewrites
    /// exist, the newest one wins.
    #[must_use]
    pub fn compile<P>(history: &[BatchHistory<P>]) -> Self
    where
        P: PolynomialView,
    {
        let mut index = Self::new();

        for (batch_index, batch) in history.iter().enumerate() {
            index.extend_with_batch(batch_index, batch);
        }

        index
    }

    /// Simplifies one unevaluated product by repeatedly applying historical
    /// rewrite rules until no further rewrite is available.
    pub fn simplify_product<F, O>(&self, ctx: &RingCtx<F, O>, product: UnevaluatedProduct<Monomial>) -> Result<UnevaluatedProduct<Monomial>>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        let mut current = product;
        #[cfg(debug_assertions)]
        let mut steps = 0usize;

        loop {
            #[cfg(debug_assertions)]
            {
                steps += 1;
                debug_assert!(steps < 128, "Simplify rewrite loop did not converge");
            }

            let Some(next) = self.find_rewrite(ctx, &current)? else {
                return Ok(current);
            };

            if next == current {
                return Ok(current);
            }

            current = next;
        }
    }

    /// Finds one rewrite step for the given product.
    ///
    /// Exact rewrites are preferred over divisor rewrites.
    fn find_rewrite<F, O>(&self, ctx: &RingCtx<F, O>, product: &UnevaluatedProduct<Monomial>) -> Result<Option<UnevaluatedProduct<Monomial>>>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        let Some(rules) = self.by_source.get(&product.source) else {
            return Ok(None);
        };

        // Exact rewrite: (t, f) -> (1, p).
        if let Some(target) = rules.exact_rewrites.get(&product.multiplier) {
            return Ok(Some(UnevaluatedProduct {
                source: target.source.clone(),
                multiplier: Monomial::one(ctx.nvars),
            }));
        }

        // Divisor rewrite: (t, f) -> (t / u, p), where u | t.
        for rewrite in &rules.divisor_rewrites {
            let u = &rewrite.divisor;
            let t = &product.multiplier;

            if u == t {
                continue;
            }

            if !divides(u, t) {
                continue;
            }

            // Must compute t / u.
            // Verify checked_div_exact's argument order in gbx_poly.
            let quotient = checked_div_exact(u, t)?;
            debug_assert_eq!(quotient.n_vars(), ctx.nvars);

            return Ok(Some(UnevaluatedProduct {
                source: rewrite.target.source.clone(),
                multiplier: quotient,
            }));
        }
        Ok(None)
    }
}

/// Builds a lookup from leading monomial to row index in `F̃_j`.
///
/// If duplicate heads appear, the first row wins.
fn build_head_to_reduced_row_map<P>(f_j_tilde: &[P]) -> HashMap<Monomial, usize>
where
    P: PolynomialView,
{
    let mut map = HashMap::with_capacity(f_j_tilde.len());

    for (row_index, row) in f_j_tilde.iter().enumerate() {
        let Some(head) = row.leading_mono().cloned() else {
            continue;
        };

        map.entry(head).or_insert(row_index);
    }

    map
}

impl Default for SimplifyIndex {
    fn default() -> Self {
        Self::new()
    }
}
