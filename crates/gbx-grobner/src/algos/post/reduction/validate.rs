use crate::algos::post::PostError;
use crate::f4_debug;

use gbx_poly::monomial::{divides, Monomial};
use gbx_poly::polynomial::PolynomialView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ReductionViolation {
    pub(crate) i: usize,
    pub(crate) j: usize,
    pub(crate) bad_term: Monomial,
}

/// Ensures no basis leading monomial is divisible by another.
pub(crate) fn assert_minimal_leading_monomials<P>(basis: &[P]) -> Result<(), PostError>
where
    P: PolynomialView,
{
    for (i, gi) in basis.iter().enumerate() {
        let Some(lmi) = gi.leading_mono() else {
            continue;
        };

        for (j, gj) in basis.iter().enumerate() {
            if i == j {
                continue;
            }

            let Some(lmj) = gj.leading_mono() else {
                continue;
            };

            if divides(lmj, lmi) {
                f4_debug!(
                    i,
                    j,
                    lm_i = ?lmi,
                    lm_j = ?lmj,
                    "post.reduce.minimality_violation"
                );

                return Err(PostError::InvariantViolation);
            }
        }
    }

    Ok(())
}

/// Returns tail terms that are reducible by another basis leading monomial.
pub(crate) fn find_reduction_violations<P>(basis: &[P]) -> Vec<ReductionViolation>
where
    P: PolynomialView,
{
    let mut violations = Vec::new();

    for (i, gi) in basis.iter().enumerate() {
        if gi.is_zero() {
            continue;
        }

        for term in gi.terms().iter().skip(1) {
            for (j, gj) in basis.iter().enumerate() {
                if i == j || gj.is_zero() {
                    continue;
                }

                let Some(lmj) = gj.leading_mono() else {
                    continue;
                };

                if divides(lmj, term.mono()) {
                    violations.push(ReductionViolation { i, j, bad_term: term.mono().clone() });
                }
            }
        }
    }

    violations
}

/// Ensures every basis tail is reduced against all other leading monomials.
pub(crate) fn assert_fully_reduced_basis<P>(basis: &[P]) -> Result<(), PostError>
where
    P: PolynomialView,
{
    let violations = find_reduction_violations(basis);

    if violations.is_empty() {
        return Ok(());
    }

    let _first = &violations[0];

    f4_debug!(
        violations = violations.len(),
        i = _first.i,
        j = _first.j,
        bad_term = ?_first.bad_term,
        "post.reduce.not_fully_reduced"
    );

    Err(PostError::InvariantViolation)
}
