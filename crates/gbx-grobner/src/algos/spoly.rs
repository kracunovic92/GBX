//! S-polynomial construction (context-driven).
//!
//! Computes `S(f, g)` inside a ring context.
//!
//! Coefficient division is performed via `ctx.field.try_inv(..)` because dynamic
//! coefficient elements (e.g. `FpDynElem`) do not carry enough information to
//! implement division on the element type itself.
//!
//! # Errors
//! - `ZeroInput` if `f` or `g` has no leading term
//! - `NonInvertibleLeadingCoefficient` if `lc(f)` or `lc(g)` is not invertible in `ctx.field`
//! - `Poly(..)` for ring/term/monomial errors
//!
//! # Panics
//! None (beyond panics in user-provided storage backends).

use gbx_poly::monomial::{Monomial, MonomialAlgos, MonomialError};
use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialError, PolynomialOps};
use gbx_poly::ring::{FieldCtx, RingCtx};
use gbx_poly::term::{TermOwned, TermView};
use thiserror::Error;

/// Errors that can occur while constructing an S-polynomial.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum SPolyError {
    /// Propagated polynomial/term/monomial/ring error.
    #[error(transparent)]
    Poly(#[from] PolynomialError),

    /// One input was zero (no leading term).
    #[error("cannot form S-polynomial: one input polynomial is zero")]
    ZeroInput,

    /// Leading coefficient was not invertible in `ctx.field`.
    #[error("cannot form S-polynomial: leading coefficient is not invertible")]
    NonInvertibleLeadingCoefficient,
}

impl From<MonomialError> for SPolyError {
    #[inline]
    fn from(e: MonomialError) -> Self {
        Self::Poly(PolynomialError::from(e))
    }
}

/// Compute `S(f, g)` inside a ring context.
///
/// This is the correct primitive for context-driven coefficient domains.
pub fn s_polynomial_in<P, F, O>(ctx: &RingCtx<F, O>, f: &P, g: &P) -> Result<P, SPolyError>
where
    // ctx
    F: FieldCtx<Elem = <P::Term as TermView>::Coeff>,
    O: MonomialOrder,
    P: PolynomialOps + Clone,
    P::Term: TermOwned + TermView + Clone,
    <P::Term as TermView>::Coeff: Copy + Eq,
    <P::Term as TermView>::Mono: Monomial + MonomialAlgos + Clone + Eq,
{
    // Ring tag checks
    ctx.assert_same_ring_id(f.ring_id())
        .map_err(PolynomialError::from)?;
    ctx.assert_same_ring_id(g.ring_id())
        .map_err(PolynomialError::from)?;

    let lt_f = f.leading_term().ok_or(SPolyError::ZeroInput)?;
    let lt_g = g.leading_term().ok_or(SPolyError::ZeroInput)?;

    let lm_f = lt_f.mono();
    let lm_g = lt_g.mono();
    let lc_f = *lt_f.coeff();
    let lc_g = *lt_g.coeff();

    // l = lcm(lm(f), lm(g))
    let l = lm_f.checked_lcm(lm_g)?;

    // mf = l / lm(f), mg = l / lm(g)
    let mf = l.checked_div_exact_by(lm_f)?;
    let mg = l.checked_div_exact_by(lm_g)?;

    // inv(lc(f)), inv(lc(g)) via ctx.field
    let inv_lc_f = ctx
        .field
        .try_inv(lc_f)
        .ok_or(SPolyError::NonInvertibleLeadingCoefficient)?;
    let inv_lc_g = ctx
        .field
        .try_inv(lc_g)
        .ok_or(SPolyError::NonInvertibleLeadingCoefficient)?;

    // a = (mf * inv_lc_f) * f
    let mut a = f.clone();
    a.mul_monomial_assign_raw(ctx, &mf)?;
    a.scale_assign_raw(ctx, inv_lc_f)?;

    // b = (mg / lc(g)) * g
    let mut b = g.clone();
    b.mul_monomial_assign_raw(ctx, &mg)?;
    b.scale_assign_raw(ctx, inv_lc_g)?;

    // S = a - b
    a.sub_assign_raw(ctx, &b)?;
    a.normalize_in_place(ctx)?;

    Ok(a)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use gbx_field::fp::Fp;
    use gbx_poly::monomial::{FixedMonomial, MonomialView};
    use gbx_poly::order::Lex;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};
    use gbx_poly::ring::{Ring, StaticFpCtx};
    use gbx_poly::term::Term;
    use gbx_storage::polynomial::VecTerms;

    type F7 = Fp<7>;
    type T2 = Term<F7, FixedMonomial<2>>;
    type P2 = Polynomial<T2, VecTerms<T2>>;

    fn ring() -> RingCtx<StaticFpCtx<7>, Lex> {
        Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap()
    }

    fn t(c: u32, e0: u32, e1: u32) -> T2 {
        Term::new(F7::new(c), FixedMonomial::from_exponents([e0, e1]))
    }

    fn p(r: &RingCtx<StaticFpCtx<7>, Lex>, ts: Vec<T2>) -> P2 {
        P2::from_terms_in(r, ts).unwrap()
    }

    fn has_mono(p: &P2, exps: &[u32]) -> bool {
        p.terms().iter().any(|tt| tt.mono().exponents() == exps)
    }

    #[test]
    fn s_poly_cancels_lcm_term() {
        let r = ring();
        // f = 2*x^2 + 1
        let f = p(&r, vec![t(2, 2, 0), t(1, 0, 0)]);
        // g = 3*x*y + 1
        let g = p(&r, vec![t(3, 1, 1), t(1, 0, 0)]);

        let s = s_polynomial_in(&r, &f, &g).unwrap();
        assert!(!has_mono(&s, &[2, 1])); // x^2 y cancels
    }

    #[test]
    fn s_poly_matches_expected_over_f7() {
        let r = ring();
        let f = p(&r, vec![t(2, 2, 0), t(1, 0, 0)]);
        let g = p(&r, vec![t(3, 1, 1), t(1, 0, 0)]);

        // Expected: 2x + 4y in F7
        let expected = p(&r, vec![t(2, 1, 0), t(4, 0, 1)]);

        let s = s_polynomial_in(&r, &f, &g).unwrap();
        assert_eq!(s, expected);
    }
}
