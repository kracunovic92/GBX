use crate::monomial::{FixedMonomial, Monomial};
use crate::term::traits::{Term, TermView};
use crate::term::TermError;
use core::fmt;
use gbx_alg::Field;

/// A term `coeff * x^α` with compile-time arity `N`.
#[derive(Clone, PartialEq, Eq)]
pub struct FixedTerm<F: Field, const N: usize> {
    pub coeff: F,
    pub mono: FixedMonomial<N>,
}

impl<F: Field, const N: usize> FixedTerm<F, N> {
    #[inline]
    pub fn new(coeff: F, mono: FixedMonomial<N>) -> Self {
        Self { coeff, mono }
    }

    #[inline]
    pub fn from_coeff_and_exponents(coeff: F, exponents: [u32; N]) -> Self {
        Self { coeff, mono: FixedMonomial::from_exponents(exponents) }
    }

    #[inline]
    pub fn degree(&self) -> u32 {
        // cached in monomial
        self.mono.degree()
    }
}

impl<F: Field + Clone, const N: usize> TermView for FixedTerm<F, N> {
    type Field = F;
    type Mono = FixedMonomial<N>;

    #[inline]
    fn coeff(&self) -> &Self::Field {
        &self.coeff
    }

    #[inline]
    fn mono(&self) -> &Self::Mono {
        &self.mono
    }
}

impl<F, const N: usize> Term for FixedTerm<F, N>
where
    F: Field + Clone,
{
    type Error = TermError;

    #[inline]
    fn from_parts(coeff: Self::Field, mono: <FixedTerm<F, N> as TermView>::Mono) -> Self {
        Self { coeff, mono }
    }

    #[inline]
    fn mul_scalar(&self, c: &Self::Field) -> Self {
        Self { coeff: self.coeff.clone() * c.clone(), mono: self.mono }
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

impl<F: Field + fmt::Debug, const N: usize> fmt::Debug for FixedTerm<F, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Term")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}
