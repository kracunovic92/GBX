use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::state::BatchHistory;
use crate::algos::f4::symbolic::ordered::{OrderedMono, OrderedSeed};
use crate::algos::f4::symbolic::reducers::find_top_reducer;
use crate::algos::f4::symbolic::seeds::{materialize_seed, SymbolicSeed};
use crate::algos::f4::types::PolyMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

pub fn symbolic_preprocess<P, F, O>(ctx: &RingCtx<F, O>, seeds: &[SymbolicSeed<PolyMono<P>>], basis: &[P], _history: &[BatchHistory<P>]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    PolyMono<P>: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let order = &ctx.order;

    let mut rows: Vec<P> = Vec::new();
    let mut seen_products: BTreeSet<OrderedSeed<'_, PolyMono<P>, O>> = BTreeSet::new();

    // F := { t * f | (index(f), t) in L }
    for seed in seeds {
        let key = OrderedSeed::new(seed.basis_index, seed.multiplier.clone(), order);

        if seen_products.insert(key) {
            let row = materialize_seed(ctx, seed, basis)?;
            rows.push(row);
        }
    }

    // Done := HT(F)
    let mut done: BTreeSet<OrderedMono<'_, PolyMono<P>, O>> = leading_monomials(&rows, order);

    // while T(F) != Done
    loop {
        let next = all_monomials(&rows, order)
            .into_iter()
            .find(|m| !done.contains(m));

        let Some(next_ordered) = next else {
            break;
        };

        done.insert(next_ordered.clone());
        let monomial = next_ordered.into_inner();

        if let Some((basis_index, multiplier)) = find_top_reducer::<P>(&monomial, basis)? {
            let key = OrderedSeed::new(basis_index, multiplier.clone(), order);

            if seen_products.insert(key) {
                let reducer_seed = SymbolicSeed { basis_index, multiplier };
                let row = materialize_seed(ctx, &reducer_seed, basis)?;
                rows.push(row);
            }
        }
    }

    Ok(rows)
}

fn leading_monomials<'a, P, O>(rows: &[P], order: &'a O) -> BTreeSet<OrderedMono<'a, PolyMono<P>, O>>
where
    P: PolynomialView,
    P::Term: TermView,
    PolyMono<P>: Clone + MonomialView<Word = u32>,
    O: MonomialOrder,
{
    rows.iter()
        .filter_map(|row| row.leading_mono().cloned())
        .map(|mono| OrderedMono::new(mono, order))
        .collect()
}

fn all_monomials<'a, P, O>(rows: &[P], order: &'a O) -> BTreeSet<OrderedMono<'a, PolyMono<P>, O>>
where
    P: PolynomialView,
    P::Term: TermView,
    PolyMono<P>: Clone + MonomialView<Word = u32>,
    O: MonomialOrder,
{
    let mut out = BTreeSet::new();

    for row in rows {
        for term in row.terms().iter() {
            out.insert(OrderedMono::new(term.mono().clone(), order));
        }
    }

    out
}
