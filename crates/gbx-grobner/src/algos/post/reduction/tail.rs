use crate::algos::post::PostError;
use crate::{GrobnerBasis, f4_debug, f4_info};

use gbx_poly::monomial::{Monomial, divides};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::Term;

use super::validate::find_reduction_violations;

/// Reduces basis elements in reverse order while preserving leading monomials.
pub fn reverse_tail_interreduce<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let snapshot = gb.as_slice().to_vec();
    let n = snapshot.len();

    let mut reduced_rev = Vec::with_capacity(n);

    f4_info!(len = n, "post.reduce.reverse_interreduce.start");

    for i in (0..n).rev() {
        let gi = &snapshot[i];

        if gi.is_zero() {
            continue;
        }

        let old_lm = gi
            .leading_mono()
            .cloned()
            .ok_or(PostError::InvariantViolation)?;

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

pub fn selective_tail_cleanup<P, F, O>(ctx: &RingCtx<F, O>, gb: &mut GrobnerBasis<P>) -> Result<(), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let mut polys = gb.as_slice().to_vec();
    let max_passes = 8usize;

    for _pass in 0..max_passes {
        let violations = find_reduction_violations(&polys);

        f4_info!(
            _pass,
            violations = violations.len(),
            "post.reduce.selective_cleanup.pass.start"
        );

        if violations.is_empty() {
            *gb = GrobnerBasis::new(ctx.id(), polys);
            f4_info!(_pass, "post.reduce.selective_cleanup.done");
            return Ok(());
        }

        // Use a stable snapshot so each pass reduces against one fixed basis.
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

            let mut reduced = reduce_basis_element_tail_only_with_selected_indices(ctx, &snapshot[i], &snapshot, i, &selected)?;

            reduced.normalize_in_place(ctx)?;

            let old_lm = snapshot[i].leading_mono().cloned();
            let new_lm = reduced.leading_mono().cloned();

            if old_lm != new_lm {
                f4_debug!(
                    _pass,
                    i,
                    old_lm = ?old_lm,
                    new_lm = ?new_lm,
                    "post.reduce.selective_cleanup.leading_monomial_changed"
                );

                return Err(PostError::LeadingMonomialChangedDuringReduction);
            }

            polys[i] = reduced;
            changed += 1;
        }

        f4_info!(_pass, changed, "post.reduce.selective_cleanup.pass.done");

        if changed == 0 {
            break;
        }
    }

    *gb = GrobnerBasis::new(ctx.id(), polys);

    let violations = find_reduction_violations(gb.as_slice());

    if !violations.is_empty() {
        let _first = &violations[0];

        f4_debug!(
            violations = violations.len(),
            i = _first.i,
            j = _first.j,
            bad_term = ?_first.bad_term,
            "post.reduce.selective_cleanup.failed"
        );

        return Err(PostError::InvariantViolation);
    }

    Ok(())
}

fn reduce_basis_element_tail_only_with_reducers<P, F, O>(ctx: &RingCtx<F, O>, gi: &P, reducer_basis: &[P], i: usize) -> Result<P, PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let (lm, lt, tail) = split_leading_term(ctx, gi)?;

    if tail.is_zero() {
        return Ok(gi.clone());
    }

    let mut reducers: Vec<&P> = reducer_basis.iter().filter(|g| !g.is_zero()).collect();

    reducers.sort_by_key(|g| g.len());

    f4_debug!(
        i,
        lm = ?lm,
        tail_terms = tail.len(),
        reducers = reducers.len(),
        "post.reduce.tail.before_normal_form"
    );

    let reduced_tail = tail.normal_form(ctx, reducers)?;

    finish_tail_reduction(ctx, gi, &lm, lt, reduced_tail, i)
}

fn reduce_basis_element_tail_only_with_selected_indices<P, F, O>(ctx: &RingCtx<F, O>, gi: &P, basis: &[P], i: usize, selected: &[usize]) -> Result<P, PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialReduce + PolynomialView + PolynomialMut + Clone,
    P::Coeff: Copy + Eq,
{
    let (lm, lt, tail) = split_leading_term(ctx, gi)?;

    if tail.is_zero() || selected.is_empty() {
        return Ok(gi.clone());
    }

    let mut reducers: Vec<&P> = selected
        .iter()
        .filter_map(|&j| {
            if j == i {
                return None;
            }

            let g = &basis[j];

            (!g.is_zero()).then_some(g)
        })
        .collect();

    reducers.sort_by_key(|g| g.len());

    f4_debug!(
        i,
        lm = ?lm,
        tail_terms = tail.len(),
        selected_reducers = reducers.len(),
        "post.reduce.selective.tail.before_normal_form"
    );

    let reduced_tail = tail.normal_form(ctx, reducers)?;

    finish_tail_reduction(ctx, gi, &lm, lt, reduced_tail, i)
}

fn split_leading_term<P, F, O>(ctx: &RingCtx<F, O>, p: &P) -> Result<(Monomial, Term<P::Coeff>, P), PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialView + PolynomialMut + Clone + PolynomialOps,
    P::Coeff: Copy + Eq,
{
    let lm = p
        .leading_mono()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    let lt = p
        .leading_term()
        .cloned()
        .ok_or(PostError::ZeroPolynomialInReduction)?;

    let mut tail = p.clone();

    tail.pop_leading_term_raw(ctx)?;
    tail.normalize_in_place(ctx)?;

    Ok((lm, lt, tail))
}

fn finish_tail_reduction<P, F, O>(ctx: &RingCtx<F, O>, _original: &P, previous_leading_mono: &Monomial, previous_leading_term: Term<P::Coeff>, reduced_tail: P, _i: usize) -> Result<P, PostError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialView + Clone,
    P::Coeff: Copy + Eq,
{
    let mut result = reduced_tail;
    let (lt_coeff, lt_mono) = previous_leading_term.into_parts();

    result.push_term_raw(Term::new(lt_coeff, lt_mono));
    result.normalize_in_place(ctx)?;

    let new_lm = result.leading_mono().cloned();

    if new_lm.as_ref() != Some(previous_leading_mono) {
        f4_debug!(
            _i,
            old_lm = ?previous_leading_mono,
            new_lm = ?new_lm,
            original_terms = _original.len(),
            result_terms = result.len(),
            "post.reduce.tail.leading_monomial_changed"
        );

        return Err(PostError::LeadingMonomialChangedDuringReduction);
    }

    Ok(result)
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

        let useful = gi
            .terms()
            .iter()
            .skip(1)
            .any(|term| divides(lmj, term.mono()));

        if useful {
            selected.push(j);
        }
    }

    selected
}
