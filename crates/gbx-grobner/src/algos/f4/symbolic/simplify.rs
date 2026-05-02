use std::collections::HashMap;

use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::types::{SymbolicProduct, SymbolicSource};

use gbx_poly::monomial::{checked_div_exact, divides, Monomial};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Compiled lookup structure for symbolic `Simplify`.
///
/// This is an implementation-oriented form of the paper's `Simplify(t, f, F)`.
///
/// If a historical product `(u, f)` exists with `u | t`, and the corresponding
/// reduced row is known, then `(t, f)` can be rewritten into `(t / u, p)`,
/// where `p` is that reduced row.
///
/// Exact rewrites are preferred over strict-divisor rewrites.
pub struct SimplifyIndex {
    /// Per-source rewrite data.
    ///
    /// Only rewrites belonging to the same symbolic source are considered.
    by_source: HashMap<SymbolicSource, SourceRewriteIndex>,
}

/// Rewrites available for one specific symbolic source.
struct SourceRewriteIndex {
    /// Exact rewrites keyed by the full multiplier `t`.
    exact: HashMap<Monomial, RewriteTarget>,

    /// Strict-divisor candidates for the same source.
    strict_candidates: Vec<StrictRewriteCandidate>,
}

/// Target of one symbolic rewrite.
#[derive(Clone)]
struct RewriteTarget {
    rewritten_source: SymbolicSource,
}

/// Historical strict-divisor rewrite candidate.
#[derive(Clone)]
struct StrictRewriteCandidate {
    multiplier: Monomial,
    target: RewriteTarget,
}

impl SimplifyIndex {
    /// Compile a simplify index from previously computed batch history.
    ///
    /// History is processed from newest batch to oldest batch, so later
    /// information wins naturally when inserted first.
    pub fn compile<P, F, O>(history: &[BatchHistory<P>]) -> Self
    where
        F: FieldCtx<Elem = P::Coeff>,
        O: MonomialOrder,
        P: PolynomialView,
        P::Coeff: Copy + Eq,
    {
        let mut by_source: HashMap<SymbolicSource, SourceRewriteIndex> = HashMap::new();

        for (batch_index, batch) in history.iter().enumerate().rev() {
            let head_to_reduced_row = build_head_to_reduced_row_map(&batch.f_j_tilde);

            for (row_index_in_f_j, historical_product) in batch.f_j_products.iter().enumerate() {
                let Some(f_j_row) = batch.f_j_rows.get(row_index_in_f_j) else {
                    continue;
                };

                let Some(head) = f_j_row.leading_mono().cloned() else {
                    continue;
                };

                let Some(&row_index_in_f_j_tilde) = head_to_reduced_row.get(&head) else {
                    continue;
                };

                let target = RewriteTarget { rewritten_source: SymbolicSource::HistoryReducedRow { batch_index, row_index: row_index_in_f_j_tilde } };

                let source_entry = by_source
                    .entry(historical_product.source.clone())
                    .or_insert_with(|| SourceRewriteIndex { exact: HashMap::new(), strict_candidates: Vec::new() });

                // Exact match entry: (t, f) -> (1, reduced_row).
                // Newer batches are processed first, so preserve first inserted value.
                source_entry
                    .exact
                    .entry(historical_product.multiplier.clone())
                    .or_insert_with(|| target.clone());

                source_entry
                    .strict_candidates
                    .push(StrictRewriteCandidate { multiplier: historical_product.multiplier.clone(), target });
            }
        }

        Self { by_source }
    }

    /// Simplify one symbolic product by repeatedly applying historical rewrites
    /// until no further rewrite is available.
    pub fn simplify_product<F, O>(&self, ctx: &RingCtx<F, O>, product: SymbolicProduct<Monomial>) -> Result<SymbolicProduct<Monomial>>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        let mut current = product;

        loop {
            let Some(next) = self.find_rewrite(ctx, &current)? else {
                return Ok(current);
            };

            if next == current {
                return Ok(current);
            }

            current = next;
        }
    }

    /// Find one rewrite step for the given symbolic product.
    ///
    /// Exact rewrites are preferred over strict-divisor rewrites.
    fn find_rewrite<F, O>(&self, ctx: &RingCtx<F, O>, product: &SymbolicProduct<Monomial>) -> Result<Option<SymbolicProduct<Monomial>>>
    where
        F: FieldCtx,
        O: MonomialOrder,
    {
        let Some(source_index) = self.by_source.get(&product.source) else {
            return Ok(None);
        };

        // Fast path: exact multiplier reuse.
        if let Some(target) = source_index.exact.get(&product.multiplier) {
            return Ok(Some(SymbolicProduct {
                source: target.rewritten_source.clone(),
                multiplier: Monomial::one(ctx.nvars),
            }));
        }

        // Fallback: strict divisor search among candidates for the same source.
        for candidate in &source_index.strict_candidates {
            let u = &candidate.multiplier;
            let t = &product.multiplier;

            if u == t {
                continue;
            }

            if !divides(u, t) {
                continue;
            }

            let quotient = checked_div_exact(u, t)?;
            debug_assert_eq!(quotient.n_vars(), ctx.nvars);

            return Ok(Some(SymbolicProduct {
                source: candidate.target.rewritten_source.clone(),
                multiplier: quotient,
            }));
        }

        Ok(None)
    }
}

/// Build a lookup map from leading monomial to row index in `f_j_tilde`.
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
