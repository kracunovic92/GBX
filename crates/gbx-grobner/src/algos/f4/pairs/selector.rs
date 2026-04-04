use crate::algos::f4::pairs::critical_pair::CriticalPair;

/// Result of selecting one F4 batch from the current pending critical pairs.
#[derive(Debug, Clone)]
pub struct Selection<M> {
    /// Pairs selected for immediate processing.
    pub selected: Vec<CriticalPair<M>>,
    /// Pairs left pending for future iterations.
    pub remaining: Vec<CriticalPair<M>>,
}

/// Strategy for choosing the next batch of critical pairs.
pub trait PairSelector<M> {
    /// Split the pending pairs into:
    /// - a batch to process now,
    /// - the pairs that remain pending.
    fn select(&mut self, pairs: Vec<CriticalPair<M>>) -> Selection<M>;
}

/// Select all critical pairs of minimal lcm degree, optionally capped by batch size.
///
/// This corresponds to the usual F4 “normal strategy”:
/// process pairs degree-by-degree, always taking the smallest currently pending degree first.
#[derive(Debug, Clone, Copy)]
pub struct MinDegreeSelector {
    batch_size: usize,
}

impl MinDegreeSelector {
    #[must_use]
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

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

impl<M> PairSelector<M> for MinDegreeSelector {
    fn select(&mut self, pairs: Vec<CriticalPair<M>>) -> Selection<M> {
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
            if pair.degree() == min_degree && selected.len() < self.batch_size {
                selected.push(pair);
            } else {
                remaining.push(pair);
            }
        }

        Selection { selected, remaining }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_ring;
    use gbx_field::fp::FpDynElem;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{PolyDyn, PolynomialView};
    use gbx_poly::term::TermView;

    type P = PolyDyn<FpDynElem>;
    type Mono = <<P as PolynomialView>::Term as TermView>::Mono;
    fn pair_from_polys(i: usize, j: usize, f: &P, g: &P) -> CriticalPair<<<P as PolynomialView>::Term as gbx_poly::term::TermView>::Mono> {
        let lm_f = f.leading_mono().expect("polynomial should be nonzero");
        let lm_g = g.leading_mono().expect("polynomial should be nonzero");
        CriticalPair::from_lms(i, j, lm_f, lm_g).expect("pair construction should succeed")
    }

    #[test]
    fn select_empty_returns_empty_selection() {
        let mut selector = MinDegreeSelector::new(8);
        let selection = selector.select(Vec::<CriticalPair<Mono>>::new());

        assert!(selection.selected.is_empty());
        assert!(selection.remaining.is_empty());
    }

    #[test]
    fn select_min_degree_pairs() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0])].unwrap(); // x^2, degree 2
        let f2: P = poly![&ring; (1, [1, 1])].unwrap(); // x y, degree 2
        let f3: P = poly![&ring; (1, [0, 2])].unwrap(); // y^2, degree 2
        let f4: P = poly![&ring; (1, [3, 0])].unwrap(); // x^3, degree 3

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

        let f1: P = poly![&ring; (1, [2, 0])].unwrap(); // x^2
        let f2: P = poly![&ring; (1, [1, 1])].unwrap(); // x y
        let f3: P = poly![&ring; (1, [0, 2])].unwrap(); // y^2

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
    }

    #[test]
    fn default_selector_has_unbounded_batch_size() {
        let selector = MinDegreeSelector::default();
        assert_eq!(selector.batch_size(), usize::MAX);
    }
}
