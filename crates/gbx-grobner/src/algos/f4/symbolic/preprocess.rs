//! Symbolic preprocessing with reducer closure for F4.

use crate::algos::f4::error::Result;
use crate::algos::f4::symbolic::helpers::{enqueue_support_terms, materialize_multiple};
use crate::algos::f4::symbolic::types::{SeedRow, SymbolicPreprocessing, SymbolicRow, SymbolicRowKind};
use crate::basis::GrobnerBasis;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

use crate::symbolic::helpers::{build_reducer_index, find_any_reducer_indexed};
use std::collections::HashSet;

/// Perform symbolic preprocessing for an F4 batch.
///
/// Starting from the aligned seed rows induced by the selected critical pairs,
/// this function repeatedly closes the row system under top reduction by the
/// current Gröbner basis.
///
/// In this correctness-first implementation:
///
/// - every support monomial encountered in the current row set is inspected,
/// - the first basis reducer found in basis order is used,
/// - duplicate row multiples are suppressed using `(basis_index, multiplier)`,
/// - all inserted rows are materialized immediately as full polynomials.
///
/// This is intentionally conservative. It may introduce more reducer rows than
/// a more optimized symbolic-preprocessing strategy, but it is much easier to
/// reason about and test.
pub fn symbolic_preprocess<F, O, P>(ctx: &RingCtx<F, O>, gb: &GrobnerBasis<P>, seeds: Vec<SeedRow<<P::Term as TermView>::Mono>>) -> Result<SymbolicPreprocessing<P, <P::Term as TermView>::Mono>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + std::hash::Hash,
{
    let reducers = build_reducer_index(gb);
    let mut seed_rows = Vec::new();
    let mut reducer_rows = Vec::new();
    let mut all_rows = Vec::new();

    // Deduplicate materialized basis multiples by:
    //   (source basis polynomial, monomial multiplier)
    let mut inserted_rows: HashSet<(usize, <P::Term as TermView>::Mono)> = HashSet::new();

    // Worklist of support monomials still to inspect for reducibility.
    let mut seen_terms: HashSet<<P::Term as TermView>::Mono> = HashSet::new();
    let mut irreducible_terms: HashSet<<P::Term as TermView>::Mono> = HashSet::new();
    let mut pending_terms: Vec<<P::Term as TermView>::Mono> = Vec::new();

    // 1. Materialize the initial seed rows.
    for seed in seeds {
        let row_key = (seed.source_basis_index, seed.multiplier.clone());
        if !inserted_rows.insert(row_key) {
            continue;
        }

        let poly = materialize_multiple(ctx, gb, seed.source_basis_index, &seed.multiplier)?;
        if poly.is_zero() {
            continue;
        }

        enqueue_support_terms(&poly, &mut seen_terms, &mut pending_terms);

        let row = SymbolicRow { poly, source_basis_index: seed.source_basis_index, multiplier: seed.multiplier, kind: seed.kind };

        seed_rows.push(row.clone());
        all_rows.push(row);
    }

    // 2. Close under reducers from the current basis.
    while let Some(term) = pending_terms.pop() {
        if irreducible_terms.contains(&term) {
            continue;
        }
        let Some((basis_index, quotient)) = find_any_reducer_indexed::<P>(&reducers, &term)? else {
            irreducible_terms.insert(term);
            continue;
        };

        let row_key = (basis_index, quotient.clone());
        if !inserted_rows.insert(row_key) {
            continue;
        }

        let poly = materialize_multiple(ctx, gb, basis_index, &quotient)?;
        if poly.is_zero() {
            continue;
        }

        enqueue_support_terms(&poly, &mut seen_terms, &mut pending_terms);

        let row = SymbolicRow { poly, source_basis_index: basis_index, multiplier: quotient, kind: SymbolicRowKind::Reducer };

        reducer_rows.push(row.clone());
        all_rows.push(row);
    }

    Ok(SymbolicPreprocessing { seed_rows, reducer_rows, all_rows })
}
