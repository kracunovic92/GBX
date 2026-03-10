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
        let reducers: Vec<&Self> = gs.into_iter().filter(|g| !g.is_zero()).collect();
        if reducers.is_empty() {
            return Ok(self.clone());
        }

        let mut f = self.clone();
        f.normalize_in_place(ctx).map_err(ReduceError::from)?;

        let mut r = Self::zero_in(ctx);

        while let Some(lt_f) = f.leading_term().cloned() {
            let mut reduced = false;

            for g in &reducers {
                let Some(lt_g) = g.leading_term() else { continue };

                let q_m = match lt_f.mono().checked_div_by(lt_g.mono())? {
                    Some(q) => q,
                    None => continue,
                };

                let inv_lc_g = ctx
                    .field
                    .try_inv(*lt_g.coeff())
                    .ok_or(ReduceError::NonInvertibleLeadingCoefficient)?;

                let q_c = ctx.field.mul(*lt_f.coeff(), inv_lc_g);

                // tmp := (q_c * q_m) * g
                let mut tmp = (*g).clone();
                tmp.mul_monomial_assign_raw(ctx, &q_m)
                    .map_err(ReduceError::from)?;
                tmp.scale_assign_raw(ctx, q_c).map_err(ReduceError::from)?;

                // f := f - tmp, then normalize
                f.sub_assign_raw(ctx, &tmp).map_err(ReduceError::from)?;
                f.normalize_in_place(ctx).map_err(ReduceError::from)?;

                reduced = true;
                break;
            }

            if !reduced {
                let lt = f
                    .pop_leading_term_raw(ctx)
                    .map_err(ReduceError::from)?
                    .ok_or(ReduceError::Poly(PolynomialError::InvariantViolation))?;

                // Disambiguate explicitly: only PolynomialMut owns push_term_raw.
                <Self as PolynomialMut>::push_term_raw(&mut r, lt);
            }
        }

        r.normalize_in_place(ctx).map_err(ReduceError::from)?;
        Ok(r)
    }
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

    #[test]
    fn normal_form_empty_set_is_identity_static() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 1, 0), (1, 0, 0)]);
        let r = f.normal_form(&ring, core::iter::empty()).unwrap();
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
    fn mod_x_plus_1_gives_two_static() {
        let ring = ring_static();
        let f = p2(&ring, &[(1, 2, 0), (1, 0, 0)]); // x^2 + 1
        let g = p2(&ring, &[(1, 1, 0), (1, 0, 0)]); // x + 1

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        let expected = p2(&ring, &[(2, 0, 0)]);
        assert_eq!(r, expected);
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
    fn reduces_to_zero_dynamic() {
        let ring = ring_dyn();

        let f = pd(&ring, &[(1, &[2, 0, 0]), (1, &[1, 0, 0])]);
        let g = pd(&ring, &[(1, &[1, 0, 0])]);

        let r = f.normal_form(&ring, core::iter::once(&g)).unwrap();
        assert!(r.is_zero());
    }

    #[test]
    fn noninvertible_lc_errors() {
        let ring = ring_dyn();

        // f = x0
        let f = pd(&ring, &[(1, &[1, 0, 0])]);

        // build a "bad" reducer with leading coeff 0 by bypassing normalization
        let mut g = PD::zero_in(&ring);
        <PD as PolynomialMut>::push_term_raw(&mut g, td(&ring, 0, &[1, 0, 0])); // lc = 0

        let err = f.normal_form(&ring, core::iter::once(&g)).unwrap_err();
        assert_eq!(err, ReduceError::NonInvertibleLeadingCoefficient);
    }
}
