use crate::algos::post::minimize::minimize_in_place;
use crate::algos::post::PostError;
use crate::{f4_debug, f4_info, f4_span, GrobnerBasis};
use gbx_poly::monomial::{divides, Monomial};
use std::time::Instant;

use crate::algos::post::types::PostReduceStats;
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::Term;

#[derive(Debug, Clone)]
struct ReductionViolation {
    i: usize,
    j: usize,
    bad_term: Monomial,
}

/// Fully reduce a Gröbner basis in place.
///
/// This performs:
/// 1. removal of zero basis elements,
/// 2. canonicalization,
/// 3. minimization,
/// 4. reverse tail-interreduction,
/// 5. final canonicalization and validation.
pub fn reduce_in_place<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let started = Instant::now();
    let mut stats = PostReduceStats { input_len: gb.len(), ..Default::default() };

    remove_zero_polys(gb);
    stats.after_remove_zero_len = gb.len();

    if gb.is_empty() {
        stats.elapsed = started.elapsed();
        f4_info!(
            input_len = stats.input_len,
            final_len = 0,
            elapsed_ms = stats.elapsed.as_secs_f64() * 1000.0,
            "post.reduce.done.empty"
        );
        return Ok(());
    }

    canonicalize_basis(ctx, gb)?;
    minimize_in_place(ctx, gb)?;

    stats.after_minimize_len = gb.len();
    assert_minimal_leading_monomials(gb.as_slice())?;

    reverse_tail_interreduce(ctx, gb)?;

    remove_zero_polys(gb);
    canonicalize_basis(ctx, gb)?;

    let violations = find_reduction_violations(gb.as_slice());

    if !violations.is_empty() {
        f4_debug!(
            violations = violations.len(),
            "post.reduce.after_reverse_interreduce.has_violations"
        );
        selective_tail_cleanup(ctx, gb)?;
        canonicalize_basis(ctx, gb)?;
    }

    let final_violations = find_reduction_violations(gb.as_slice());
    stats.final_violations = final_violations.len();

    if !final_violations.is_empty() {
        let first = &final_violations[0];

        f4_debug!(
            violations = final_violations.len(),
            i = first.i,
            j = first.j,
            bad_term = ?first.bad_term,
            "post.reduce.not_fully_reduced"
        );

        return Err(PostError::InvariantViolation);
    }

    stats.final_len = gb.len();
    stats.elapsed = started.elapsed();

    f4_info!(
        input_len = stats.input_len,
        after_remove_zero_len = stats.after_remove_zero_len,
        after_minimize_len = stats.after_minimize_len,
        after_reverse_len = stats.after_reverse_len,
        final_len = stats.final_len,
        reverse_changed = stats.reverse_changed,
        selective_passes = stats.selective_passes,
        selective_changed = stats.selective_changed,
        violations_after_reverse = stats.initial_violations_after_reverse,
        final_violations = stats.final_violations,
        elapsed_ms = stats.elapsed.as_secs_f64() * 1000.0,
        "post.reduce.done"
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

fn reverse_tail_interreduce<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let snapshot = gb.as_slice().to_vec();
    let n = snapshot.len();

    let mut reduced_rev: Vec<P> = Vec::with_capacity(n);

    f4_info!(len = n, "post.reduce.reverse_interreduce.start");

    for i in (0..n).rev() {
        let gi = &snapshot[i];

        if gi.is_zero() {
            f4_debug!(i, "post.reduce.reverse_interreduce.skip_zero");
            continue;
        }

        let old_lm = gi
            .leading_mono()
            .cloned()
            .ok_or(PostError::InvariantViolation)?;

        f4_debug!(
            i,
            lm = ?old_lm,
            terms = gi.len(),
            reducers = reduced_rev.len(),
            "post.reduce.reverse_interreduce.element.start"
        );

        let mut r = reduce_basis_element_tail_only_with_reducers(ctx, gi, reduced_rev.as_slice(), i)?;

        if r.is_zero() {
            f4_debug!(
                i,
                old_lm = ?old_lm,
                "post.reduce.reverse_interreduce.element.became_zero"
            );
            continue;
        }

        r.normalize_in_place(ctx)?;

        let new_lm = r
            .leading_mono()
            .cloned()
            .ok_or(PostError::InvariantViolation)?;

        if old_lm != new_lm {
            f4_debug!(
                i,
                old_lm = ?old_lm,
                new_lm = ?new_lm,
                old_terms = gi.len(),
                new_terms = r.len(),
                "post.reduce.reverse_interreduce.leading_monomial_changed"
            );

            return Err(PostError::LeadingMonomialChangedDuringReduction);
        }

        f4_debug!(
            i,
            lm = ?new_lm,
            old_terms = gi.len(),
            new_terms = r.len(),
            "post.reduce.reverse_interreduce.element.done"
        );

        reduced_rev.push(r);
    }

    reduced_rev.reverse();

    f4_info!(
        before = n,
        after = reduced_rev.len(),
        "post.reduce.reverse_interreduce.done"
    );

    *gb = GrobnerBasis::new(ctx.id(), reduced_rev);

    Ok(())
}

