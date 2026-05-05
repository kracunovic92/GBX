//! Polynomial reduction / normal form (context-driven).
//!
//! Computes the normal form (remainder) of `f` with respect to a set of reducers.
//!
//! # Design
//!
//! This module supports two reduction entry points:
//!
//! - [`PolynomialReduce::normal_form`] for convenience when the caller has raw
//!   reducer polynomials.
//! - [`PolynomialReduce::normal_form_prepared`] for the hot path, where the
//!   caller supplies already-prepared reducer metadata.
//!
//! Reducer preparation / caching is intentionally handled outside this module.

use crate::monomial::{checked_quotient, Monomial, MonomialError};
use crate::order::MonomialOrder;
use crate::polynomial::error::PolynomialError;
use crate::polynomial::ops::PolynomialOps;
use crate::polynomial::PolynomialView;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::Term;
use thiserror::Error;

/// Errors that can occur during polynomial reduction.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum ReduceError {
    /// Propagated polynomial/term/monomial/ring error.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// Attempted to divide by a non-invertible leading coefficient of a reducer.
    #[error("cannot reduce: reducer leading coefficient is not invertible")]
    NonInvertibleLeadingCoefficient,
}

impl From<MonomialError> for ReduceError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        Self::Poly(PolynomialError::from(e))
    }
}

/// Reduction / normal form operations.
#[allow(missing_docs)]
pub trait PolynomialReduce: PolynomialOps
where
    Self: Sized + Clone,
    Self::Coeff: Copy + Eq,
{
    fn normal_form<'a, F, O, I>(&self, ctx: &RingCtx<F, O>, gs: I) -> Result<Self, ReduceError>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
        I: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        let prepared = gs
            .into_iter()
            .filter(|g| !g.is_zero())
            .map(|g| {
                let lt_g = g
                    .leading_term()
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                let inv_lc = ctx
                    .field
                    .try_inv(*lt_g.coeff())
                    .ok_or(ReduceError::NonInvertibleLeadingCoefficient)?;

                Ok((g, lt_g.mono(), inv_lc, lt_g.mono().degree()))
            })
            .collect::<Result<Vec<_>, ReduceError>>()?;

        self.normal_form_prepared(ctx, prepared)
    }

    fn normal_form_prepared<'a, F, O, I>(&self, ctx: &RingCtx<F, O>, reducers: I) -> Result<Self, ReduceError>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: MonomialOrder,
        I: IntoIterator<Item = (&'a Self, &'a Monomial, Self::Coeff, u32)>,
        Self: 'a,
    {
        let mut reducers: Vec<_> = reducers.into_iter().collect();
        reducers.sort_by_key(|(_, _, _, degree)| *degree);

        let mut f = self.clone();
        f.normalize_in_place(ctx).map_err(ReduceError::from)?;

        if reducers.is_empty() {
            return Ok(f);
        }

        let mut r = Self::zero_in(ctx);

        let mut steps = 0usize;
        let max_steps = 1_000_000usize;

        while let Some((lt_coeff, lt_mono, lt_degree)) = f
            .leading_term()
            .map(|lt| (*lt.coeff(), lt.mono().clone(), lt.mono().degree()))
        {
            steps += 1;

            if steps % 10_000 == 0 {
                tracing::warn!(
                    steps,
                    f_terms = f.len(),
                    r_terms = r.len(),
                    lt = ?lt_mono,
                    lt_degree,
                    "normal_form.progress"
                );
            }

            if steps > max_steps {
                tracing::error!(
                    steps,
                    f_terms = f.len(),
                    r_terms = r.len(),
                    lt = ?lt_mono,
                    lt_degree,
                    "normal_form.stalled"
                );

                return Err(ReduceError::Poly(PolynomialError::InvariantViolation));
            }

            let mut reduced = false;

            for (reducer_index, (g, lm_g, inv_lc_g, lm_degree_g)) in reducers.iter().enumerate() {
                if *lm_degree_g > lt_degree {
                    break;
                }

                let Some(q_m) = checked_quotient(lm_g, &lt_mono)? else {
                    continue;
                };

                let q_c = ctx.field.mul(lt_coeff, *inv_lc_g);

                tracing::trace!(
                    steps,
                    reducer_index,
                    f_terms_before = f.len(),
                    lt = ?lt_mono,
                    reducer_lm = ?lm_g,
                    quotient_mono = ?q_m,
                    "normal_form.reduce_step"
                );

                let _ = f
                    .pop_leading_term_raw(ctx)
                    .map_err(ReduceError::from)?
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                let old_terms = f.len();

                f = sub_scaled_monomial_multiple_tail_canonical(ctx, &f, g, &q_m, q_c)?;

                tracing::trace!(
                    steps,
                    reducer_index,
                    f_terms_before_tail_sub = old_terms,
                    f_terms_after = f.len(),
                    new_lt = ?f.leading_mono(),
                    "normal_form.after_tail_sub"
                );

                reduced = true;
                break;
            }

            if !reduced {
                let lt = f
                    .pop_leading_term_raw(ctx)
                    .map_err(ReduceError::from)?
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                tracing::trace!(
                    steps,
                    moved_mono = ?lt.mono(),
                    f_terms_after_pop = f.len(),
                    r_terms_before = r.len(),
                    "normal_form.move_to_remainder"
                );

                r.push_term_raw(lt);
            }
        }

        tracing::debug!(steps, r_terms = r.len(), "normal_form.done");

        Ok(r)
    }
}

