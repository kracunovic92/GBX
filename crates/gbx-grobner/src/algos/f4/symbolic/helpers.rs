//! Helper routines for F4 symbolic preprocessing.

use crate::algos::f4::error::{F4Error, Result};
use crate::basis::GrobnerBasis;
use crate::pairing::leading_mono_at;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView, MonomialViewExtU32};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

use std::collections::{BTreeMap, HashSet};

/// Indexed view of basis reducers used during symbolic preprocessing.
///
/// Reducers are grouped by leading-monomial total degree so reducer search can
/// skip impossible candidates quickly.
#[derive(Debug, Clone)]
pub struct ReducerIndex<M> {
    by_degree: BTreeMap<u32, Vec<(usize, M)>>,
}

impl<M> ReducerIndex<M> {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_degree.is_empty()
    }
}

/// Build a reducer index from the current Gröbner basis.
pub fn build_reducer_index<P>(gb: &GrobnerBasis<P>) -> ReducerIndex<<P::Term as TermView>::Mono>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: MonomialView<Word = u32> + Clone,
{
    let mut by_degree: BTreeMap<u32, Vec<(usize, <P::Term as TermView>::Mono)>> = BTreeMap::new();

    for i in 0..gb.len() {
        let Some(lm) = leading_mono_at(gb, i) else {
            continue;
        };

        let deg = lm.total_degree_u32().unwrap_or(u32::MAX);
        by_degree.entry(deg).or_default().push((i, lm.clone()));
    }

    ReducerIndex { by_degree }
}

/// Materialize the multiple `multiplier * gb[basis_index]`.
///
/// This is the main bridge from symbolic seed/reducer descriptors into actual
/// polynomial rows used by matrix construction.
///
/// # Errors
///
/// - [`F4Error::MissingBasisPolynomial`] if `basis_index` is out of range
/// - any polynomial/monomial error produced by the underlying multiplication
pub fn materialize_multiple<F, O, P>(ctx: &RingCtx<F, O>, gb: &GrobnerBasis<P>, basis_index: usize, multiplier: &<P::Term as TermView>::Mono) -> Result<P>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let base = gb
        .get(basis_index)
        .ok_or(F4Error::MissingBasisPolynomial { index: basis_index })?;

    let mut out = base.clone();
    out.mul_monomial_assign_raw(ctx, multiplier)?;

    Ok(out)
}

/// Add all support monomials from `poly` into the pending symbolic worklist.
///
/// Every monomial is inserted into `pending` at most once, controlled by
/// `seen_terms`.
pub fn enqueue_support_terms<P>(poly: &P, seen_terms: &mut HashSet<<P::Term as TermView>::Mono>, pending_terms: &mut Vec<<P::Term as TermView>::Mono>)
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Clone + Eq + std::hash::Hash,
{
    for term in poly.terms() {
        let mono = term.mono().clone();

        if seen_terms.insert(mono.clone()) {
            pending_terms.push(mono);
        }
    }
}

/// Find any reducer in the current basis for `target`, using a prebuilt index.
///
/// Returns `Some((basis_index, quotient))` if there exists a basis polynomial
/// `g_i` such that
///
/// `target = quotient * LM(g_i)`.
///
/// Returns `None` if `target` is not divisible by any current leading monomial.
pub fn find_any_reducer_indexed<P>(reducers: &ReducerIndex<<P::Term as TermView>::Mono>, target: &<P::Term as TermView>::Mono) -> Result<Option<(usize, <P::Term as TermView>::Mono)>>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let target_degree = target.total_degree_u32().unwrap_or(u32::MAX);

    for (&deg, bucket) in reducers.by_degree.range(..=target_degree) {
        let _ = deg;

        for (basis_index, lm) in bucket {
            let quotient = target
                .checked_div_by(lm)
                .map_err(|_| F4Error::SymbolicInvariant)?;

            if let Some(q) = quotient {
                return Ok(Some((*basis_index, q)));
            }
        }
    }

    Ok(None)
}
