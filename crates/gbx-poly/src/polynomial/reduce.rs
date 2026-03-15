//! Polynomial reduction / normal form (context-driven).
//!
//! Computes the normal form (remainder) of `f` with respect to a set of reducers.
//!
//! Division and cancellation use the ring context:
//! - coefficient division is performed using `ctx.field.try_inv` + `ctx.field.mul`
//!
//! # Errors
//! - `ReduceError::Poly(..)` for ring/term/monomial errors
//! - `ReduceError::NonInvertibleLeadingCoefficient` if a reducer has non-invertible LC
//!
//! # Panics
//! None.

use crate::monomial::{Monomial, MonomialAlgos, MonomialError};
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
    <Self::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
    <Self::Term as TermView>::Coeff: Copy + Eq,
{
    /// Compute the normal form (remainder) of `self` w.r.t. reducers `gs`.
    ///
    /// Reducers that are zero are ignored.
    fn normal_form<'a, F, O, I>(&self, ctx: &RingCtx<F, O>, gs: I) -> Result<Self, ReduceError>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder,
        I: IntoIterator<Item = &'a Self>,
        Self: 'a,
    {
        struct ReducerRef<'a, P, C, M> {
            poly: &'a P,
            lm: &'a M,
            inv_lc: C,
        }

        let mut reducers = Vec::new();

        for g in gs.into_iter().filter(|g| !g.is_zero()) {
            let lt_g = g
                .leading_term()
                .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

            let inv_lc = ctx
                .field
                .try_inv(*lt_g.coeff())
                .ok_or(ReduceError::NonInvertibleLeadingCoefficient)?;

            reducers.push(ReducerRef { poly: g, lm: lt_g.mono(), inv_lc });
        }

        let mut f = self.clone();
        f.normalize_in_place(ctx).map_err(ReduceError::from)?;

        if reducers.is_empty() {
            return Ok(f);
        }

        let mut r = Self::zero_in(ctx);

        while let Some((lt_coeff, lt_mono)) = f.leading_term().map(|lt| (*lt.coeff(), lt.mono().clone())) {
            let mut reduced = false;

            for red in &reducers {
                let q_m = match lt_mono.checked_div_by(red.lm)? {
                    Some(q) => q,
                    None => continue,
                };

                let q_c = ctx.field.mul(lt_coeff, red.inv_lc);

                f = sub_scaled_monomial_multiple_canonical(ctx, &f, red.poly, &q_m, q_c)?;
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

/// Compute `f - scalar * monomial * g`, assuming both `f` and `g` are canonical,
/// and return the result in canonical form **without** global normalization.
///
/// This is a linear merge of two sorted term streams:
/// - terms of `f`
/// - terms of `scalar * monomial * g`
fn sub_scaled_monomial_multiple_canonical<P, F, O>(ctx: &RingCtx<F, O>, f: &P, g: &P, mul_mono: &<P::Term as TermView>::Mono, scalar: <P::Term as TermView>::Coeff) -> Result<P, ReduceError>
where
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + PolynomialView + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
{
    let mut out = P::zero_in(ctx);

    // Replace `.terms()` below with your actual canonical term accessor if needed.
    let f_terms = f.terms();
    let g_terms = g.terms();

    let mut i = 0usize;
    let mut j = 0usize;

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
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
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
    use crate::ring::{Ring, StaticFpCtx};
    use crate::term::Term;
    use gbx_field::fp::{Fp, FpDyn, FpDynElem};
    use gbx_storage::polynomial::VecTerms;

    // ----------------------------
    // Static: Fp<7>, FixedMonomial<2>
    // ----------------------------
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

    // ----------------------------
    // Dynamic: FpDyn + DynamicMonomial
    // ----------------------------
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
        let f = p2(&ring, &[(1, 1, 0), (2, 0, 0)]); // x + 2
        let g = p2(&ring, &[(1, 0, 1), (1, 0, 0)]); // y + 1

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
        let f = p2(&ring, &[(1, 2, 0), (1, 0, 0)]); // x^2 + 1
        let g = p2(&ring, &[(1, 1, 0), (1, 0, 0)]); // x + 1

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        let expected = p2(&ring, &[(2, 0, 0)]);
        assert_eq!(r, expected);
    }

    #[test]
    fn multiple_reducers_produce_expected_remainder() {
        let ring = ring_static();
        // f = x^2 + x*y + 1
        let f = p2(&ring, &[(1, 2, 0), (1, 1, 1), (1, 0, 0)]);
        // g1 = x
        let g1 = p2(&ring, &[(1, 1, 0)]);
        // g2 = y
        let g2 = p2(&ring, &[(1, 0, 1)]);

        let r = f.normal_form(&ring, [&g1, &g2]).unwrap();
        let expected = p2(&ring, &[(1, 0, 0)]);
        assert_eq!(r, expected);
    }

    #[test]
    fn remainder_is_already_in_normal_form() {
        let ring = ring_static();
        // f = y + 1, reducer x
        let f = p2(&ring, &[(1, 0, 1), (1, 0, 0)]);
        let g = p2(&ring, &[(1, 1, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert_eq!(r, f);
    }

    #[test]
    fn noninvertible_lc_errors() {
        let ring = ring_dyn();

        let f = pd(&ring, &[(1, &[1, 0, 0])]);

        // Build a "bad" reducer with lc = 0 by bypassing normalization.
        let mut g = PD::zero_in(&ring);
        <PD as PolynomialMut>::push_term_raw(&mut g, td(&ring, 0, &[1, 0, 0]));

        let err = f.normal_form(&ring, core::iter::once(&g)).unwrap_err();
        assert_eq!(err, ReduceError::NonInvertibleLeadingCoefficient);
    }

    #[test]
    fn dynamic_multiple_reducers_produce_expected_remainder() {
        let ring = ring_dyn();

        // f = x0^2 + x0*x1 + 1
        let f = pd(&ring, &[(1, &[2, 0, 0]), (1, &[1, 1, 0]), (1, &[0, 0, 0])]);

        let g1 = pd(&ring, &[(1, &[1, 0, 0])]);
        let g2 = pd(&ring, &[(1, &[0, 1, 0])]);

        let r = f.normal_form(&ring, [&g1, &g2]).unwrap();
        let expected = pd(&ring, &[(1, &[0, 0, 0])]);
        assert_eq!(r, expected);
    }
    #[test]
    fn empty_reducers_canonicalize_input() {
        let ring = ring_static();

        // Build a non-canonical polynomial by bypassing normalization:
        // x + x + 0
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

        // f = x^2 + y + 1
        // g = x
        let f = p2(&ring, &[(1, 2, 0), (1, 0, 1), (1, 0, 0)]);
        let g = p2(&ring, &[(1, 1, 0)]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        let expected = p2(&ring, &[(1, 0, 1), (1, 0, 0)]);

        assert_eq!(r, expected);
    }
}
