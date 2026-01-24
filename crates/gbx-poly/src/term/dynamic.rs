use crate::monomial::{DynamicMonomial, Monomial, MonomialViewExt};
use crate::term::traits::{Term, TermView};
use crate::term::TermError;
use core::fmt;
use gbx_alg::Field;

/// A term `coeff * x^α` with runtime-chosen arity.
#[derive(Clone, PartialEq, Eq)]
pub struct DynamicTerm<F: Field> {
    pub coeff: F,
    pub mono: DynamicMonomial,
}

impl<F: Field> DynamicTerm<F> {
    #[inline]
    pub fn new(coeff: F, mono: DynamicMonomial) -> Self {
        Self { coeff, mono }
    }

    #[inline]
    pub fn from_coeff_and_slice(coeff: F, exponents: &[u32]) -> Self {
        Self { coeff, mono: DynamicMonomial::from_slice(exponents) }
    }

    #[inline]
    pub fn degree(&self) -> u32 {
        self.mono.degree()
    }
}

impl<F: Field + Clone> TermView for DynamicTerm<F> {
    type Field = F;
    type Mono = DynamicMonomial;

    #[inline]
    fn coeff(&self) -> &Self::Field {
        &self.coeff
    }

    #[inline]
    fn mono(&self) -> &Self::Mono {
        &self.mono
    }
}

impl<F> Term for DynamicTerm<F>
where
    F: Field + Clone,
{
    type Error = TermError;

    #[inline]
    fn from_parts(coeff: Self::Field, mono: Self::Mono) -> Self {
        Self { coeff, mono }
    }

    #[inline]
    fn mul_scalar(&self, c: &Self::Field) -> Self {
        Self { coeff: self.coeff.clone() * c.clone(), mono: self.mono.clone() }
    }

    #[inline]
    fn checked_mul_monomial(&self, m: &Self::Mono) -> Result<Self, Self::Error> {
        let mono = self.mono.checked_mul(m).map_err(TermError::from)?;
        Ok(Self::from_parts(self.coeff.clone(), mono))
    }

    #[inline]
    fn checked_mul_term(&self, other: &Self) -> Result<Self, Self::Error> {
        let coeff = self.coeff.clone() * other.coeff.clone();
        let mono = self
            .mono
            .checked_mul(&other.mono)
            .map_err(TermError::from)?;
        Ok(Self::from_parts(coeff, mono))
    }
}

impl<F: Field + fmt::Debug> fmt::Debug for DynamicTerm<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynamicTerm")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

impl<F: Field + fmt::Display> fmt::Display for DynamicTerm<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mono.is_one() { write!(f, "{}", self.coeff) } else { write!(f, "{}*{:?}", self.coeff, self.mono) }
    }
}
