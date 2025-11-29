use crate::monomial::DynamicMonomial;
use algebra_core::{Field, Zero};
use core::fmt;

use super::{TermError, TermLike};

/// A single term `coeff * x^α` in a multivariate polynomial with a
/// **runtime-chosen** number of variables.
///
/// - `F` is a field.
/// - The monomial part is represented by a [`DynamicMonomial`].
#[derive(Clone, PartialEq, Eq)]
pub struct DynamicTerm<F: Field> {
    /// The coefficient in the field.
    pub coeff: F,
    /// The monomial (runtime-sized exponent vector).
    pub mono: DynamicMonomial,
}

impl<F: Field> DynamicTerm<F> {
    /// Constructs a dynamic term from a coefficient and a dynamic monomial.
    #[inline]
    pub fn new(coeff: F, mono: DynamicMonomial) -> Self {
        Self { coeff, mono }
    }

    /// Constructs a term from a coefficient and a slice of exponents.
    ///
    /// Convenience constructor that avoids manually calling
    /// [`DynamicMonomial::from_slice`].
    #[inline]
    pub fn from_coeff_and_slice(coeff: F, exponents: &[u32]) -> Self {
        Self { coeff, mono: DynamicMonomial::from_slice(exponents) }
    }

    /// Constructs a term from a coefficient and a `Vec<u32>` of exponents.
    #[inline]
    pub fn from_coeff_and_vec(coeff: F, exponents: Vec<u32>) -> Self {
        Self { coeff, mono: DynamicMonomial::from_vec(exponents) }
    }

    /// Degree of the term = degree of its monomial.
    #[inline]
    pub fn degree(&self) -> u64 {
        self.mono
            .degree()
    }

    /// Returns `true` if the coefficient is zero.
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
    #[inline]
    pub fn mul_scalar(&self, c: &F) -> Self
    where
        F: Clone,
    {
        <Self as TermLike>::mul_scalar(self, c)
    }

    /// Checked multiplication by a dynamic monomial.
    ///
    /// Mathematically: `(a * x^α) * x^β = a * x^{α+β}`.
    ///
    /// Returns an error if:
    /// - exponent addition would overflow, or
    /// - the monomials have mismatched variable counts.
    #[inline]
    pub fn checked_mul_monomial(&self, m: &DynamicMonomial) -> Result<Self, TermError>
    where
        F: Clone,
    {
        <Self as TermLike>::checked_mul_monomial(self, m)
    }

    /// Multiplies this term by a dynamic monomial.
    ///
    /// # Panics
    ///
    /// Panics if exponent addition overflows or if the variable counts
    /// do not match. For a non-panicking version, use
    /// [`Self::checked_mul_monomial`].
    #[inline]
    pub fn mul_monomial(&self, m: &DynamicMonomial) -> Self
    where
        F: Clone,
    {
        <Self as TermLike>::mul_monomial(self, m)
    }
}

/// `TermLike` implementation for dynamic terms.
impl<F> TermLike for DynamicTerm<F>
where
    F: Field + Clone,
{
    type Field = F;
    type Mono = DynamicMonomial;
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
            mono: self
                .mono
                .clone(),
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

impl<F: Field + fmt::Debug> fmt::Debug for DynamicTerm<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynTerm")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicTerm;
    use super::TermError;
    use crate::monomial::DynamicMonomial;
    use algebra_core::{One, Zero};
    use algebra_field::Zp;

    type F7 = Zp<7>;
    type DT = DynamicTerm<F7>;

    #[test]
    fn dyn_term_from_coeff_and_slice_constructs_term() {
        let t = DT::from_coeff_and_slice(F7::new(6), &[2, 0]);

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
    fn dyn_term_degree_matches_monomial_degree() {
        let t = DT::from_coeff_and_slice(F7::new(3), &[1, 2, 0]);
        assert_eq!(t.degree(), 3);
    }

    #[test]
    fn dyn_term_is_zero_uses_field_zero() {
        let zero_term = DT::from_coeff_and_slice(F7::zero(), &[0, 0]);
        let nonzero_term = DT::from_coeff_and_slice(F7::one(), &[0, 0]);

        assert!(zero_term.is_zero());
        assert!(!nonzero_term.is_zero());
    }

    #[test]
    fn dyn_term_mul_scalar_multiplies_coefficient_and_preserves_monomial() {
        let t = DT::from_coeff_and_slice(F7::new(3), &[1, 2]);
        let c = F7::new(5);

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
    fn dyn_term_mul_monomial_adds_exponents_and_preserves_coefficient() {
        let t = DT::from_coeff_and_slice(F7::new(4), &[1, 1]); // 4 * x1^1 x2^1
        let m = DynamicMonomial::from_slice(&[2, 3]); // x1^2 x2^3

        let result = t.mul_monomial(&m);

        assert_eq!(
            result
                .coeff
                .value(),
            4
        );
        assert_eq!(
            result
                .mono
                .exponents(),
            &[3, 4]
        );
    }

    #[test]
    fn dyn_term_checked_mul_monomial_propagates_monomial_errors() {
        let max = u32::MAX;
        let t = DT::from_coeff_and_slice(F7::one(), &[max, 0]);
        let m = DynamicMonomial::from_slice(&[1, 0]);

        let err = t
            .checked_mul_monomial(&m)
            .unwrap_err();
        assert!(matches!(err, TermError::Monomial(_)));
    }
}