/// Compute
///
/// `f_tail - scalar * monomial * tail(g)`
///
/// assuming:
///
/// - `f_tail` is canonical and is `f` after removing its leading term,
/// - `g` is canonical,
/// - the leading term of `monomial * g` already canceled externally.
///
/// Therefore, this merge skips the leading term of `g`.
fn sub_scaled_monomial_multiple_tail_canonical<P, F, O>(ctx: &RingCtx<F, O>, f_tail: &P, g: &P, mul_mono: &Monomial, scalar: P::Coeff) -> Result<P, ReduceError>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq,
{
    let mut out = P::zero_in(ctx);

    let f_terms = f_tail.terms();
    let g_terms = g.terms();

    let mut i = 0usize;
    let mut j = 1usize;

    while i < f_terms.len() || j < g_terms.len() {
        match (f_terms.get(i), g_terms.get(j)) {
            (Some(tf), Some(tg)) => {
                let mg = mul_mono.checked_mul(tg.mono())?;
                let cmp = ctx.order.cmp(tf.mono(), &mg);

                if cmp.is_gt() {
                    out.push_term_raw(tf.clone());
                    i += 1;
                } else if cmp.is_lt() {
                    let scaled = ctx.field.mul(scalar, *tg.coeff());
                    let neg_scaled = ctx.field.neg(scaled);

                    if !ctx.field.is_zero(neg_scaled) {
                        out.push_term_raw(Term::new(neg_scaled, mg));
                    }

                    j += 1;
                } else {
                    let scaled = ctx.field.mul(scalar, *tg.coeff());
                    let coeff = ctx.field.sub(*tf.coeff(), scaled);

                    if !ctx.field.is_zero(coeff) {
                        out.push_term_raw(Term::new(coeff, tf.mono().clone()));
                    }

                    i += 1;
                    j += 1;
                }
            }

            (Some(tf), None) => {
                out.push_term_raw(tf.clone());
                i += 1;
            }

            (None, Some(tg)) => {
                let mg = mul_mono.checked_mul(tg.mono())?;
                let scaled = ctx.field.mul(scalar, *tg.coeff());
                let neg_scaled = ctx.field.neg(scaled);

                if !ctx.field.is_zero(neg_scaled) {
                    out.push_term_raw(Term::new(neg_scaled, mg));
                }

                j += 1;
            }

            (None, None) => break,
        }
    }

    Ok(out)
}

impl<P> PolynomialReduce for P
where
    P: PolynomialOps + Clone,
    P::Coeff: Copy + Eq,
{
}
