//! Structural snapshots for F4 state.
//!
//! These snapshots are intentionally small. They are meant to answer questions
//! like:
//!
//! - how large is the basis?
//! - how many pending pairs exist?
//! - how many terms are currently stored in the basis?
//!
//! They are not full object dumps.

use crate::state::F4State;
use gbx_poly::polynomial::PolynomialView;

#[cfg_attr(feature = "profile-dump", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct F4StateSnapshot {
    pub phase: &'static str,
    pub iteration: usize,

    pub basis_len: usize,
    pub pending_pairs: usize,
    pub history_len: usize,

    pub basis_terms: usize,
}

#[cfg_attr(feature = "profile-dump", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct F4IterationSnapshot {
    pub phase: &'static str,
    pub iteration: usize,

    pub selected_pairs: usize,
    pub l_d_len: usize,

    pub basis_len: usize,
    pub pending_pairs: usize,
    pub history_len: usize,
}

#[cfg_attr(feature = "profile-dump", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct F4ReductionSnapshot {
    pub phase: &'static str,
    pub iteration: usize,

    pub symbolic_rows: usize,
    pub symbolic_terms: usize,

    pub reduced_rows: usize,
    pub reduced_terms: usize,

    pub extracted_rows: usize,
    pub extracted_terms: usize,
}

pub fn snapshot_state<P>(phase: &'static str, iteration: usize, state: &F4State<P>) -> F4StateSnapshot
where
    P: PolynomialView,
{
    F4StateSnapshot { phase, iteration, basis_len: state.basis.len(), pending_pairs: state.pending.len(), history_len: state.history.len(), basis_terms: count_polynomial_terms(&state.basis) }
}

pub fn count_polynomial_terms<P>(polys: &[P]) -> usize
where
    P: PolynomialView,
{
    polys.iter().map(|p| p.len()).sum()
}
