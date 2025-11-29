//! Multivariate polynomials in `N` variables over a field `F`.
//!
//! Representation: sparse sum of terms
//!
//! ```text
//! p(x) = Σ_i coeff_i * x^{α_i}
//! ```
//!
//! where each term is a [`Term<F, N>`].

use super::{PolynomialLike, Term};
use crate::monomial::{Monomial, MonomialOrder};
use algebra_core::{Field, Zero};
use core::cmp::Ordering;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Neg};

/// A sparse multivariate polynomial in `N` variables over a field `F`,
/// with monomials ordered by `O`.
///
/// Invariants:
/// - All coefficients are non-zero.
/// - No duplicate monomials.
/// - Terms are sorted in *decreasing* order w.r.t. `O`.
#[derive(Clone, PartialEq, Eq)]
pub struct Polynomial<F: Field, const N: usize, O: MonomialOrder<Monomial<N>>> {
    pub(crate) terms: Vec<Term<F, N>>,
    pub(crate) _order: PhantomData<O>,
}

impl<F: Field, const N: usize, O> Polynomial<F, N, O>
where
    O: MonomialOrder<Monomial<N>>,
{
    /// The zero polynomial (no terms).
    #[inline]
    pub fn zero() -> Self {
        Self { terms: Vec::new(), _order: PhantomData }
    }

    /// Returns `true` if this is the zero polynomial.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.terms
            .is_empty()
    }

    /// Immutable view of the internal term list (highest to lowest).
    #[inline]
    pub fn terms(&self) -> &[Term<F, N>] {
        &self.terms
    }

    /// Constructs a polynomial from a raw list of terms.
    ///
    /// This will:
    /// - drop terms with zero coefficient,
    /// - combine terms with the same monomial,
    /// - sort terms in decreasing order w.r.t. `O`.
    pub fn from_terms(terms: Vec<Term<F, N>>) -> Self
    where
        F: Zero + Clone,
    {
        let mut p = Self { terms, _order: PhantomData };
        p.normalize();
        p
    }

    /// Construct `coeff * mono` as a polynomial (or zero if `coeff == 0`).
    #[inline]
    pub fn from_monomial(coeff: F, mono: Monomial<N>) -> Self
    where
        F: Zero + Clone,
    {
        if coeff == F::zero() {
            Self::zero()
        } else {
            Self::from_terms(vec![Term::new(coeff, mono)])
        }
    }

    /// Construct a polynomial from a single term (or zero if that term is zero).
    #[inline]
    pub fn from_term(term: Term<F, N>) -> Self
    where
        F: Zero + Clone,
    {
        if term.is_zero() { Self::zero() } else { Self::from_terms(vec![term]) }
    }

    /// Leading term w.r.t. `O` (if non-zero).
    #[inline]
    pub fn lt(&self) -> Option<&Term<F, N>> {
        self.terms
            .first()
    }

    /// Leading monomial (`lm`) w.r.t. `O`.
    #[inline]
    pub fn lm(&self) -> Option<&Monomial<N>> {
        self.lt()
            .map(|t| &t.mono)
    }

    /// Leading coefficient (`LC`) w.r.t. `O`.
    #[inline]
    pub fn leading_coefficient(&self) -> Option<&F> {
        self.lt()
            .map(|t| &t.coeff)
    }

    /// Total degree of the polynomial = degree of the leading monomial,
    /// or `None` for the zero polynomial.
    #[inline]
    pub fn degree(&self) -> Option<u64> {
        self.lm()
            .map(|m| m.degree())
    }

    /// Internal normalization:
    /// - drop zero coefficients,
    /// - combine like monomials,
    /// - sort in decreasing order w.r.t. `O`.
    fn normalize(&mut self)
    where
        F: Zero + Clone,
    {
        // Drop zero terms first.
        self.terms
            .retain(|t| !t.is_zero());

        if self
            .terms
            .is_empty()
        {
            return;
        }

        // Sort by monomial in decreasing order: largest first.
        self.terms
            .sort_by(|a, b| O::cmp(&a.mono, &b.mono).reverse());

        // Combine like terms in a single pass.
        let mut normalized: Vec<Term<F, N>> = Vec::with_capacity(
            self.terms
                .len(),
        );

        for term in self
            .terms
            .drain(..)
        {
            if let Some(last) = normalized.last_mut() {
                if last.mono == term.mono {
                    // Same monomial: add coefficients.
                    let sum = last
                        .coeff
                        .clone()
                        + term
                            .coeff
                            .clone();
                    if sum == F::zero() {
                        // Coefficients cancel → drop the term.
                        normalized.pop();
                    } else {
                        last.coeff = sum;
                    }
                    continue;
                }
            }

            // Different monomial, or first term.
            if !term.is_zero() {
                normalized.push(term);
            }
        }

        self.terms = normalized;
    }

    /// Polynomial addition: `self + other` (returns a new polynomial).
    ///
    /// Assumes both inputs are already normalized.
    pub fn add_ref(&self, other: &Self) -> Self
    where
        F: Zero + Clone,
    {
        let mut result: Vec<Term<F, N>> = Vec::with_capacity(
            self.terms
                .len()
                + other
                    .terms
                    .len(),
        );

        let mut i = 0;
        let mut j = 0;

        while i < self
            .terms
            .len()
            && j < other
                .terms
                .len()
        {
            let ti = &self.terms[i];
            let tj = &other.terms[j];

            match O::cmp(&ti.mono, &tj.mono) {
                Ordering::Greater => {
                    // ti.mono > tj.mono → keep ti first
                    result.push(ti.clone());
                    i += 1;
                }
                Ordering::Less => {
                    // tj.mono > ti.mono → keep tj first
                    result.push(tj.clone());
                    j += 1;
                }
                Ordering::Equal => {
                    // Same monomial: add coefficients, maybe drop if zero.
                    let sum = ti
                        .coeff
                        .clone()
                        + tj.coeff
                            .clone();
                    if sum != F::zero() {
                        result.push(Term::new(sum, ti.mono));
                    }
                    i += 1;
                    j += 1;
                }
            }
        }

        // Append any remaining terms.
        while i < self
            .terms
            .len()
        {
            result.push(self.terms[i].clone());
            i += 1;
        }
        while j < other
            .terms
            .len()
        {
            result.push(other.terms[j].clone());
            j += 1;
        }

        Self { terms: result, _order: PhantomData }
    }

    /// In-place polynomial addition: `self += other`.
    ///
    /// This is implemented by computing the sum and assigning it back.
    pub fn add_assign_ref(&mut self, other: &Self)
    where
        F: Zero + Clone,
    {
        let sum = self.add_ref(other);
        *self = sum;
    }

    /// Negation: `-self`.
    ///
    /// Mathematically: `-(Σ c_i x^{α_i}) = Σ (-c_i) x^{α_i}`.
    pub fn neg_ref(&self) -> Self
    where
        F: Clone,
    {
        let terms = self
            .terms
            .iter()
            .map(|t| {
                Term::new(
                    -t.coeff
                        .clone(),
                    t.mono,
                )
            })
            .collect();

        Self { terms, _order: PhantomData }
    }
}

