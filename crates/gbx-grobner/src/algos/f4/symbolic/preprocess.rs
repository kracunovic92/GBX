use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::materialize::materialize_product;
use crate::algos::f4::symbolic::ordered::{OrderedMono, OrderedProduct};
use crate::algos::f4::symbolic::reducers::find_top_reducer_product;
use crate::algos::f4::symbolic::simplify::SimplifyIndex;
use crate::algos::f4::symbolic::types::{SymbolicPreprocessOutput, SymbolicProduct, SymbolicRow, SymbolicRowKind};
use crate::algos::f4::symbolic::worklist::MonomialWorklist;

use gbx_poly::monomial::Monomial;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

/// Symbolic preprocessing phase of F4.
///
/// Paper form:
///
/// ```text
/// F := { mult(Simplify(m, f, F)) | (m, f) in L }
/// Done := HT(F)
/// while T(F) != Done do
///     choose m in T(F) \ Done
///     Done := Done ∪ {m}
///     if m is top reducible modulo G then
///         m = m0 * HT(f)
///         F := F ∪ { mult(Simplify(m0, f, F)) }
/// return F
/// ```
///
/// Implementation notes:
/// - `f_d` stores the current symbolic family.
/// - `ht_f_d` stores heads of rows in `f_d`.
/// - `seen_products` deduplicates symbolic products before materialization.
/// - `done` stores monomials already considered by the closure loop.
/// - `pending_terms` incrementally represents terms of `T(F_d)`.
pub fn symbolic_preprocess<P, F, O>(ctx: &RingCtx<F, O>, l_d: &[SymbolicProduct<Monomial>], basis: &[P], history: &[BatchHistory<P>]) -> Result<SymbolicPreprocessOutput<P, Monomial>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let order = &ctx.order;

    let simplify_index = SimplifyIndex::compile::<P, F, O>(history);

    let mut f_d: Vec<SymbolicRow<P, Monomial>> = Vec::new();
    let mut ht_f_d: Vec<Monomial> = Vec::new();

    let mut seen_products: BTreeSet<OrderedProduct<'_, Monomial, O>> = BTreeSet::new();
    let mut done: BTreeSet<OrderedMono<'_, Monomial, O>> = BTreeSet::new();
    let mut pending_terms: MonomialWorklist<'_, O> = MonomialWorklist::new();

    for product in l_d {
        let simplified = simplify_index.simplify_product::<F, O>(ctx, product.clone())?;

        add_to_f_d(
            ctx,
            &simplified,
            basis,
            history,
            SymbolicRowKind::InitialSeed,
            order,
            &mut seen_products,
            &mut f_d,
            &mut ht_f_d,
            &mut done,
            &mut pending_terms,
        )?;
    }

    while let Some(next) = pending_terms.pop_next() {
        let m = next.into_inner();

        if !done.insert(OrderedMono::new(m.clone(), order)) {
            continue;
        }

        if let Some(product) = find_top_reducer_product(&m, basis)? {
            let simplified = simplify_index.simplify_product::<F, O>(ctx, product.clone())?;

            add_to_f_d(
                ctx,
                &simplified,
                basis,
                history,
                SymbolicRowKind::TopReducerClosure,
                order,
                &mut seen_products,
                &mut f_d,
                &mut ht_f_d,
                &mut done,
                &mut pending_terms,
            )?;
        }
    }

    Ok(SymbolicPreprocessOutput::new(f_d, ht_f_d))
}

/// Add `mult(product)` to `F_d` if the symbolic product is new.
fn add_to_f_d<'a, P, F, O>(
    ctx: &RingCtx<F, O>,
    product: &SymbolicProduct<Monomial>,
    basis: &[P],
    history: &[BatchHistory<P>],
    kind: SymbolicRowKind,
    order: &'a O,
    seen_products: &mut BTreeSet<OrderedProduct<'a, Monomial, O>>,
    f_d: &mut Vec<SymbolicRow<P, Monomial>>,
    ht_f_d: &mut Vec<Monomial>,
    done: &mut BTreeSet<OrderedMono<'a, Monomial, O>>,
    pending_terms: &mut MonomialWorklist<'a, O>,
) -> Result<()>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    let key = OrderedProduct::new(product.source.clone(), product.multiplier.clone(), order);

    if !seen_products.insert(key) {
        return Ok(());
    }

    let mult_product = materialize_product(ctx, product, basis, history)?;

    if let Some(ht) = mult_product.leading_mono().cloned() {
        ht_f_d.push(ht.clone());

        // Eagerly mark row heads as done so closure does not immediately
        // reconsider the head term of a just-inserted row.
        done.insert(OrderedMono::new(ht, order));
    }

    pending_terms.extend_from_row(&mult_product, order);
    f_d.push(SymbolicRow::new(product.clone(), mult_product, kind));

    Ok(())
}
