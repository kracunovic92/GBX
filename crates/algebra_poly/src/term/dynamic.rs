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
    pub fn from_coeff_and_slice(coeff: F, exponents: &[u64]) -> Self {
        Self { coeff, mono: DynamicMonomial::from_slice(exponents) }
    }

    /// Constructs a term from a coefficient and a `Vec<u32>` of exponents.
    #[inline]
    pub fn from_coeff_and_vec(coeff: F, exponents: Vec<u64>) -> Self {
        Self { coeff, mono: DynamicMonomial::from_vec(exponents) }
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
        self.coeff
            .is_zero()
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

    /// Multiplies this term by a dynamic monomial.
    ///
    /// # Panics
    ///
    /// Panics if exponent addition overflows or if the variable counts do not match.
    /// Prefer [`Self::checked_mul_monomial`] in algorithmic code.
    #[inline]
    pub fn mul_monomial(&self, m: &DynamicMonomial) -> Self
    where
        F: Clone,
    {
        <Self as TermLike>::mul_monomial(self, m)
    }

    /// Multiplies two terms.
    ///
    /// # Panics
    ///
    /// Panics if monomial multiplication fails (overflow or mismatched variable counts).
    /// Prefer [`Self::checked_mul_term`] in algorithmic code.
    #[inline]
    pub fn mul_term(&self, other: &Self) -> Self
    where
        F: Clone,
    {
        self.checked_mul_term(other)
            .expect("DynamicTerm::mul_term: monomial multiplication failed")
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
    fn from_parts(coeff: Self::Field, mono: Self::Mono) -> Self {
        Self { coeff, mono }
    }

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
}

impl<F: Field + fmt::Debug> fmt::Debug for DynamicTerm<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynamicTerm")
            .field("coeff", &self.coeff)
            .field("mono", &self.mono)
            .finish()
    }
}

impl<F> fmt::Display for DynamicTerm<F>
where
    F: Field + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self
            .mono
            .is_one()
        {
            write!(f, "{}", self.coeff)
        } else {
            write!(f, "{}*{}", self.coeff, self.mono)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicTerm;
    use crate::monomial::DynamicMonomial;
    use crate::term::TermLike;
    use algebra_core::{One, Zero};
    use algebra_field::Zp;

    type F7 = Zp<7>;
    type DT = DynamicTerm<F7>;

    #[test]
    fn from_coeff_and_slice_constructs_term() {
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
    fn degree_matches_monomial_degree() {
        let t = DT::from_coeff_and_slice(F7::new(3), &[1, 2, 0]);
        assert_eq!(t.degree(), 3);
    }

    #[test]
    fn is_zero_detects_zero_and_nonzero_coefficients() {
        let zero_term = DT::from_coeff_and_slice(F7::zero(), &[0, 0]);
        let nonzero_term = DT::from_coeff_and_slice(F7::one(), &[0, 0]);
        assert!(zero_term.is_zero());
        assert!(!nonzero_term.is_zero());
    }

    #[test]
    fn mul_scalar_multiplies_coefficient_and_preserves_monomial() {
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
    fn mul_monomial_uses_trait_default_and_adds_exponents() {
        let t = DT::from_coeff_and_slice(F7::new(4), &[1, 1]);
        let m = DynamicMonomial::from_slice(&[2, 3]);

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
    fn checked_mul_monomial_errors_on_mismatched_variable_counts() {
        let t = DT::from_coeff_and_slice(F7::one(), &[1, 2]);
        let m = DynamicMonomial::from_slice(&[1, 2, 3]);

        assert!(
            t.checked_mul_monomial(&m)
                .is_err()
        );
    }

    #[test]
    fn mul_term_multiplies_coeffs_and_adds_exponents() {
        let a = DT::from_coeff_and_slice(F7::new(3), &[1, 2]);
        let b = DT::from_coeff_and_slice(F7::new(5), &[2, 1]);

        let c = a.mul_term(&b);

        // 3*5 = 15 ≡ 1 (mod 7)
        assert_eq!(
            c.coeff
                .value(),
            1
        );
        assert_eq!(
            c.mono
                .exponents(),
            &[3, 3]
        );
    }

    #[test]
    fn checked_mul_term_propagates_monomial_overflow() {
        let max = u64::MAX;
        let t = DT::from_coeff_and_slice(F7::one(), &[max, 0]);
        let u = DT::from_coeff_and_slice(F7::one(), &[1, 0]);

        assert!(
            t.checked_mul_term(&u)
                .is_err()
        );
    }
}
