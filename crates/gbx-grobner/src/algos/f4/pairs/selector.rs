//! Critical-pair batch selection.
use crate::algos::f4::pairs::critical_pair::CriticalPair;

/// Result of selecting one F4 batch from the current pending critical pairs.
#[derive(Debug, Clone)]
pub struct Selection {
    /// Pairs selected for immediate processing.
    pub selected: Vec<CriticalPair>,

    /// Pairs left pending for future iterations.
    pub remaining: Vec<CriticalPair>,
}

/// Strategy for choosing the next batch of critical pairs.
pub trait PairSelector {
    /// Split pending pairs into:
    /// - selected batch,
    /// - remaining pairs.
    fn select(&mut self, pairs: Vec<CriticalPair>) -> Selection;
}

/// Select all critical pairs of minimal lcm degree, optionally capped by batch size.
///
/// This is the usual F4 normal strategy:
/// process critical pairs degree-by-degree, always taking the smallest pending
/// lcm degree first.
#[derive(Debug, Clone, Copy)]
pub struct MinDegreeSelector {
    batch_size: usize,
}

impl MinDegreeSelector {
    /// Creates a selector with a maximum selected batch size.
    #[inline]
    #[must_use]
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size: batch_size.max(1) }
    }

    /// Returns the maximum selected batch size.
    #[inline]
    #[must_use]
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

impl Default for MinDegreeSelector {
    fn default() -> Self {
        Self { batch_size: usize::MAX }
    }
}

impl PairSelector for MinDegreeSelector {
    fn select(&mut self, pairs: Vec<CriticalPair>) -> Selection {
        if pairs.is_empty() {
            return Selection { selected: Vec::new(), remaining: Vec::new() };
        }

        let min_degree = pairs
            .iter()
            .map(CriticalPair::degree)
            .min()
            .expect("non-empty pair list must have a minimum degree");

        let mut selected = Vec::new();
        let mut remaining = Vec::new();

        for pair in pairs {
            if pair.degree() == min_degree && selected.len() < self.batch_size() {
                selected.push(pair);
            } else {
                remaining.push(pair);
            }
        }
        debug_assert!(
            !selected.is_empty() || remaining.is_empty(),
            "MinDegreeSelector stalled: selected is empty but remaining is non-empty"
        );

        Selection { selected, remaining }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::order::{Grevlex, Lex};
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    fn pair_from_polys(i: usize, j: usize, f: &P, g: &P) -> CriticalPair {
        let lm_f = f.leading_mono().expect("polynomial should be nonzero");
        let lm_g = g.leading_mono().expect("polynomial should be nonzero");

        CriticalPair::from_lms(i, j, lm_f, lm_g).expect("pair construction should succeed")
    }

    #[test]
    fn select_empty_returns_empty_selection() {
        let mut selector = MinDegreeSelector::new(8);
        let selection = selector.select(Vec::new());

        assert!(selection.selected.is_empty());
        assert!(selection.remaining.is_empty());
    }

    #[test]
    fn select_min_degree_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 2])].unwrap();
        let f4: P = poly![&ring; (1, [3, 0])].unwrap();

        let pairs = vec![
            pair_from_polys(0, 1, &f1, &f2), // lcm degree 3
            pair_from_polys(0, 2, &f1, &f3), // lcm degree 4
            pair_from_polys(1, 2, &f2, &f3), // lcm degree 3
            pair_from_polys(0, 3, &f1, &f4), // lcm degree 3
        ];

        let mut selector = MinDegreeSelector::default();
        let selection = selector.select(pairs);

        assert_eq!(selection.selected.len(), 3);
        assert!(selection.selected.iter().all(|p| p.degree() == 3));

        assert_eq!(selection.remaining.len(), 1);
        assert!(selection.remaining.iter().all(|p| p.degree() != 3));
    }

    #[test]
    fn select_respects_batch_size_cap() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1])].unwrap();
        let f3: P = poly![&ring; (1, [0, 2])].unwrap();

        let pairs = vec![
            pair_from_polys(0, 1, &f1, &f2), // degree 3
            pair_from_polys(1, 2, &f2, &f3), // degree 3
            pair_from_polys(0, 2, &f1, &f3), // degree 4
        ];

        let mut selector = MinDegreeSelector::new(1);
        let selection = selector.select(pairs);

        assert_eq!(selection.selected.len(), 1);
        assert_eq!(selection.selected[0].degree(), 3);

        assert_eq!(selection.remaining.len(), 2);
        assert!(selection.remaining.iter().any(|p| p.degree() == 3));
        assert!(selection.remaining.iter().any(|p| p.degree() == 4));
    }

    #[test]
    fn selector_uses_pair_lcm_degree_not_basis_order() {
        let lex_ring = test_ring(7, 2, Lex).expect("lex ring construction should succeed");
        let grevlex_ring = test_ring(7, 2, Grevlex).expect("grevlex ring construction should succeed");

        // Same input shape, but order changes LM(f1).
        //
        // Lex:
        //   LM(x^2 + y^5) = x^2
        //   pair with x has lcm degree 2
        //
        // Grevlex:
        //   LM(x^2 + y^5) = y^5
        //   pair with x has lcm degree 6
        let f1_lex: P = poly![&lex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_lex: P = poly![&lex_ring; (1, [1, 0])].unwrap();

        let f1_grevlex: P = poly![&grevlex_ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2_grevlex: P = poly![&grevlex_ring; (1, [1, 0])].unwrap();

        let pair_lex = pair_from_polys(0, 1, &f1_lex, &f2_lex);
        let pair_grevlex = pair_from_polys(0, 1, &f1_grevlex, &f2_grevlex);

        assert_eq!(pair_lex.degree(), 2);
        assert_eq!(pair_grevlex.degree(), 6);
    }

    #[test]
    fn default_selector_has_unbounded_batch_size() {
        let selector = MinDegreeSelector::default();

        assert_eq!(selector.batch_size(), usize::MAX);
    }
}
