//! Shared helper functions for Buchberger pair management.
//!
//! This module contains small, policy-free utilities used by pairing,
//! pair-key strategies, and pair-update strategies.
//!
//! The helpers here should remain lightweight and reusable.
//!
//! Those decisions belong to higher-level components implementing
//! [`PairCriterion`](PairCriterion),
//! [`PairKey`](PairKey),
//! and [`PairUpdate`](crate::pairing::PairUpdate).

use crate::pairing::filters::{PairFilter, PairSetView};
use crate::pairing::{PairCriterion, PairKey};
use crate::{GrobnerBasis, Pair, PairQueue};
use gbx_poly::polynomial::PolynomialView;
use gbx_poly::term::TermView;

/// Returns the leading monomial of `gb[index]`.
///
/// If the index is out of bounds or the polynomial is zero, returns `None`.
///
/// # Parameters
///
/// - `gb`: current Gröbner basis
/// - `index`: basis position
///
/// # Returns
///
/// - `Some(&LM(gb[index]))` if the polynomial exists and is nonzero
/// - `None` otherwise
#[inline]
pub fn leading_mono_at<P>(gb: &GrobnerBasis<P>, index: usize) -> Option<&<P::Term as TermView>::Mono>
where
    P: PolynomialView,
    P::Term: TermView,
{
    gb.get(index)?.leading_term().map(|t| t.mono())
}

/// Seed all candidate pairs `(i, new_index)` with `i < new_index`.
///
/// This is the generic seeding loop used by simple pair-update strategies.
/// For each old basis element `i`, it:
///
/// # Parameters
///
/// - `gb`: current Gröbner basis after the new polynomial was appended
/// - `pairs`: active pair queue / store
/// - `new_index`: index of the newly appended basis element
/// - `criterion`: pair-level criterion
/// - `keyer`: queue-key strategy
///
/// # Preconditions
///
/// The caller should ensure that `new_index < gb.len()`.
/// This is debug-asserted in debug builds.
#[inline]
pub fn seed_pairs<P, Q, C, F, K>(gb: &GrobnerBasis<P>, pairs: &mut Q, new_index: usize, criterion: &mut C, filter: &mut F, keyer: &mut K)
where
    Q: PairQueue + PairSetView,
    C: PairCriterion<P>,
    F: PairFilter<P>,
    K: PairKey<P, Key = Q::Key>,
{
    for i in 0..new_index {
        if !criterion.keep_pair(gb, i, new_index) {
            continue;
        }

        if !filter.keep_pair(gb, pairs, i, new_index) {
            continue;
        }

        let Some(key) = keyer.key_for_pair(gb, i, new_index) else {
            continue;
        };

        pairs.push(Pair::new(key, i, new_index));
    }
}
