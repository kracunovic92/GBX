//! Core polynomial traits.

use crate::monomial::Monomial;
use crate::ring::{FieldCtx, RingCtx, RingId};
use crate::term::Term;

/// Read-only polynomial interface.
pub trait PolynomialView {
    /// Coefficient type.
    type Coeff;

    /// Ring identity tag stored in the polynomial.
    fn ring_id(&self) -> RingId;

    /// Canonical term slice.
    fn terms(&self) -> &[Term<Self::Coeff>];

    /// Returns true if this is the zero polynomial.
    #[inline]
    fn is_zero(&self) -> bool {
        self.terms().is_empty()
    }

    /// Leading term.
    #[inline]
    fn leading_term(&self) -> Option<&Term<Self::Coeff>> {
        self.terms().first()
    }

    /// Leading monomial.
    #[inline]
    fn leading_mono(&self) -> Option<&Monomial> {
        self.leading_term().map(|t| t.mono())
    }

    /// Leading coefficient.
    #[inline]
    fn leading_coeff(&self) -> Option<&Self::Coeff> {
        self.leading_term().map(|t| t.coeff())
    }

    /// Number of terms.
    #[inline]
    fn len(&self) -> usize {
        self.terms().len()
    }

    /// Returns true if this polynomial has exactly one term.
    #[inline]
    fn is_monomial(&self) -> bool {
        self.len() == 1
    }

    /// Returns true if this polynomial is constant.
    ///
    /// The zero polynomial is considered constant.
    #[inline]
    fn is_constant(&self) -> bool {
        self.is_zero() || self.leading_term().is_some_and(|t| t.mono().is_one())
    }

    /// Returns true if this polynomial is a nonzero constant.
    #[inline]
    fn is_nonzero_constant(&self) -> bool {
        self.len() == 1 && self.leading_term().is_some_and(|t| t.mono().is_one())
    }
}

/// Mutation hooks used by algorithms.
pub trait PolynomialMut: PolynomialView + Sized {
    /// Creates zero polynomial tagged with `ctx.id()`.
    fn zero_in<F, O>(ctx: &RingCtx<F, O>) -> Self
    where
        F: FieldCtx<Elem = Self::Coeff>;

    /// Builds from raw terms and normalizes.
    fn from_terms_in<F, O>(ctx: &RingCtx<F, O>, terms: Vec<Term<Self::Coeff>>) -> crate::polynomial::PolynomialResult<Self>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: crate::order::MonomialOrder;

    /// Pushes a raw term.
    ///
    /// This may break canonical invariants until normalization.
    fn push_term_raw(&mut self, t: Term<Self::Coeff>);

    /// Normalizes in-place.
    fn normalize_in_place<F, O>(&mut self, ctx: &RingCtx<F, O>) -> crate::polynomial::PolynomialResult<()>
    where
        F: FieldCtx<Elem = Self::Coeff>,
        O: crate::order::MonomialOrder;
}
