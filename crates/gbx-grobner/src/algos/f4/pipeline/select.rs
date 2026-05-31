use crate::algos::f4::pairs::critical_pair::CriticalPair;
use crate::algos::f4::pairs::selector::{MinDegreeSelector, PairSelector};
use crate::algos::f4::state::F4State;

use crate::pairs::PairSide;
use crate::symbolic::{ProductSource, UnevaluatedProduct};
use gbx_poly::monomial::Monomial;
use gbx_poly::polynomial::PolynomialView;

/// Output of the F4 pair-selection stage.
///
/// `selected_pairs` is the selected critical-pair batch `P_d`.
/// `l_d` is `L_d = Left(P_d) ∪ Right(P_d)`.
pub struct SelectedPairBatch {
    pub selected_pairs: Vec<CriticalPair>,
    pub l_d: Vec<UnevaluatedProduct<Monomial>>,
}

/// Selects one F4 batch from pending critical pairs.
///
/// This stage computes `P_d := Sel(P)`, removes those pairs from `P`,
/// and builds `L_d = Left(P_d) ∪ Right(P_d)` as unevaluated products.
pub fn select_pairs_phase<P>(state: &mut F4State<P>, selector: &mut MinDegreeSelector) -> SelectedPairBatch
where
    P: PolynomialView,
{
    let selected_pairs = {
        let pairs = state.pending.drain_all();
        let selection = selector.select(pairs);

        state.pending.replace(selection.remaining);

        selection.selected
    };

    let l_d = build_l_d(&selected_pairs);

    SelectedPairBatch { selected_pairs, l_d }
}

/// Builds `L_d = Left(P_d) ∪ Right(P_d)` from selected critical pairs.
fn build_l_d(pairs: &[CriticalPair]) -> Vec<UnevaluatedProduct<Monomial>> {
    let mut l_d = Vec::with_capacity(pairs.len() * 2);

    for pair in pairs {
        l_d.push(pair_side_to_product(pair.left()));
        l_d.push(pair_side_to_product(pair.right()));
    }

    l_d
}
fn pair_side_to_product(side: PairSide<'_>) -> UnevaluatedProduct<Monomial> {
    UnevaluatedProduct { source: ProductSource::Basis(side.basis_index), multiplier: side.multiplier.clone() }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    use gbx_field::fp::FpElem;
    use gbx_poly::monomial::MonomialView;
    use gbx_poly::polynomial::Polynomial;

    type P = Polynomial<FpElem>;

    fn m(exps: &[u32]) -> Monomial {
        Monomial::from_slice(exps)
    }

    fn pair(i: usize, j: usize, lcm: &[u32], degree: u32, ti: &[u32], tj: &[u32]) -> CriticalPair {
        CriticalPair::new_for_test(i, j, m(lcm), degree, m(ti), m(tj))
    }

    #[test]
    fn build_l_d_creates_two_symbolic_products_per_pair() {
        let p = pair(0, 1, &[2, 1], 3, &[0, 1], &[1, 0]);

        let l_d = build_l_d(&[p]);

        assert_eq!(l_d.len(), 2);

        match l_d[0].source {
            ProductSource::Basis(i) => assert_eq!(i, 0),
            ProductSource::HistoryReducedRow { .. } => panic!("expected basis source"),
        }

        match l_d[1].source {
            ProductSource::Basis(i) => assert_eq!(i, 1),
            ProductSource::HistoryReducedRow { .. } => panic!("expected basis source"),
        }

        assert_eq!(l_d[0].multiplier.exponents(), &[0, 1]);
        assert_eq!(l_d[1].multiplier.exponents(), &[1, 0]);
    }

    #[test]
    fn select_pairs_phase_selects_min_degree_pairs_and_keeps_remaining() {
        let mut state = F4State::<P>::new();

        let p01 = pair(0, 1, &[2, 1], 3, &[0, 1], &[1, 0]);
        let p02 = pair(0, 2, &[2, 2], 4, &[0, 2], &[2, 0]);
        let p12 = pair(1, 2, &[1, 2], 3, &[0, 1], &[1, 0]);

        assert!(state.pending.insert(p01));
        assert!(state.pending.insert(p02));
        assert!(state.pending.insert(p12));

        let mut selector = MinDegreeSelector::default();

        let selection = select_pairs_phase(&mut state, &mut selector);

        assert_eq!(selection.selected_pairs.len(), 2);
        assert!(selection.selected_pairs.iter().all(|p| p.degree() == 3));

        assert_eq!(selection.l_d.len(), 4);

        assert_eq!(state.pending.len(), 1);
        assert!(state.pending.as_slice().iter().all(|p| p.degree() == 4));
    }

    #[test]
    fn select_pairs_phase_respects_batch_size() {
        let mut state = F4State::<P>::new();

        let p01 = pair(0, 1, &[2, 1], 3, &[0, 1], &[1, 0]);
        let p12 = pair(1, 2, &[1, 2], 3, &[0, 1], &[1, 0]);
        let p02 = pair(0, 2, &[2, 2], 4, &[0, 2], &[2, 0]);

        assert!(state.pending.insert(p01));
        assert!(state.pending.insert(p12));
        assert!(state.pending.insert(p02));

        let mut selector = MinDegreeSelector::new(1);

        let selection = select_pairs_phase(&mut state, &mut selector);

        assert_eq!(selection.selected_pairs.len(), 1);
        assert_eq!(selection.selected_pairs[0].degree(), 3);

        assert_eq!(selection.l_d.len(), 2);

        assert_eq!(state.pending.len(), 2);
        assert!(state.pending.as_slice().iter().any(|p| p.degree() == 3));
        assert!(state.pending.as_slice().iter().any(|p| p.degree() == 4));
    }

    #[test]
    fn select_pairs_phase_handles_empty_pending_set() {
        let mut state = F4State::<P>::new();
        let mut selector = MinDegreeSelector::default();

        let selection = select_pairs_phase(&mut state, &mut selector);

        assert!(selection.selected_pairs.is_empty());
        assert!(selection.l_d.is_empty());
        assert!(state.pending.is_empty());
    }
}
