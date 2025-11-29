use crate::monomial::Monomial;
use algebra_core::{Field, Zero};
use core::fmt;

use super::{TermError, TermLike};

/// A single term `coeff * x^α` in a multivariate polynomial with
/// a **fixed** number of variables.
///
/// - `F` is a field (e.g. `Zp<P>` from `algebra_field`).
/// - `N` is the number of variables.
/// - The monomial part is represented by a [`Monomial<N>`] (exponent vector).
#[derive(Clone, PartialEq, Eq)]
pub struct Term<F: Field, const N: usize> {
    /// The coefficient in the field.
    pub coeff: F,
    /// The monomial (exponent vector).
    pub mono: Monomial<N>,
}

impl<F: Field, const N: usize> Term<F, N> {
    /// Constructs a term from a coefficient and a monomial.
    #[inline]
    pub fn new(coeff: F, mono: Monomial<N>) -> Self {
        Self { coeff, mono }
    }

    /// Constructs a term from a coefficient and an exponent vector.
    ///
    /// Convenience constructor that avoids manually calling `Monomial::from_exponents`.
    #[inline]
    pub fn from_coeff_and_exponents(coeff: F, exponents: [u32; N]) -> Self {
        Self { coeff, mono: Monomial::from_exponents(exponents) }
    }

    /// Degree of the term = degree of its monomial.
    #[inline]
    pub fn degree(&self) -> u64 {
        self.mono
            .degree()
    }

    /// Returns `true` if the coefficient is zero.
    ///
    /// In a normalized polynomial representation, such terms are usually removed.
    #[inline]
    pub fn is_zero(&self) -> bool
    where
        F: Zero,
    {
        self.coeff == F::zero()
    }

    /// Multiplies this term by a scalar `c` in the field.
    ///
    /// Mathematically: `(a * x^α) * c = (a * c) * x^α`.
    ///
    /// This clones both `self.coeff` and `c` to avoid requiring `Copy`.
    #[inline]
    pub fn mul_scalar(&self, c: &F) -> Self
    where
        F: Clone,
    {
        <Self as TermLike>::mul_scalar(self, c)
    }

    /// Checked multiplication by a monomial.
    ///
    /// Mathematically: `(a * x^α) * x^β = a * x^{α+β}`.
    /// Returns an error if monomial exponent addition overflows.
    #[inline]
    pub fn checked_mul_monomial(&self, m: &Monomial<N>) -> Result<Self, TermError>
    where
        F: Clone,
    {
        <Self as TermLike>::checked_mul_monomial(self, m)
    }

    /// Multiplies this term by a monomial.
    ///
    /// Mathematically: `(a * x^α) * x^β = a * x^{α+β}`.
    ///
    /// # Panics
    ///
    /// Panics if exponent addition overflows.
    #[inline]
    pub fn mul_monomial(&self, m: &Monomial<N>) -> Self
    where
        F: Clone,
    {
        <Self as TermLike>::mul_monomial(self, m)
    }
}

/// `TermLike` implementation for fixed-size terms.
impl<F, const N: usize> TermLike for Term<F, N>
where
    F: Field + Clone,
{
    type Field = F;
    type Mono = Monomial<N>;
    type Error = TermError;

    #[inline]
    fn coeff(&self) -> &Self::Field {
        &self.coeff
    }

    #[inline]
    fn mono(&self) -> &Self::Mono {
        &self.mono
    }

    #[inline]
    fn mul_scalar(&self, c: &Self::Field) -> Self {
        Self {
            coeff: self
                .coeff
                .clone()
                * c.clone(),
            mono: self.mono, // Monomial<N> is Copy
        }
    }

    #[inline]
    fn checked_mul_monomial(&self, m: &Self::Mono) -> Result<Self, Self::Error> {
        let mono = self
            .mono
            .checked_mul(m)
            .map_err(TermError::Monomial)?;
        Ok(Self {
            coeff: self
                .coeff
                .clone(),
            mono,
        })
    }
}

impl<F: Field + fmt::Debug, const N: usize> fmt::Debug for Term<F, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Term")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Term;
    use super::TermError;
    use crate::monomial::Monomial;
    use algebra_core::{One, Zero};
    use algebra_field::Zp;

    type F7 = Zp<7>;
    type T2 = Term<F7, 2>;

    #[test]
    fn new_stores_coeff_and_monomial() {
        let mono = Monomial::<2>::from_exponents([1, 3]); // x1^1 x2^3
        let coeff = F7::new(5);
        let t = T2::new(coeff, mono);

        assert_eq!(
            t.coeff
                .value(),
            5
        );
        assert_eq!(
            t.mono
                .exponents(),
            &[1, 3]
        );
    }

    #[test]
    fn from_coeff_and_exponents_constructs_term_directly() {
        let t = T2::from_coeff_and_exponents(F7::new(6), [2, 0]);

        assert_eq!(
            t.coeff
                .value(),
            6
        );
        assert_eq!(
            t.mono
                .exponents(),
            &[2, 0]
        );
    }

    #[test]
    fn degree_matches_monomial_degree() {
        let t = T2::from_coeff_and_exponents(F7::new(3), [1, 2]);
        assert_eq!(t.degree(), 3);
    }

    #[test]
    fn is_zero_detects_zero_and_nonzero_coefficients() {
        let zero_term = T2::from_coeff_and_exponents(F7::zero(), [0, 0]);
        let nonzero_term = T2::from_coeff_and_exponents(F7::one(), [0, 0]);

        assert!(zero_term.is_zero());
        assert!(!nonzero_term.is_zero());
    }

    #[test]
    fn mul_scalar_multiplies_coefficient_mod_p_and_preserves_monomial() {
        let t = T2::from_coeff_and_exponents(F7::new(3), [1, 2]); // 3 * x1^1 x2^2
        let c = F7::new(5);

        // In F7, 3 * 5 = 15 ≡ 1 mod 7
        let result = t.mul_scalar(&c);

        assert_eq!(
            result
                .coeff
                .value(),
            (3 * 5) % 7
        );
        assert_eq!(
            result
                .mono
                .exponents(),
            &[1, 2]
        );
    }

    #[test]
    fn mul_monomial_adds_exponents_and_preserves_coefficient() {
        let t = T2::from_coeff_and_exponents(F7::new(4), [1, 1]); // 4 * x1^1 x2^1
        let m = Monomial::<2>::from_exponents([2, 3]); // x1^2 x2^3

        let result = t.mul_monomial(&m);

        // coeff stays 4
        assert_eq!(
            result
                .coeff
                .value(),
            4
        );
        // exponents should add: [1+2, 1+3] = [3, 4]
        assert_eq!(
            result
                .mono
                .exponents(),
            &[3, 4]
        );
    }

    #[test]
    fn checked_mul_monomial_propagates_monomial_overflow() {
        let max = u32::MAX;
        let t = T2::from_coeff_and_exponents(F7::one(), [max, 0]);
        let m = Monomial::<2>::from_exponents([1, 0]);

        let err = t
            .checked_mul_monomial(&m)
            .unwrap_err();
        assert!(matches!(err, TermError::Monomial(_)));
    }
}
