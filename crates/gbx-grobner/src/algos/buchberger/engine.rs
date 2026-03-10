use super::bounds::BuchbergerTerm;
use super::init::init_basis;
use super::options::{BasisPost, BuchbergerOptions};
use super::trace::{measure_duration, SharedTracer};
use crate::algos::{minimize_in_place, reduce_in_place};
use crate::{s_polynomial_in, BuchbergerError, GrobnerBasis, PairQueue, PairUpdate};
use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialOps, PolynomialReduce, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::TermView;
use std::time::Instant;

pub(crate) fn run_buchberger<P, F, O, Q, U>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: BuchbergerOptions, pairs: Q, update: U) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, None)
}

pub(crate) fn run_buchberger_traced<P, F, O, Q, U>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    pairs: Q,
    update: U,
    tracer: SharedTracer,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    run_buchberger_impl(ctx, fs, opts, pairs, update, Some(tracer))
}

fn run_buchberger_impl<P, F, O, Q, U>(
    ctx: &RingCtx<F, O>,
    fs: impl IntoIterator<Item = P>,
    opts: BuchbergerOptions,
    mut pairs: Q,
    mut update: U,
    tracer: Option<SharedTracer>,
) -> Result<GrobnerBasis<P>, BuchbergerError>
where
    O: MonomialOrder,
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    P: PolynomialOps + PolynomialReduce + Clone,
    P::Term: BuchbergerTerm,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    Q: PairQueue,
    U: PairUpdate<P>,
{
    let init_t0 = Instant::now();
    let mut gb = init_basis(ctx, fs, opts)?;
    if let Some(tr) = &tracer {
        let mut tr = tr.borrow_mut();
        tr.phases.init += init_t0.elapsed();
        tr.set_initial_basis_len(gb.len());
    }

    if gb.is_empty() {
        if let Some(tr) = &tracer {
            let snap = tr
                .borrow()
                .make_snapshot(0, pairs.len(), Some(0), Some(0), true);
            tr.borrow().print_summary(&snap);
        }
        return Ok(gb);
    }

    let seed_t0 = Instant::now();
    for k in 0..gb.len() {
        let t0 = Instant::now();
        update.on_new_poly(&gb, &mut pairs, k);
        let dt = t0.elapsed();

        if let Some(tr) = &tracer {
            let mut tr = tr.borrow_mut();
            tr.while_times.pair_update += dt;
            tr.on_seeded_pair(pairs.len());
        }
    }
    if let Some(tr) = &tracer {
        tr.borrow_mut().phases.seed += seed_t0.elapsed();
    }

    let while_t0 = Instant::now();
    while let Some((_key, i, j)) = pairs.pop() {
        let fi = gb.get(i).ok_or(BuchbergerError::InvariantViolation)?;
        let fj = gb.get(j).ok_or(BuchbergerError::InvariantViolation)?;

        let s = if let Some(tr) = &tracer {
            measure_duration(&mut tr.borrow_mut().while_times.s_poly, || {
                s_polynomial_in(ctx, fi, fj)
            })?
        } else {
            s_polynomial_in(ctx, fi, fj)?
        };

        let mut r = if let Some(tr) = &tracer {
            measure_duration(&mut tr.borrow_mut().while_times.normal_form, || {
                s.normal_form(ctx, gb.as_slice().iter())
            })?
        } else {
            s.normal_form(ctx, gb.as_slice().iter())?
        };

        if opts.normalize_remainders {
            if let Some(tr) = &tracer {
                measure_duration(&mut tr.borrow_mut().while_times.remainder_normalize, || {
                    r.normalize_in_place(ctx)
                })?;
            } else {
                r.normalize_in_place(ctx)?;
            }
        }

        if r.is_zero() {
            if let Some(tr) = &tracer {
                tr.borrow_mut().on_zero_reduction();
            }
            continue;
        }

        if r.is_nonzero_constant() {
            if !opts.normalize_remainders {
                if let Some(tr) = &tracer {
                    measure_duration(&mut tr.borrow_mut().while_times.remainder_normalize, || {
                        r.normalize_in_place(ctx)
                    })?;
                } else {
                    r.normalize_in_place(ctx)?;
                }
            }

            if let Some(tr) = &tracer {
                {
                    let mut tr = tr.borrow_mut();
                    tr.on_unit_reduction();
                    tr.phases.while_loop += while_t0.elapsed();
                }
                let snap = tr.borrow().make_snapshot(1, pairs.len(), None, None, true);
                tr.borrow().print_summary(&snap);
            }

            return Ok(GrobnerBasis::new(ctx.id(), vec![r]));
        }

        let new_idx = gb.len();
        gb.push(r);

        if let Some(tr) = &tracer {
            tr.borrow_mut().on_inserted_poly(gb.len(), pairs.len());
        }

        if let Some(tr) = &tracer {
            let t0 = Instant::now();
            update.on_new_poly(&gb, &mut pairs, new_idx);
            let dt = t0.elapsed();
            tr.borrow_mut().while_times.pair_update += dt;
        } else {
            update.on_new_poly(&gb, &mut pairs, new_idx);
        }

        if let Some(tr) = &tracer {
            let basis_term_count = gb.as_slice().iter().map(|p| p.len()).sum::<usize>();
            let max_poly_terms = gb.as_slice().iter().map(|p| p.len()).max().unwrap_or(0);
            let snap = tr.borrow().make_snapshot(
                gb.len(),
                pairs.len(),
                Some(basis_term_count),
                Some(max_poly_terms),
                true,
            );
            tr.borrow_mut().maybe_print_progress(&snap);
        }
    }

    if let Some(tr) = &tracer {
        tr.borrow_mut().phases.while_loop += while_t0.elapsed();
    }

    let post_t0 = Instant::now();
    match opts.post {
        BasisPost::None => {}
        BasisPost::Minimal => minimize_in_place(ctx, &mut gb)?,
        BasisPost::Reduced => reduce_in_place(ctx, &mut gb)?,
    }

    if let Some(tr) = &tracer {
        {
            let mut tr = tr.borrow_mut();
            tr.phases.post += post_t0.elapsed();
            tr.set_gb_len(gb.len());
        }

        let basis_term_count = gb.as_slice().iter().map(|p| p.len()).sum::<usize>();
        let max_poly_terms = gb.as_slice().iter().map(|p| p.len()).max().unwrap_or(0);
        let snap = tr.borrow().make_snapshot(
            gb.len(),
            pairs.len(),
            Some(basis_term_count),
            Some(max_poly_terms),
            true,
        );
        tr.borrow().print_summary(&snap);
    }

    Ok(gb)
}
