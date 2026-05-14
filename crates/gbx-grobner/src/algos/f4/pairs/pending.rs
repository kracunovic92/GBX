//! Pending critical-pair storage.
use std::collections::BTreeSet;

use crate::algos::f4::error::Result;
use crate::algos::f4::pairs::criterion::PairCriterion;
use crate::algos::f4::pairs::critical_pair::CriticalPair;

use gbx_poly::polynomial::PolynomialView;

/// Storage for critical pairs waiting to be selected by a future F4 iteration.
///
/// Invariant:
/// - `pairs` contains at most one critical pair for each unordered index pair `{i, j}`
/// - `present` stores exactly the normalized keys of the pairs currently in `pairs`
#[derive(Debug, Clone, Default)]
pub struct PendingPairs {
    pairs: Vec<CriticalPair>,
    present: BTreeSet<(usize, usize)>,
}

impl PendingPairs {
    /// Creates an empty pending-pair set.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` when no pairs are pending.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Returns the number of pending pairs.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// Borrows the pending pairs in insertion order.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[CriticalPair] {
        &self.pairs
    }

    /// Returns `true` when the unordered pair `{i, j}` is pending.
    #[inline]
    #[must_use]
    pub fn contains(&self, i: usize, j: usize) -> bool {
        self.present.contains(&normalize_pair_key(i, j))
    }

    /// Insert a critical pair if its unordered index pair is not already present.
    ///
    /// Returns `true` if the pair was inserted, `false` if it was already present.
    pub fn insert(&mut self, pair: CriticalPair) -> bool {
        let key = normalize_pair_key(pair.i(), pair.j());

        if self.present.insert(key) {
            self.pairs.push(pair);
            true
        } else {
            false
        }
    }

    /// Consumes the pending set and returns its pairs.
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<CriticalPair> {
        self.pairs
    }

    /// Removes all pending pairs.
    #[inline]
    pub fn clear(&mut self) {
        self.pairs.clear();
        self.present.clear();
    }

    /// Removes and returns all pending pairs.
    pub fn drain_all(&mut self) -> Vec<CriticalPair> {
        self.present.clear();
        std::mem::take(&mut self.pairs)
    }

    /// Replaces the pending set with `pairs`.
    ///
    /// Duplicate unordered index pairs are collapsed, preserving the first
    /// occurrence.
    pub fn replace(&mut self, pairs: Vec<CriticalPair>) {
        self.clear();

        for pair in pairs {
            self.insert(pair);
        }
    }
}

#[inline]
#[must_use]
fn normalize_pair_key(i: usize, j: usize) -> (usize, usize) {
    if i < j { (i, j) } else { (j, i) }
}

/// Construct all admissible critical pairs from the current basis.
pub fn build_all_pairs<P, C>(basis: &[P], criterion: &C) -> Result<Vec<CriticalPair>>
where
    P: PolynomialView,
    C: PairCriterion<P>,
{
    let mut pairs = Vec::new();

    for i in 0..basis.len() {
        for j in (i + 1)..basis.len() {
            if let Some(pair) = make_pair(basis, i, j)? {
                if criterion.allows(basis, &pair) {
                    pairs.push(pair);
                }
            }
        }
    }

    Ok(pairs)
}

/// Inserts all admissible pairs involving `new_index`.
///
/// This is used after appending a new basis element.
pub fn add_pairs_with_new_basis_element<P, C>(basis: &[P], new_index: usize, pending: &mut PendingPairs, criterion: &C) -> Result<()>
where
    P: PolynomialView,
    C: PairCriterion<P>,
{
    for old_index in 0..new_index {
        if let Some(pair) = make_pair(basis, old_index, new_index)? {
            if criterion.allows(basis, &pair) {
                pending.insert(pair);
            }
        }
    }

    Ok(())
}

/// Constructs a critical pair from two basis indices.
///
/// Returns `Ok(None)` when the indices are equal or either basis element is
/// zero.
pub fn make_pair<P>(basis: &[P], i: usize, j: usize) -> Result<Option<CriticalPair>>
where
    P: PolynomialView,
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::algos::f4::pairs::criterion::{NoCriterion, ProductCriterion};
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    #[test]
    fn insert_deduplicates_unordered_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let basis = vec![f1, f2];

        let lm1 = basis[0].leading_mono().unwrap();
        let lm2 = basis[1].leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        let mut pending = PendingPairs::new();

        assert!(pending.insert(pair.clone()));
        assert!(!pending.insert(pair));

        assert_eq!(pending.len(), 1);
        assert!(pending.contains(0, 1));
        assert!(pending.contains(1, 0));
    }

    #[test]
    fn drain_all_empties_pending_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();

        let basis = vec![f1, f2];

        let pair = make_pair(&basis, 0, 1).unwrap().unwrap();

        let mut pending = PendingPairs::new();
        pending.insert(pair);

        let drained = pending.drain_all();

        assert_eq!(drained.len(), 1);
        assert!(pending.is_empty());
        assert_eq!(pending.len(), 0);
        assert!(!pending.contains(0, 1));
    }

    #[test]
    fn make_pair_returns_none_for_equal_indices() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let basis = vec![f];

        let pair = make_pair(&basis, 0, 0).unwrap();

        assert!(pair.is_none());
    }

    #[test]
    fn build_all_pairs_respects_no_criterion() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 1])].unwrap();

        let basis = vec![f1, f2, f3];

        let pairs = build_all_pairs(&basis, &NoCriterion).unwrap();

        assert_eq!(pairs.len(), 3);
    }

    #[test]
    fn build_all_pairs_respects_product_criterion() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 1])].unwrap();

        let basis = vec![f1, f2, f3];

        let pairs = build_all_pairs(&basis, &ProductCriterion).unwrap();

        // (x^2, y) is rejected because gcd = 1.
        // (x^2, xy) and (xy, y) are kept.
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn add_pairs_with_new_basis_element_adds_only_new_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 1])].unwrap();

        let basis = vec![f1, f2, f3];

        let mut pending = PendingPairs::new();

        add_pairs_with_new_basis_element(&basis, 2, &mut pending, &NoCriterion).unwrap();

        assert_eq!(pending.len(), 2);
        assert!(pending.contains(0, 2));
        assert!(pending.contains(1, 2));
        assert!(!pending.contains(0, 1));
    }
}
