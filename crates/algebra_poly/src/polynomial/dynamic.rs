//! Multivariate polynomials in a runtime number of variables over a field `F`.
//!
//! Representation: sparse sum of [`DynTerm<F>`].

use super::PolynomialLike;
use crate::monomial::{DynamicMonomial, MonomialOrder};
use crate::term::DynamicTerm;
use algebra_core::{Field, Zero};
use core::cmp::Ordering;
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Neg};

/// A sparse multivariate polynomial over a field `F` with a
/// **runtime-chosen** number of variables, ordered by `O`.
///
/// Invariants:
/// - All coefficients are non-zero.
/// - No duplicate monomials.
/// - Terms are sorted in *decreasing* order w.r.t. `O`.
#[derive(Clone, PartialEq, Eq)]
pub struct DynamicPolynomial<F: Field, O: MonomialOrder<DynamicMonomial>> {
    pub(crate) terms: Vec<DynamicTerm<F>>,
    pub(crate) _order: PhantomData<O>,
}

impl<F: Field, O> DynamicPolynomial<F, O>
where
    O: MonomialOrder<DynamicMonomial>,
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
    pub fn terms(&self) -> &[DynamicTerm<F>] {
        &self.terms
    }

    /// Constructs a polynomial from a raw list of terms.
    ///
    /// This will:
    /// - drop terms with zero coefficient,
    /// - combine terms with the same monomial,
    /// - sort terms in decreasing order w.r.t. `O`.
    pub fn from_terms(terms: Vec<DynamicTerm<F>>) -> Self
    where
        F: Zero + Clone,
    {
        let mut p = Self { terms, _order: PhantomData };
        p.normalize();
        p
    }

    /// Construct `coeff * mono` as a polynomial (or zero if `coeff == 0`).
    #[inline]
    pub fn from_monomial(coeff: F, mono: DynamicMonomial) -> Self
    where
        F: Zero + Clone,
    {
        if coeff == F::zero() {
            Self::zero()
        } else {
            Self::from_terms(vec![DynamicTerm::new(coeff, mono)])
        }
    }

    /// Construct a polynomial from a single term (or zero if that term is zero).
    #[inline]
    pub fn from_term(term: DynamicTerm<F>) -> Self
    where
        F: Zero + Clone,
    {
        if term.is_zero() { Self::zero() } else { Self::from_terms(vec![term]) }
    }

    /// Leading term w.r.t. `O` (if non-zero).
    #[inline]
    pub fn lt(&self) -> Option<&DynamicTerm<F>> {
        self.terms
            .first()
    }

    /// Leading monomial (`lm`) w.r.t. `O`.
    #[inline]
    pub fn lm(&self) -> Option<&DynamicMonomial> {
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
        let mut normalized: Vec<DynamicTerm<F>> = Vec::with_capacity(
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
        let mut result: Vec<DynamicTerm<F>> = Vec::with_capacity(
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
                    result.push(ti.clone());
                    i += 1;
                }
                Ordering::Less => {
                    result.push(tj.clone());
                    j += 1;
                }
                Ordering::Equal => {
                    let sum = ti
                        .coeff
                        .clone()
                        + tj.coeff
                            .clone();
                    if sum != F::zero() {
                        result.push(DynamicTerm::new(
                            sum,
                            ti.mono
                                .clone(),
                        ));
                    }
                    i += 1;
                    j += 1;
                }
            }
        }

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
    pub fn add_assign_ref(&mut self, other: &Self)
    where
        F: Zero + Clone,
    {
        let sum = self.add_ref(other);
        *self = sum;
    }

    /// Negation: `-self`.
    pub fn neg_ref(&self) -> Self
    where
        F: Clone,
    {
        let terms = self
            .terms
            .iter()
            .map(|t| {
                DynamicTerm::new(
                    -t.coeff
                        .clone(),
                    t.mono
                        .clone(),
                )
            })
            .collect();

        Self { terms, _order: PhantomData }
    }
}

// ----- std trait impls -----

impl<F, O> Add for DynamicPolynomial<F, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<DynamicMonomial>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        DynamicPolynomial::add_ref(&self, &rhs)
    }
}

impl<F, O> Add<&DynamicPolynomial<F, O>> for DynamicPolynomial<F, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<DynamicMonomial>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: &Self) -> Self::Output {
        DynamicPolynomial::add_ref(&self, rhs)
    }
}

impl<F, O> AddAssign for DynamicPolynomial<F, O>
where
    F: Field + Zero + Clone,
    O: MonomialOrder<DynamicMonomial>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign_ref(&rhs);
    }
}

impl<F, O> Neg for DynamicPolynomial<F, O>
where
    F: Field + Clone,
    O: MonomialOrder<DynamicMonomial>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.neg_ref()
    }
}

impl<F, O> fmt::Debug for DynamicPolynomial<F, O>
where
    F: Field + fmt::Debug,
    O: MonomialOrder<DynamicMonomial>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DynamicPolynomial")
            .field("terms", &self.terms)
            .finish()
    }
}

// ----- PolynomialLike impl -----

impl<F, O> PolynomialLike for DynamicPolynomial<F, O>
where
    F: Field + Clone,
    O: MonomialOrder<DynamicMonomial> + Clone + PartialEq,
{
    type Field = F;
    type Mono = DynamicMonomial;
    type Order = O;
    type Term = DynamicTerm<F>;

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
