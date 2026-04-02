use crate::algos::f4::error::{F4Error, Result};
use crate::algos::f4::linear::reducer::BatchReducer;
use crate::algos::f4::symbolic::ordered::OrderedMono;

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialView};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};

/// Current correctness-first F4 linear reducer.
///
/// This performs echelon-style reduction directly on polynomial rows.
/// It is a placeholder for a future matrix-based reducer.
#[derive(Debug, Default, Clone, Copy)]
pub struct PolynomialEchelonReducer;

impl<P, F, O> BatchReducer<P, F, O> for PolynomialEchelonReducer
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Default,
{
    fn reduce(&self, ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>> {
        echelon_reduce(ctx, rows)
    }
}

/// Reduce symbolic rows into echelon-style pivot rows.
pub fn echelon_reduce<P, F, O>(ctx: &RingCtx<F, O>, rows: &[P]) -> Result<Vec<P>>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
    <<P as PolynomialView>::Term as TermView>::Mono: Default,
{
    let order = &ctx.order;

    let mut work: Vec<P> = rows.to_vec();
    sort_rows_by_leading_mono_desc(&mut work, order);

    let mut pivots: Vec<P> = Vec::new();

    for mut row in work {
        reduce_by_pivots(ctx, &mut row, &pivots)?;

        if row.is_zero() {
            continue;
        }

        make_monic(ctx, &mut row)?;
        pivots.push(row);
    }

    // Backward cleanup: reduce earlier pivots by later pivots.
    for i in (0..pivots.len()).rev() {
        let later: Vec<P> = pivots.iter().skip(i + 1).cloned().collect();
        reduce_by_pivots(ctx, &mut pivots[i], &later)?;

        if !pivots[i].is_zero() {
            make_monic(ctx, &mut pivots[i])?;
        }
    }

    sort_rows_by_leading_mono_desc(&mut pivots, order);
    Ok(pivots)
}

fn sort_rows_by_leading_mono_desc<P, O>(rows: &mut [P], order: &O)
where
    O: MonomialOrder,
    P: PolynomialView,
    P::Term: TermView,
    <P::Term as TermView>::Mono: Clone + MonomialView<Word = u32>,
{
    rows.sort_by(
        |a, b| match (a.leading_mono().cloned(), b.leading_mono().cloned()) {
            (Some(ma), Some(mb)) => OrderedMono::new(mb, order).cmp(&OrderedMono::new(ma, order)),
            (Some(_), None) => core::cmp::Ordering::Less,
            (None, Some(_)) => core::cmp::Ordering::Greater,
            (None, None) => core::cmp::Ordering::Equal,
        },
    );
}

fn make_monic<P, F, O>(ctx: &RingCtx<F, O>, row: &mut P) -> Result<()>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq + Default,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    let Some(lt) = row.leading_term() else {
        return Ok(());
    };

    let lc = *lt.coeff();
    let Some(inv) = ctx.field.try_inv(lc) else {
        return Err(F4Error::NonInvertibleLeadingCoefficient);
    };

    row.scale_in_place(ctx, inv)?;
    Ok(())
}

fn reduce_by_pivots<P, F, O>(ctx: &RingCtx<F, O>, row: &mut P, pivots: &[P]) -> Result<()>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <<P as PolynomialView>::Term as TermView>::Coeff: Default,
{
    loop {
        let Some(row_lt) = row.leading_term().cloned() else {
            break;
        };

        let mut changed = false;

        for pivot in pivots {
            let Some(pivot_lt) = pivot.leading_term() else {
                continue;
            };

            if pivot_lt.mono().divides(row_lt.mono()) {
                let mono_mul = row_lt.mono().checked_div_exact_by(pivot_lt.mono())?;

                let Some(inv_lc_pivot) = ctx.field.try_inv(*pivot_lt.coeff()) else {
                    return Err(F4Error::NonInvertibleLeadingCoefficient);
                };

                let coeff_mul = ctx.field.mul(*row_lt.coeff(), inv_lc_pivot);

                row.sub_scaled_monomial_multiple_in_place(ctx, pivot, &mono_mul, coeff_mul)?;
                row.normalize_in_place(ctx)?;
                changed = true;
                break;
            }
        }

        if !changed {
            break;
        }
    }

    row.normalize_in_place(ctx)?;
    Ok(())
}
