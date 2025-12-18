//! Multivariate polynomials in `N` variables over a field `F`.
//!
//! Representation: sparse sum of terms, normalized and ordered by `O`.
//!
//! IMPORTANT:
//! - We implement `Clone/PartialEq/Eq` manually to avoid requiring `O: Clone + PartialEq`.
//! - Normalization is shared via `normalize_terms`.

use super::{normalize_terms, PolynomialLike, PolynomialMut};
use crate::monomial::{Monomial, MonomialOrder};
use crate::term::Term;
use algebra_core::{Field, Zero};
use core::fmt;
use core::marker::PhantomData;

/// A sparse multivariate polynomial in `N` variables over a field `F`,
/// with monomials ordered by `O`.
///
/// Invariants after normalization:
/// - All coefficients are non-zero.
/// - No duplicate monomials.
/// - Terms are sorted in decreasing order w.r.t. `O` (largest first).
pub struct Polynomial<F: Field, const N: usize, O: MonomialOrder<Monomial<N>>> {
    pub(crate) terms: Vec<Term<F, N>>,
    pub(crate) _order: PhantomData<O>,
}

// --- Manual trait impls (avoid bounds on O) ---

impl<F, const N: usize, O> Clone for Polynomial<F, N, O>
where
    F: Field + Clone,
    O: MonomialOrder<Monomial<N>>,
{
    fn clone(&self) -> Self {
        Self {
            terms: self
                .terms
                .clone(),
            _order: PhantomData,
        }
    }
}

impl<F, const N: usize, O> PartialEq for Polynomial<F, N, O>
where
    F: Field + PartialEq,
    O: MonomialOrder<Monomial<N>>,
{
    fn eq(&self, other: &Self) -> bool {
        self.terms == other.terms
    }
}

impl<F, const N: usize, O> Eq for Polynomial<F, N, O>
where
    F: Field + Eq,
    O: MonomialOrder<Monomial<N>>,
{
}

impl<F: Field, const N: usize, O> Polynomial<F, N, O>
where
    O: MonomialOrder<Monomial<N>>,
{
    /// Zero polynomial.
    #[inline]
    pub fn zero() -> Self {
        Self { terms: Vec::new(), _order: PhantomData }
    }

    /// Is zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.terms
            .is_empty()
    }

    /// Terms view.
    #[inline]
    pub fn terms(&self) -> &[Term<F, N>] {
        &self.terms
    }

    /// Build from raw terms and normalize.
    pub fn from_terms(terms: Vec<Term<F, N>>) -> Self
    where
        F: Zero + Clone,
    {
        let mut p = Self { terms, _order: PhantomData };
        p.normalize();
        p
    }

    fn normalize(&mut self)
    where
        F: Zero + Clone,
    {
        normalize_terms::<Term<F, N>, Monomial<N>, F, O>(&mut self.terms);
    }
}

// --- PolynomialLike impl ---

impl<F, const N: usize, O> PolynomialLike for Polynomial<F, N, O>
where
    F: Field + Clone + PartialEq,
    O: MonomialOrder<Monomial<N>>,
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

// --- PolynomialMut impl ---

impl<F, const N: usize, O> PolynomialMut for Polynomial<F, N, O>
where
    F: Field + Clone + PartialEq,
    O: MonomialOrder<Monomial<N>>,
{
    fn zero() -> Self {
        Self::zero()
    }

    fn from_terms(terms: Vec<Self::Term>) -> Self
    where
        Self::Field: Zero + Clone,
    {
        Self::from_terms(terms)
    }

    fn push_term(&mut self, term: Self::Term) {
        self.terms
            .push(term);
    }

    fn normalize_in_place(&mut self)
    where
        Self::Field: Zero + Clone,
    {
        normalize_terms::<Self::Term, Self::Mono, Self::Field, Self::Order>(&mut self.terms);
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
