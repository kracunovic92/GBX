use crate::monomial::{Monomial, MonomialView};

/// Read-only view of a term.
pub trait TermView: Clone + PartialEq {
    type Field;
    type Mono: MonomialView;

    fn coeff(&self) -> &Self::Field;
    fn mono(&self) -> &Self::Mono;
}

/// A term that supports constructing new values (needed by algorithms).
pub trait Term: TermView
where
    Self::Mono: Monomial,
{
    type Error;

    fn from_parts(coeff: Self::Field, mono: Self::Mono) -> Self;

    /// Multiply by scalar (affects only coefficient).
    fn mul_scalar(&self, c: &Self::Field) -> Self;

    /// Multiply by monomial (affects only monomial).
    fn checked_mul_monomial(&self, m: &Self::Mono) -> Result<Self, Self::Error>;

    /// Multiply by another term.
    fn checked_mul_term(&self, other: &Self) -> Result<Self, Self::Error>;
}

/// Ergonomic extension trait.
pub trait TermOps: Term
where
    <Self as TermView>::Mono: Monomial,
{
    #[inline]
    fn mul_term(&self, other: &Self) -> Self
    where
        Self::Error: core::fmt::Debug,
        <Self as TermView>::Mono: Monomial,
    {
        self.checked_mul_term(other)
            .expect("TermOps::mul_term failed")
    }

    #[inline]
    fn mul_monomial(&self, m: &Self::Mono) -> Self
    where
        Self::Error: core::fmt::Debug,
        <Self as TermView>::Mono: Monomial,
    {
        self.checked_mul_monomial(m)
            .expect("TermOps::mul_monomial failed")
    }
}

impl<T: Term> TermOps for T where <T as TermView>::Mono: Monomial {}
