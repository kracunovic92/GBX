use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::materialize::materialize_product;
use crate::algos::f4::symbolic::ordered::{OrderedMono, OrderedProduct};
use crate::algos::f4::symbolic::reducers::find_top_reducer_product;
use crate::algos::f4::symbolic::simplify::simplify_product;
use crate::algos::f4::symbolic::types::{SymbolicPreprocessOutput, SymbolicProduct, SymbolicRow, SymbolicRowKind};
use crate::algos::f4::symbolic::worklist::MonomialWorklist;
use crate::algos::f4::types::PolyMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Symbolic Preprocessing
///
/// Paper form:
/// F := { mult(Simplify(m, f, F)) | (m, f) in L }
/// Done := HT(F)
/// while T(F) != Done do
///     choose m in T(F) \ Done
///     Done := Done ∪ {m}
///     if m is top reducible modulo G then
///         m = m0 * HT(f)
///         F := F ∪ { mult(Simplify(m0, f, F)) }
/// return F
pub fn symbolic_preprocess<P, F, O>(
    ctx: &RingCtx<F, O>,
    l_d: &[SymbolicProduct<PolyMono<P>>],
    basis: &[P],
    history: &[BatchHistory<P, PolyMono<P>>],
) -> Result<SymbolicPreprocessOutput<P, PolyMono<P>>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    PolyMono<P>: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Default,
{
    let order = &ctx.order;

    // F_d: the current symbolic family.
    let mut f_d: Vec<SymbolicRow<P, PolyMono<P>>> = Vec::new();

    // HT(F_d): leading monomials of the symbolic family.
    let mut ht_f_d: Vec<PolyMono<P>> = Vec::new();

    // Deduplicate symbolic products before materialization.
    let mut seen_products: BTreeSet<OrderedProduct<'_, PolyMono<P>, O>> = BTreeSet::new();

    // Done := HT(F_d)
    let mut done: BTreeSet<OrderedMono<'_, PolyMono<P>, O>> = BTreeSet::new();

    // Incremental representation of T(F_d).
    let mut pending_terms: MonomialWorklist<'_, PolyMono<P>, O> = MonomialWorklist::new();

    // F := { mult(Simplify(m, f, F)) | (m, f) ∈ L }
    for product in l_d {
        let simplified = simplify_product::<P, F, O>(ctx, product.clone(), history)?;

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

    // while T(F_d) != Done
    while let Some(next) = pending_terms.pop_next() {
        if done.contains(&next) {
            continue;
        }

        // m ∈ T(F_d) \ Done
        let m = next.into_inner();

        // Done := Done ∪ {m}
        done.insert(OrderedMono::new(m.clone(), order));

        // if m top reducible modulo G then
        if let Some(product) = find_top_reducer_product::<P>(&m, basis)? {
            let simplified = simplify_product::<P, F, O>(ctx, product.clone(), history)?;

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
///
/// This helper is used in both places where the paper does:
/// F := F ∪ { mult(...) }
fn add_to_f_d<'a, P, F, O>(
    ctx: &RingCtx<F, O>,
    product: &SymbolicProduct<PolyMono<P>>,
    basis: &[P],
    history: &[BatchHistory<P, PolyMono<P>>],
    kind: SymbolicRowKind,
    order: &'a O,
    seen_products: &mut BTreeSet<OrderedProduct<'a, PolyMono<P>, O>>,
    f_d: &mut Vec<SymbolicRow<P, PolyMono<P>>>,
    ht_f_d: &mut Vec<PolyMono<P>>,
    done: &mut BTreeSet<OrderedMono<'a, PolyMono<P>, O>>,
    pending_terms: &mut MonomialWorklist<'a, PolyMono<P>, O>,
) -> Result<()>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    PolyMono<P>: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let key = OrderedProduct::new(product.source.clone(), product.multiplier.clone(), order);

    if !seen_products.insert(key) {
        return Ok(());
    }

    let mult_product = materialize_product(ctx, product, basis, history)?;

    if let Some(ht) = mult_product.leading_mono().cloned() {
        ht_f_d.push(ht.clone());
        done.insert(OrderedMono::new(ht, order));
    }

    pending_terms.extend_from_row(&mult_product, order);

    f_d.push(SymbolicRow::new(product.clone(), mult_product, kind));

    Ok(())
}