fn reduce_basis_element_tail_only_with_reducers<P, F, O>(ctx: &RingCtx<F, O>, gi: &P, reducer_basis: &[P], i: usize) -> Result<P, PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let lm = gi
        .leading_mono()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    let lt = gi
        .leading_term()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    f4_debug!(
        i,
        lm = ?lm,
        terms = gi.len(),
        "post.reduce.tail.start"
    );

    let mut tail = gi.clone();

    tail.pop_leading_term_raw(ctx)?;
    tail.normalize_in_place(ctx)?;

    if tail.is_zero() {
        f4_debug!(
            i,
            lm = ?lm,
            "post.reduce.tail.empty"
        );

        return Ok(gi.clone());
    }

    let mut reducers: Vec<&P> = reducer_basis.iter().filter(|g| !g.is_zero()).collect();

    /*
        Keep this heuristic local.

        Later we can experiment with:
        - degree first,
        - leading monomial order,
        - term count,
        - reverse basis order.
    */
    reducers.sort_by_key(|g| g.len());

    f4_debug!(
        i,
        lm = ?lm,
        tail_terms = tail.len(),
        reducers = reducers.len(),
        "post.reduce.tail.before_normal_form"
    );

    for (reducer_pos, g) in reducers.iter().enumerate() {
        f4_debug!(
            i,
            reducer_pos,
            reducer_lm = ?g.leading_mono(),
            reducer_terms = g.len(),
            "post.reduce.tail.reducer"
        );
    }

    let reduced_tail = tail.normal_form(ctx, reducers.into_iter())?;

    f4_debug!(
        i,
        lm = ?lm,
        before_terms = tail.len(),
        after_terms = reduced_tail.len(),
        after_zero = reduced_tail.is_zero(),
        "post.reduce.tail.after_normal_form"
    );

    let mut result = reduced_tail;

    let (lt_coeff, lt_mono) = lt.into_parts();

    result.push_term_raw(Term::new(lt_coeff, lt_mono));
    result.normalize_in_place(ctx)?;

    let new_lm = result.leading_mono().cloned();

    if new_lm.as_ref() != Some(&lm) {
        f4_debug!(
            i,
            old_lm = ?lm,
            new_lm = ?new_lm,
            original_terms = gi.len(),
            tail_terms = tail.len(),
            result_terms = result.len(),
            "post.reduce.tail.leading_monomial_changed"
        );

        return Err(PostError::LeadingMonomialChangedDuringReduction);
    }

    f4_debug!(
        i,
        lm = ?lm,
        result_terms = result.len(),
        "post.reduce.tail.done"
    );

    Ok(result)
}
fn debug_basis_lms<P>(label: &'static str, basis: &[P])
where
    P: PolynomialView,
{
    f4_debug!(label, len = basis.len(), "post.reduce.basis_lms.start");

    for (i, p) in basis.iter().enumerate() {
        f4_debug!(
            label,
            i,
            lm = ?p.leading_mono(),
            terms = p.len(),
            zero = p.is_zero(),
            "post.reduce.basis_lm"
        );
    }

    f4_debug!(label, len = basis.len(), "post.reduce.basis_lms.end");
}
fn assert_minimal_leading_monomials<P>(basis: &[P]) -> Result<(), PostError>
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
fn find_reduction_violations<P>(basis: &[P]) -> Vec<ReductionViolation>
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

fn assert_fully_reduced_basis<P>(basis: &[P]) -> Result<(), PostError>
where
    P: PolynomialView,
{
    let violations = find_reduction_violations(basis);

    if violations.is_empty() {
        return Ok(());
    }

    let first = &violations[0];

    f4_debug!(
        violations = violations.len(),
        i = first.i,
        j = first.j,
        bad_term = ?first.bad_term,
        "post.reduce.not_fully_reduced"
    );

    Err(PostError::InvariantViolation)
}

fn selected_tail_reducer_indices<P>(basis: &[P], i: usize) -> Vec<usize>
where
    P: PolynomialView,
{
    let gi = &basis[i];

    if gi.is_zero() {
        return Vec::new();
    }

    let mut selected = Vec::new();

    for (j, gj) in basis.iter().enumerate() {
        if i == j || gj.is_zero() {
            continue;
        }

        let Some(lmj) = gj.leading_mono() else {
            continue;
        };

        let useful = gi.terms().iter().skip(1).any(|t| divides(lmj, t.mono()));

        if useful {
            selected.push(j);
        }
    }

    selected
}
fn reduce_basis_element_tail_only_with_selected_indices<P, F, O>(ctx: &RingCtx<F, O>, gi: &P, basis: &[P], i: usize, selected: &[usize]) -> Result<P, PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let lm = gi
        .leading_mono()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    let lt = gi
        .leading_term()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    let mut tail = gi.clone();

    tail.pop_leading_term_raw(ctx)?;
    tail.normalize_in_place(ctx)?;

    if tail.is_zero() || selected.is_empty() {
        return Ok(gi.clone());
    }

    let mut reducers: Vec<(usize, &P)> = selected
        .iter()
        .filter_map(|&j| {
            if j == i {
                return None;
            }

            let g = &basis[j];

            if g.is_zero() { None } else { Some((j, g)) }
        })
        .collect();

    reducers.sort_by_key(|(_, g)| g.len());

    f4_debug!(
        i,
        lm = ?lm,
        tail_terms = tail.len(),
        selected_reducers = reducers.len(),
        "post.reduce.selective.tail.before_normal_form"
    );

    for (j, g) in &reducers {
        f4_debug!(
            i,
            reducer = *j,
            reducer_lm = ?g.leading_mono(),
            reducer_terms = g.len(),
            "post.reduce.selective.tail.reducer"
        );
    }

    let reduced_tail = tail.normal_form(ctx, reducers.iter().map(|(_, g)| *g))?;

    f4_debug!(
        i,
        lm = ?lm,
        before_terms = tail.len(),
        after_terms = reduced_tail.len(),
        after_zero = reduced_tail.is_zero(),
        "post.reduce.selective.tail.after_normal_form"
    );

    let mut result = reduced_tail;

    let (lt_coeff, lt_mono) = lt.into_parts();

    result.push_term_raw(Term::new(lt_coeff, lt_mono));
    result.normalize_in_place(ctx)?;

    let new_lm = result.leading_mono().cloned();

    if new_lm.as_ref() != Some(&lm) {
        f4_debug!(
            i,
            old_lm = ?lm,
            new_lm = ?new_lm,
            selected_reducers = selected.len(),
            result_terms = result.len(),
            "post.reduce.selective.tail.leading_monomial_changed"
        );

        return Err(PostError::LeadingMonomialChangedDuringReduction);
    }

    Ok(result)
}
fn selective_tail_cleanup<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let mut polys = gb.as_slice().to_vec();

    let max_passes = 8usize;

    for pass in 0..max_passes {
        let violations = find_reduction_violations(&polys);

        f4_info!(
            pass,
            violations = violations.len(),
            "post.reduce.selective_cleanup.pass.start"
        );

        if violations.is_empty() {
            *gb = GrobnerBasis::new(ctx.id(), polys);

            f4_info!(pass, "post.reduce.selective_cleanup.done");

            return Ok(());
        }

        let snapshot = polys.clone();

        let mut changed = 0usize;

        for i in 0..snapshot.len() {
            if snapshot[i].is_zero() {
                continue;
            }

            let selected = selected_tail_reducer_indices(&snapshot, i);

            if selected.is_empty() {
                continue;
            }

            f4_debug!(
                pass,
                i,
                selected_reducers = ?selected,
                "post.reduce.selective_cleanup.element.start"
            );

            let mut reduced = reduce_basis_element_tail_only_with_selected_indices(ctx, &snapshot[i], &snapshot, i, &selected)?;

            reduced.normalize_in_place(ctx)?;

            let old_lm = snapshot[i].leading_mono().cloned();
            let new_lm = reduced.leading_mono().cloned();

            if old_lm != new_lm {
                f4_debug!(
                    pass,
                    i,
                    old_lm = ?old_lm,
                    new_lm = ?new_lm,
                    "post.reduce.selective_cleanup.leading_monomial_changed"
                );

                return Err(PostError::LeadingMonomialChangedDuringReduction);
            }

            let old_terms = snapshot[i].len();
            let new_terms = reduced.len();

            polys[i] = reduced;
            changed += 1;

            f4_debug!(
                pass,
                i,
                old_terms,
                new_terms,
                "post.reduce.selective_cleanup.element.done"
            );
        }

        f4_info!(pass, changed, "post.reduce.selective_cleanup.pass.done");

        if changed == 0 {
            break;
        }
    }

    *gb = GrobnerBasis::new(ctx.id(), polys);

    let violations = find_reduction_violations(gb.as_slice());

    if !violations.is_empty() {
        let first = &violations[0];

        f4_debug!(
            violations = violations.len(),
            i = first.i,
            j = first.j,
            bad_term = ?first.bad_term,
            "post.reduce.selective_cleanup.failed"
        );

        return Err(PostError::InvariantViolation);
    }

    Ok(())
}
