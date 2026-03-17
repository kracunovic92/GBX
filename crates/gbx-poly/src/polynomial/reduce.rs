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

use crate::monomial::{Monomial, MonomialAlgos, MonomialError, MonomialView};
use crate::order::MonomialOrder;
use crate::polynomial::error::PolynomialError;
use crate::polynomial::ops::PolynomialOps;
use crate::polynomial::traits::PolynomialMut;
use crate::polynomial::PolynomialView;
use crate::ring::{FieldCtx, RingCtx};
use crate::term::{TermOwned, TermView};
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
pub trait PolynomialReduce: PolynomialOps
where
    Self: Sized + Clone,
    Self::Term: TermOwned + TermView + Clone,
    <Self::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <Self::Term as TermView>::Coeff: Copy + Eq,
{
    /// Compute the normal form (remainder) of `self` w.r.t. raw reducer polynomials.
    ///
    /// This is a convenience wrapper. Hot-path Gröbner code should prefer
    /// [`Self::normal_form_prepared`].
    fn normal_form<'a, F, O, I>(&self, ctx: &RingCtx<F, O>, gs: I) -> Result<Self, ReduceError>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
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

                Ok((
                    g,
                    lt_g.mono(),
                    inv_lc,
                    lt_g.mono()
                        .degree_hint()
                        .expect("Always carry total degree"),
                ))
            })
            .collect::<Result<Vec<_>, ReduceError>>()?;

        self.normal_form_prepared(ctx, prepared)
    }

    /// Compute the normal form using already-prepared reducers.
    ///
    /// Each reducer is supplied as:
    ///
    /// - `&Self`: reducer polynomial
    /// - `&Mono`: cached leading monomial
    /// - `Coeff`: cached inverse leading coefficient
    /// - `u32`: cached degree of the leading monomial
    ///
    /// This avoids rebuilding reducer metadata inside reduction.
    fn normal_form_prepared<'a, F, O, I>(&self, ctx: &RingCtx<F, O>, reducers: I) -> Result<Self, ReduceError>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
        I: IntoIterator<
            Item = (
                &'a Self,
                &'a <Self::Term as TermView>::Mono,
                <Self::Term as TermView>::Coeff,
                u32,
            ),
        >,
        Self: 'a,
    {
        let reducers: Vec<_> = reducers.into_iter().collect();

        let mut f = self.clone();
        f.normalize_in_place(ctx).map_err(ReduceError::from)?;

        if reducers.is_empty() {
            return Ok(f);
        }

        let mut r = Self::zero_in(ctx);

        while let Some((lt_coeff, lt_mono, lt_degree)) = f.leading_term().map(|lt| {
            (
                *lt.coeff(),
                lt.mono().clone(),
                lt.mono().degree_hint().expect("Always carry total degree"),
            )
        }) {
            let mut reduced = false;

            for (g, lm_g, inv_lc_g, lm_degree_g) in &reducers {
                if *lm_degree_g > lt_degree {
                    continue;
                }

                let Some(q_m) = lt_mono.checked_div_by(lm_g)? else {
                    continue;
                };

                let q_c = ctx.field.mul(lt_coeff, *inv_lc_g);

                // Remove lt(f) first because it is guaranteed to cancel with
                // q * lt(g). Then merge only tails.
                let _ = f
                    .pop_leading_term_raw(ctx)
                    .map_err(ReduceError::from)?
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                f = sub_scaled_monomial_multiple_tail_canonical(ctx, &f, g, &q_m, q_c)?;
                reduced = true;
                break;
            }

            if !reduced {
                let lt = f
                    .pop_leading_term_raw(ctx)
                    .map_err(ReduceError::from)?
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                <Self as PolynomialMut>::push_term_raw(&mut r, lt);
            }
        }

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
/// Therefore this merge skips the leading term of `g`.
fn sub_scaled_monomial_multiple_tail_canonical<P, F, O>(ctx: &RingCtx<F, O>, f_tail: &P, g: &P, mul_mono: &<P::Term as TermView>::Mono, scalar: <P::Term as TermView>::Coeff) -> Result<P, ReduceError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
{
    let mut out = P::zero_in(ctx);

    let f_terms = f_tail.terms();
    let g_terms = g.terms();

    let mut i = 0usize;
    let mut j = 1usize; // skip lt(g): it already canceled

    while i < f_terms.len() || j < g_terms.len() {
        match (f_terms.get(i), g_terms.get(j)) {
            (Some(tf), Some(tg)) => {
                let mg = mul_mono.checked_mul(tg.mono())?;
                let cmp = ctx.order.cmp(tf.mono(), &mg);

                if cmp.is_gt() {
                    <P as PolynomialMut>::push_term_raw(&mut out, tf.clone());
                    i += 1;
                } else if cmp.is_lt() {
                    let scaled = ctx.field.mul(scalar, *tg.coeff());
                    let neg_scaled = ctx.field.sub(ctx.field.zero(), scaled);

                    if !ctx.field.is_zero(neg_scaled) {
                        <P as PolynomialMut>::push_term_raw(&mut out, <P::Term as TermOwned>::from_parts(neg_scaled, mg));
                    }
                    j += 1;
                } else {
                    let scaled = ctx.field.mul(scalar, *tg.coeff());
                    let coeff = ctx.field.sub(*tf.coeff(), scaled);

                    if !ctx.field.is_zero(coeff) {
                        <P as PolynomialMut>::push_term_raw(
                            &mut out,
                            <P::Term as TermOwned>::from_parts(coeff, tf.mono().clone()),
                        );
                    }

                    i += 1;
                    j += 1;
                }
            }

            (Some(tf), None) => {
                <P as PolynomialMut>::push_term_raw(&mut out, tf.clone());
                i += 1;
            }

            (None, Some(tg)) => {
                let mg = mul_mono.checked_mul(tg.mono())?;
                let scaled = ctx.field.mul(scalar, *tg.coeff());
                let neg_scaled = ctx.field.sub(ctx.field.zero(), scaled);

                if !ctx.field.is_zero(neg_scaled) {
                    <P as PolynomialMut>::push_term_raw(&mut out, <P::Term as TermOwned>::from_parts(neg_scaled, mg));
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
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + MonomialView<Word = u32> + Clone + Eq,
    <P::Term as TermView>::Coeff: Copy + Eq,
{
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::{PolynomialReduce, ReduceError};

    use crate::monomial::{DynamicMonomial, FixedMonomial};
    use crate::order::Lex;
    use crate::polynomial::poly::Polynomial;
    use crate::polynomial::traits::{PolynomialMut, PolynomialView};
    use crate::ring::{FieldCtx, Ring, StaticFpCtx};
    use crate::term::{Term, TermView};
    use gbx_field::fp::{Fp, FpDyn, FpDynElem};
    use gbx_storage::polynomial::VecTerms;

    type F7 = Fp<7>;
    type T2 = Term<F7, FixedMonomial<2>>;
    type P2 = Polynomial<T2, VecTerms<T2>>;

    fn ring_static() -> crate::ring::RingCtx<StaticFpCtx<7>, Lex> {
        Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap()
    }

    fn t2(c: u32, e0: u32, e1: u32) -> T2 {
        Term::new(F7::new(c), FixedMonomial::from_exponents([e0, e1]))
    }

    fn p2(r: &crate::ring::RingCtx<StaticFpCtx<7>, Lex>, terms: &[(u32, u32, u32)]) -> P2 {
        let ts = terms.iter().map(|&(c, a, b)| t2(c, a, b)).collect();
        P2::from_terms_in(r, ts).unwrap()
    }

    type TD = Term<FpDynElem, DynamicMonomial>;
    type PD = Polynomial<TD, VecTerms<TD>>;

    fn ring_dyn() -> crate::ring::RingCtx<FpDyn, Lex> {
        Ring::builder()
            .field(FpDyn::prime(7).unwrap())
            .order(Lex)
            .nvars(3)
            .build()
            .unwrap()
    }

    fn td(r: &crate::ring::RingCtx<FpDyn, Lex>, c: u32, exps: &[u32]) -> TD {
        Term::new(r.field.new(c), DynamicMonomial::from_slice(exps))
    }

    fn pd(r: &crate::ring::RingCtx<FpDyn, Lex>, terms: &[(u32, &[u32])]) -> PD {
        let ts = terms.iter().map(|&(c, e)| td(r, c, e)).collect();
        PD::from_terms_in(r, ts).unwrap()
    }

    #[test]
    fn normal_form_empty_set_is_identity_static() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 1, 0), (1, 0, 0)]);

        let r = f.normal_form(&ring, core::iter::empty()).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn normal_form_empty_set_is_identity_dynamic() {
        let ring = ring_dyn();
        let f = pd(&ring, &[(1, &[2, 0, 0]), (1, &[1, 0, 0]), (1, &[0, 0, 0])]);

        let r = f.normal_form(&ring, core::iter::empty()).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn zero_reducers_are_ignored() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (3, 0, 0)]);
        let g0 = P2::zero_in(&ring);

        let r = f.normal_form(&ring, core::iter::once(&g0)).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn reducer_with_non_dividing_lm_is_ignored() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 1, 0), (2, 0, 0)]);
        let g = p2(&ring, &[(1, 0, 1), (1, 0, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn reduces_to_zero_by_x_static() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 1, 0)]);
        let g = p2(&ring, &[(1, 1, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert!(r.is_zero());
    }

    #[test]
    fn reduces_to_zero_dynamic() {
        let ring = ring_dyn();
        let f = pd(&ring, &[(1, &[2, 0, 0]), (1, &[1, 0, 0])]);
        let g = pd(&ring, &[(1, &[1, 0, 0])]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert!(r.is_zero());
    }

    #[test]
    fn mod_x_plus_1_gives_two_static() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 0, 0)]);
        let g = p2(&ring, &[(1, 1, 0), (1, 0, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        let expected = p2(&ring, &[(2, 0, 0)]);
        assert_eq!(r, expected);
    }

    #[test]
    fn multiple_reducers_produce_expected_remainder() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 1, 1), (1, 0, 0)]);
        let g1 = p2(&ring, &[(1, 1, 0)]);
        let g2 = p2(&ring, &[(1, 0, 1)]);

        let r = f.normal_form(&ring, [&g1, &g2]).unwrap();
        let expected = p2(&ring, &[(1, 0, 0)]);
        assert_eq!(r, expected);
    }

    #[test]
    fn remainder_is_already_in_normal_form() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 0, 1), (1, 0, 0)]);
        let g = p2(&ring, &[(1, 1, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn noninvertible_lc_errors() {
        let ring = ring_dyn();

        let f = pd(&ring, &[(1, &[1, 0, 0])]);

        let mut g = PD::zero_in(&ring);
        <PD as PolynomialMut>::push_term_raw(&mut g, td(&ring, 0, &[1, 0, 0]));

        let err = f.normal_form(&ring, core::iter::once(&g)).unwrap_err();
        assert_eq!(err, ReduceError::NonInvertibleLeadingCoefficient);
    }

    #[test]
    fn dynamic_multiple_reducers_produce_expected_remainder() {
        let ring = ring_dyn();

        let f = pd(&ring, &[(1, &[2, 0, 0]), (1, &[1, 1, 0]), (1, &[0, 0, 0])]);

        let g1 = pd(&ring, &[(1, &[1, 0, 0])]);
        let g2 = pd(&ring, &[(1, &[0, 1, 0])]);

        let r = f.normal_form(&ring, [&g1, &g2]).unwrap();
        let expected = pd(&ring, &[(1, &[0, 0, 0])]);
        assert_eq!(r, expected);
    }

    #[test]
    fn prepared_reducers_match_normal_form() {
        let ring = ring_static();

        let f = p2(&ring, &[(1, 2, 0), (1, 1, 1), (1, 0, 0)]);
        let g1 = p2(&ring, &[(1, 1, 0)]);
        let g2 = p2(&ring, &[(1, 0, 1)]);

        let lt1 = g1.leading_term().unwrap();
        let lt2 = g2.leading_term().unwrap();

        let inv1 = ring.field.try_inv(*lt1.coeff()).unwrap();
        let inv2 = ring.field.try_inv(*lt2.coeff()).unwrap();

        let prepared = vec![(&g1, lt1.mono(), inv1, lt1.mono().degree()), (&g2, lt2.mono(), inv2, lt2.mono().degree())];

        let r1 = f.normal_form(&ring, [&g1, &g2]).unwrap();
        let r2 = f.normal_form_prepared(&ring, prepared).unwrap();

        assert_eq!(r1, r2);
    }

    #[test]
    fn empty_reducers_canonicalize_input() {
        let ring = ring_static();

        let mut f = P2::zero_in(&ring);
        <P2 as PolynomialMut>::push_term_raw(&mut f, t2(1, 1, 0));
        <P2 as PolynomialMut>::push_term_raw(&mut f, t2(1, 1, 0));
        <P2 as PolynomialMut>::push_term_raw(&mut f, t2(0, 0, 0));

        let r = f.normal_form(&ring, core::iter::empty()).unwrap();
        let expected = p2(&ring, &[(2, 1, 0)]);

        assert_eq!(r, expected);
    }

    #[test]
    fn remainder_is_canonical_without_final_normalization() {
        let ring = ring_static();

        let f = p2(&ring, &[(1, 2, 0), (1, 0, 1), (1, 0, 0)]);
        let g = p2(&ring, &[(1, 1, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        let expected = p2(&ring, &[(1, 0, 1), (1, 0, 0)]);

        assert_eq!(r, expected);
    }
}
