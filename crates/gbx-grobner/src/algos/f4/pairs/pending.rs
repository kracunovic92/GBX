use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::PairCriterion;
use crate::algos::f4::pairs::critical_pair::CriticalPair;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

#[derive(Debug, Clone)]
pub struct PendingPairs<M> {
    pairs: Vec<CriticalPair<M>>,
    present: BTreeSet<(usize, usize)>,
}

impl<M> Default for PendingPairs<M> {
    fn default() -> Self {
        Self { pairs: Vec::new(), present: BTreeSet::new() }
    }
}

impl<M> PendingPairs<M> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[CriticalPair<M>] {
        &self.pairs
    }

    #[must_use]
    pub fn contains(&self, i: usize, j: usize) -> bool {
        self.present.contains(&normalize_pair_key(i, j))
    }

    pub fn push(&mut self, pair: CriticalPair<M>) {
        let key = normalize_pair_key(pair.i, pair.j);
        if self.present.insert(key) {
            self.pairs.push(pair);
        }
    }

    pub fn into_vec(self) -> Vec<CriticalPair<M>> {
        self.pairs
    }

    pub fn clear(&mut self) {
        self.pairs.clear();
        self.present.clear();
    }

    pub fn drain_all(&mut self) -> Vec<CriticalPair<M>> {
        self.present.clear();
        std::mem::take(&mut self.pairs)
    }

    pub fn replace(&mut self, pairs: Vec<CriticalPair<M>>) {
        self.present = pairs
            .iter()
            .map(|pair| normalize_pair_key(pair.i, pair.j))
            .collect();
        self.pairs = pairs;
    }
}

#[inline]
fn normalize_pair_key(i: usize, j: usize) -> (usize, usize) {
    if i <= j { (i, j) } else { (j, i) }
}

pub fn build_all_pairs<P, C>(basis: &[P], criterion: &C) -> Result<Vec<CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>>>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    C: PairCriterion<P>,
{
    let mut out = Vec::new();

    for i in 0..basis.len() {
        for j in (i + 1)..basis.len() {
            if let Some(pair) = make_pair(basis, i, j)? {
                if criterion.allows(basis, &pair) {
                    out.push(pair);
                }
            }
        }
    }

    Ok(out)
}

pub fn add_pairs_with_new_basis_element<P, C>(basis: &[P], new_index: usize, pending: &mut PendingPairs<<<P as PolynomialView>::Term as TermView>::Mono>, criterion: &C) -> Result<()>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    C: PairCriterion<P>,
{
    for old_index in 0..new_index {
        if let Some(pair) = make_pair(basis, old_index, new_index)? {
            if criterion.allows(basis, &pair) {
                pending.push(pair);
            }
        }
    }

    Ok(())
}

pub fn make_pair<P>(basis: &[P], i: usize, j: usize) -> Result<Option<CriticalPair<<<P as PolynomialView>::Term as TermView>::Mono>>>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    if i == j {
        return Ok(None);
    }

    let fi = &basis[i];
    let fj = &basis[j];

    let Some(lm_i) = fi.leading_mono() else {
        return Ok(None);
    };
    let Some(lm_j) = fj.leading_mono() else {
        return Ok(None);
    };

    Ok(Some(CriticalPair::from_lms(i, j, lm_i, lm_j)?))
}
