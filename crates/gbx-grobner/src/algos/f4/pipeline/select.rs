use crate::algos::f4::pairs::critical_pair::CriticalPair;
use crate::algos::f4::pairs::selector::{MinDegreeSelector, PairSelector};
use crate::algos::f4::state::F4State;
use crate::instrumentation::{f4_debug, f4_span};

use crate::symbolic::{SymbolicProduct, SymbolicSource};
use gbx_poly::monomial::{Monomial, MonomialAlgos};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

pub struct PairSelection<M> {
    pub selected_pairs: Vec<CriticalPair<M>>, // P_d
    pub l_d: Vec<SymbolicProduct<M>>,         // L_d
}

#[cfg_attr(feature = "instrumentation", tracing::instrument(level = "debug", skip(state, selector)))]
pub fn select_pairs_phase<P>(state: &mut F4State<P>, selector: &mut MinDegreeSelector) -> PairSelection<<P::Term as TermView>::Mono>
where
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Clone + Monomial + MonomialAlgos,
{
    let selected_pairs = {
        f4_span!("select_pairs");
        let pairs = state.pending.drain_all();
        let selection = selector.select(pairs);
        state.pending.replace(selection.remaining);
        selection.selected
    };

    f4_debug!(
        selected_pairs = selected_pairs.len(),
        pending_remaining = state.pending.len(),
        "selected pairs"
    );

    let l_d = {
        f4_span!("build_l_d");
        build_l_d(&selected_pairs)
    };

    f4_debug!(
        selected_pairs = selected_pairs.len(),
        l_d_len = l_d.len(),
        "built l_d"
    );

    PairSelection { selected_pairs, l_d }
}

fn build_l_d<M>(pairs: &[CriticalPair<M>]) -> Vec<SymbolicProduct<M>>
where
    M: Clone + Monomial + MonomialAlgos,
{
    let mut l_d = Vec::with_capacity(pairs.len() * 2);

    for pair in pairs {
        let left = pair.left();
        let right = pair.right();

        l_d.push(SymbolicProduct { source: SymbolicSource::Basis(left.basis_index), multiplier: left.multiplier.clone() });

        l_d.push(SymbolicProduct { source: SymbolicSource::Basis(right.basis_index), multiplier: right.multiplier.clone() });
    }

    l_d
}
