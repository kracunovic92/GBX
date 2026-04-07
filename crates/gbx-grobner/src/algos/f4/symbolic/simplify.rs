use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::types::{SymbolicProduct, SymbolicSource};

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialExtU32, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;

/// Simplify a symbolic product using previously computed batch history.
///
/// Practical implementation of the paper's `Simplify(t, f, F)`:
/// instead of enumerating all divisors of `t`, we scan historical products
/// `(u, f)` already present in old batches `F_j` and test whether `u | t`.
///
/// If a match is found, we replace `(t, f)` by a product built from the
/// corresponding reduced row in `\tilde F_j`, and recurse.
pub fn simplify_product<P, F, O>(
    ctx: &RingCtx<F, O>,
    product: SymbolicProduct<<P::Term as TermView>::Mono>,
    history: &[BatchHistory<P, <P::Term as TermView>::Mono>],
) -> Result<SymbolicProduct<<P::Term as TermView>::Mono>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    simplify_recursive::<P, F, O>(ctx, product, history)
}

fn simplify_recursive<P, F, O>(
    ctx: &RingCtx<F, O>,
    product: SymbolicProduct<<P::Term as TermView>::Mono>,
    history: &[BatchHistory<P, <P::Term as TermView>::Mono>],
) -> Result<SymbolicProduct<<P::Term as TermView>::Mono>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let Some(rewrite) = find_rewrite::<P, F, O>(ctx, &product, history)? else {
        return Ok(product);
    };

    if rewrite == product {
        return Ok(rewrite);
    }

    simplify_recursive::<P, F, O>(ctx, rewrite, history)
}

/// Try to find one historical rewrite of `(t, f)` using older batches.
///
/// We scan batches from newest to oldest so that simplification prefers the
/// most recent available historical information.
fn find_rewrite<P, F, O>(
    ctx: &RingCtx<F, O>,
    product: &SymbolicProduct<<P::Term as TermView>::Mono>,
    history: &[BatchHistory<P, <P::Term as TermView>::Mono>],
) -> Result<Option<SymbolicProduct<<P::Term as TermView>::Mono>>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    for (batch_index, batch) in history.iter().enumerate().rev() {
        if let Some(rewritten) = try_rewrite_in_batch::<P, F, O>(ctx, product, batch_index, batch, true)? {
            return Ok(Some(rewritten));
        }

        if let Some(rewritten) = try_rewrite_in_batch::<P, F, O>(ctx, product, batch_index, batch, false)? {
            return Ok(Some(rewritten));
        }
    }

    Ok(None)
}

/// Try to rewrite `(t, f)` using one specific batch `F_j`.
///
/// If `exact_only` is true, only consider `u = t`.
/// Otherwise, only consider strict divisors `u != t`.
fn try_rewrite_in_batch<P, F, O>(
    ctx: &RingCtx<F, O>,
    product: &SymbolicProduct<<P::Term as TermView>::Mono>,
    batch_index: usize,
    batch: &BatchHistory<P, <P::Term as TermView>::Mono>,
    exact_only: bool,
) -> Result<Option<SymbolicProduct<<P::Term as TermView>::Mono>>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    for (row_index_in_f_j, historical_product) in batch.f_j_products.iter().enumerate() {
        if historical_product.source != product.source {
            continue;
        }

        let u = &historical_product.multiplier;
        let t = &product.multiplier;

        if !u.divides(t) {
            continue;
        }

        let is_exact = *u == *t;

        if exact_only && !is_exact {
            continue;
        }

        if !exact_only && is_exact {
            continue;
        }

        let Some(f_j_row) = batch.f_j_rows.get(row_index_in_f_j) else {
            continue;
        };

        let Some(head) = f_j_row.leading_mono().cloned() else {
            continue;
        };

        let Some(row_index_in_f_j_tilde) = find_reduced_row_by_head(&batch.f_j_tilde, &head) else {
            continue;
        };

        let rewritten_source = SymbolicSource::HistoryReducedRow { batch_index, row_index: row_index_in_f_j_tilde };

        if is_exact {
            return Ok(Some(SymbolicProduct {
                source: rewritten_source,
                multiplier: one_monomial::<P, F, O>(ctx)?,
            }));
        }

        let quotient = t.checked_div_exact_by(u)?;
        debug_assert_eq!(quotient.n_vars(), ctx.nvars);

        return Ok(Some(SymbolicProduct {
            source: rewritten_source,
            multiplier: quotient,
        }));
    }

    Ok(None)
}

/// Find the unique reduced row in `\tilde F_j` with the requested head.
///
/// First version: linear scan.
/// Later this can be replaced by a cached head -> row-index map.
fn find_reduced_row_by_head<P>(f_j_tilde: &[P], head: &<P::Term as TermView>::Mono) -> Option<usize>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Eq,
{
    f_j_tilde
        .iter()
        .position(|row| row.leading_mono().map(|m| m == head).unwrap_or(false))
}

/// Construct the multiplicative identity monomial with the ring arity from `ctx`.
#[inline]
fn one_monomial<P, F, O>(ctx: &RingCtx<F, O>) -> Result<<P::Term as TermView>::Mono>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    Ok(<<P::Term as TermView>::Mono as MonomialExtU32>::one(
        ctx.nvars,
    )?)
}