// ----- std trait impls -----

impl<F, const N: usize, O> Add for Polynomial<F, N, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<Monomial<N>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Polynomial::add_ref(&self, &rhs)
    }
}

impl<F, const N: usize, O> Add<&Polynomial<F, N, O>> for Polynomial<F, N, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<Monomial<N>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        Polynomial::add_ref(&self, rhs)
    }
}

impl<F, const N: usize, O> AddAssign for Polynomial<F, N, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<Monomial<N>>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign_ref(&rhs);
    }
}

impl<F, const N: usize, O> Neg for Polynomial<F, N, O>
where
    F: Field + Clone,
    O: MonomialOrder<Monomial<N>>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.neg_ref()
    }
}

impl<F: Field + fmt::Debug, const N: usize, O> fmt::Debug for Polynomial<F, N, O>
where
    O: MonomialOrder<Monomial<N>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Polynomial")
            .field("terms", &self.terms)
            .finish()
    }
}

// ----- PolynomialLike impl -----

impl<F, const N: usize, O> PolynomialLike for Polynomial<F, N, O>
where
    F: Field + Clone + PartialEq,
    O: MonomialOrder<Monomial<N>> + std::clone::Clone + std::cmp::PartialEq,
{
    type Field = F;
    type Mono = Monomial<N>;
    type Order = O;
    type Term = Term<F, N>;

    #[inline]
    fn is_zero(&self) -> bool {
        self.terms
            .is_empty()
    }

    #[inline]
    fn terms(&self) -> &[Self::Term] {
        &self.terms
    }

    #[inline]
    fn leading_term(&self) -> Option<&Self::Term> {
        self.terms
            .first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{Grevlex, Lex};
    use algebra_core::{One, Zero};
    use algebra_field::Zp;

    type F7 = Zp<7>;
    type P2Lex = Polynomial<F7, 2, Lex>;
    type P2Grev = Polynomial<F7, 2, Grevlex>;

    #[test]
    fn zero_polynomial_has_no_terms_and_no_degree() {
        let p: P2Lex = P2Lex::zero();
        assert!(p.is_zero());
        assert!(
            p.lt()
                .is_none()
        );
        assert_eq!(p.degree(), None);
    }

    #[test]
    fn from_monomial_builds_single_term_polynomial() {
        let mono = Monomial::<2>::from_exponents([1, 2]);
        let coeff = F7::new(5);
        let p: P2Grev = P2Grev::from_monomial(coeff, mono);

        assert!(!p.is_zero());
        let lt = p
            .lt()
            .unwrap();
        assert_eq!(
            lt.mono
                .exponents(),
            &[1, 2]
        );
        assert_eq!(
            lt.coeff
                .value(),
            5
        );
    }

    #[test]
    fn from_terms_normalizes_duplicates_and_zeros() {
        let m1 = Monomial::<2>::from_exponents([1, 0]); // x1
        let m2 = Monomial::<2>::from_exponents([0, 1]); // x2

        let terms = vec![
            Term::new(F7::one(), m1),
            Term::new(F7::one(), m2),
            Term::new(F7::one(), m1),       // duplicate m1
            Term::new(F7::zero(), m2),      // zero coeff
            Term::new(F7::one().neg(), m2), // cancels x2
        ];

        let p: P2Lex = P2Lex::from_terms(terms);

        // x1 + x2 + x1 + 0 - x2 = 2 x1
        assert!(!p.is_zero());
        assert_eq!(
            p.terms()
                .len(),
            1
        );
        let t = &p.terms()[0];
        assert_eq!(t.mono, m1);
        assert_eq!(t.coeff, F7::one() + F7::one());
    }

    #[test]
    fn addition_combines_like_terms_and_drops_zero_coeffs() {
        let m1 = Monomial::<2>::from_exponents([1, 0]); // x1
        let m2 = Monomial::<2>::from_exponents([0, 1]); // x2

        let p1: P2Lex = P2Lex::from_terms(vec![Term::new(F7::one(), m1), Term::new(F7::one(), m2)]);

        let p2: P2Lex = P2Lex::from_terms(vec![
            Term::new(F7::one(), m1),
            Term::new(F7::one().neg(), m2),
        ]);

        // x1 + x2 + x1 - x2 = 2 x1
        let sum = p1.add_ref(&p2);
        assert!(!sum.is_zero());
        assert_eq!(
            sum.terms()
                .len(),
            1
        );
        assert_eq!(sum.terms()[0].mono, m1);
        assert_eq!(sum.terms()[0].coeff, F7::one() + F7::one());
    }

    #[test]
    fn neg_flips_all_coefficients() {
        let m1 = Monomial::<2>::from_exponents([1, 0]);
        let m2 = Monomial::<2>::from_exponents([0, 1]);

        let p: P2Grev =
            P2Grev::from_terms(vec![Term::new(F7::new(3), m1), Term::new(F7::new(4), m2)]);

        let q = -p.clone();
        assert_eq!(
            q.terms()
                .len(),
            2
        );
        assert_eq!(q.terms()[0].coeff, -p.terms()[0].coeff);
        assert_eq!(q.terms()[1].coeff, -p.terms()[1].coeff);
    }
}
