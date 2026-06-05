//! Interreduction of computed Gröbner bases.
//!
//! The routines in this module turn a generated Gröbner basis into a canonical
//! reduced basis by removing redundant leading monomials and reducing basis
//! tails against the remaining elements.

mod tail;
mod validate;

use std::time::{Duration, Instant};

use crate::algos::post::PostError;
use crate::algos::post::minimize::minimize_in_place;
use crate::{GrobnerBasis, f4_debug, f4_info};

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

use self::tail::{reverse_tail_interreduce, selective_tail_cleanup};
use self::validate::{assert_fully_reduced_basis, assert_minimal_leading_monomials, find_reduction_violations};

/// Interreduces a Gröbner basis in place.
///
/// The resulting basis is minimal, monic, and tail-reduced with respect to its
/// own leading monomials. Tail reduction preserves the leading monomial of each
/// surviving basis element.
///
/// # Errors
///
/// Returns an error if canonicalization, minimization, tail reduction, or final
/// validation fails.
pub fn reduce_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let started = Instant::now();
    let input_len = gb.len();

    remove_zero_polys(gb);

    if gb.is_empty() {
        log_reduce_done(input_len, 0, 0, started.elapsed());
        return Ok(());
    }

    canonicalize_basis(ctx, gb)?;
    minimize_in_place(ctx, gb)?;

    assert_minimal_leading_monomials(gb.as_slice())?;

    reverse_tail_interreduce(ctx, gb)?;

    remove_zero_polys(gb);
    canonicalize_basis(ctx, gb)?;

    let violations_after_reverse = find_reduction_violations(gb.as_slice()).len();

    if violations_after_reverse != 0 {
        f4_debug!(
            violations = violations_after_reverse,
            "post.reduce.after_reverse_interreduce.has_violations"
        );

        selective_tail_cleanup(ctx, gb)?;
        canonicalize_basis(ctx, gb)?;
    }

    assert_fully_reduced_basis(gb.as_slice())?;

    log_reduce_done(
        input_len,
        gb.len(),
        violations_after_reverse,
        started.elapsed(),
    );

    Ok(())
}

fn remove_zero_polys<P>(gb: &mut GrobnerBasis<P>)
where
    P: PolynomialView,
{
    gb.retain(|p| !p.is_zero());
}

fn canonicalize_basis<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialView,
    P::Coeff: Copy + Eq,
{
    for p in gb.as_mut_vec() {
        p.normalize_in_place(ctx)?;
    }

    Ok(())
}

fn log_reduce_done(_input_len: usize, _final_len: usize, _violations_after_reverse: usize, _elapsed: Duration) {
    f4_info!(
        _input_len,
        _final_len,
        _violations_after_reverse,
        elapsed_ms = _elapsed.as_secs_f64() * 1000.0,
        "post.reduce.done"
    );
}

#[cfg(test)]
mod tests;
