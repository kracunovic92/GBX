//! Core polynomial traits used by algorithms.
//!
//! Algorithms should depend on these traits rather than concrete polynomial types.
//!
//! Polynomials are context-driven:
//! - field arithmetic comes from `ctx.field`
//! - monomial order comes from `ctx.order`
//! - polynomials store a [`RingId`](crate::ring::RingId) safety tag to detect mixing rings.

use crate::order::MonomialOrder;
use crate::ring::{FieldCtx, RingCtx, RingId};
use crate::term::TermView;

/// Read-only polynomial interface.
///
/// If normalized, the leading term is expected at index 0.
pub trait PolynomialView {
    /// Sparse term type.
    type Term: TermView;

    /// Ring identity tag stored in the polynomial.
    fn ring_id(&self) -> RingId;

    /// Terms slice (canonical order if normalized).
    fn terms(&self) -> &[Self::Term];

    /// Is this polynomial exactly zero?
    #[inline]
    fn is_zero(&self) -> bool {
        self.terms().is_empty()
    }

    /// Leading term (if normalized, this is `terms().first()`).
    #[inline]
    fn leading_term(&self) -> Option<&Self::Term> {
        self.terms().first()
    }

    /// Leading monomial (if any).
    #[inline]
    fn leading_mono(&self) -> Option<&<Self::Term as TermView>::Mono> {
        self.leading_term().map(|t| t.mono())
    }

    /// Leading coefficient (if any).
    #[inline]
    fn leading_coeff(&self) -> Option<&<Self::Term as TermView>::Coeff> {
        self.leading_term().map(|t| t.coeff())
    }

    /// Number of terms.
    #[inline]
    fn len(&self) -> usize {
        self.terms().len()
    }
}

/// Minimal mutation hooks needed by generic algorithms.
///
/// Note: these methods are context-driven; the polynomial itself does not know
/// modulus/order/nvars.
pub trait PolynomialMut: PolynomialView + Sized {
    /// Create the zero polynomial tagged with `ctx.id()`.
    fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx,
        O: MonomialOrder;

    /// Build from raw terms and normalize using `ctx`.
    fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, terms: Vec<Self::Term>) -> crate::polynomial::Result<Self>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;

    /// Push a raw term (may violate invariants until normalized).
    fn push_term_raw(&mut self, t: Self::Term);

    /// Normalize in-place using `ctx` (canonical form).
    fn normalize_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>) -> crate::polynomial::Result<()>
    where
        F: FieldCtx<Elem = <Self::Term as TermView>::Coeff>,
        O: MonomialOrder;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::monomial::FixedMonomial;
    use crate::order::Lex;
    use crate::polynomial::poly::Polynomial;
    use crate::ring::{Ring, StaticFpCtx};
    use crate::term::Term;
    use gbx_field::fp::Fp;
    use gbx_storage::polynomial::VecTerms;

    type F7 = Fp<7>;
    type T2 = Term<F7, FixedMonomial<2>>;
    type P2 = Polynomial<T2, VecTerms<T2>>;

    #[test]
    fn view_helpers_work() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let p = P2::from_terms_in(
            &ring,
            vec![Term::new(F7::new(1), FixedMonomial::<2>::from_exponents([2, 0])), Term::new(F7::new(3), FixedMonomial::<2>::from_exponents([1, 0]))],
        )
        .unwrap();

        assert_eq!(p.len(), 2);
        assert!(p.leading_term().is_some());
        assert!(p.leading_mono().is_some());
        assert!(p.leading_coeff().is_some());
        assert_eq!(p.ring_id(), ring.id());
    }
}
