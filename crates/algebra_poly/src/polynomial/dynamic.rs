//! Multivariate polynomials in a runtime number of variables over a field `F`.
//!
//! Representation: sparse sum of [`DynamicTerm<F>`].
//!
//! IMPORTANT:
//! - We implement `Clone/PartialEq/Eq` manually to avoid requiring `O: Clone + PartialEq`.
//! - Normalization is shared via `normalize_terms` in `polynomial/mod.rs`.

use super::{normalize_terms, PolynomialLike, PolynomialMut};
use crate::monomial::{DynamicMonomial, MonomialOrder};
use crate::term::DynamicTerm;
use algebra_core::{Field, Zero};
use core::fmt;
use core::marker::PhantomData;

/// A sparse multivariate polynomial over a field `F` with a runtime number of variables,
/// ordered by `O`.
///
/// Invariants after normalization:
/// - All coefficients are non-zero.
/// - No duplicate monomials.
/// - Terms are sorted in decreasing order w.r.t. `O` (largest first).
pub struct DynamicPolynomial<F: Field, O>
where
    O: MonomialOrder<DynamicMonomial>,
{
    pub(crate) terms: Vec<DynamicTerm<F>>,
    pub(crate) _order: PhantomData<O>,
}

// --- Manual trait impls (avoid bounds on O) ---

impl<F, O> Clone for DynamicPolynomial<F, O>
where
    F: Field + Clone,
    O: MonomialOrder<DynamicMonomial>,
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

impl<F, O> PartialEq for DynamicPolynomial<F, O>
where
    F: Field + PartialEq,
    O: MonomialOrder<DynamicMonomial>,
{
    fn eq(&self, other: &Self) -> bool {
        self.terms == other.terms
    }
}

impl<F, O> Eq for DynamicPolynomial<F, O>
where
    F: Field + Eq,
    O: MonomialOrder<DynamicMonomial>,
{
}

impl<F: Field, O> DynamicPolynomial<F, O>
where
    O: MonomialOrder<DynamicMonomial>,
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

    /// Terms view (not necessarily normalized unless constructors/mutators do so).
    #[inline]
    pub fn terms(&self) -> &[DynamicTerm<F>] {
        &self.terms
    }

    /// Build from raw terms and normalize.
    pub fn from_terms(terms: Vec<DynamicTerm<F>>) -> Self
    where
        F: Zero + Clone,
    {
        let mut p = Self { terms, _order: PhantomData };
        p.normalize();
        p
    }

    /// In-place normalization.
    fn normalize(&mut self)
    where
        F: Zero + Clone,
    {
        normalize_terms::<DynamicTerm<F>, DynamicMonomial, F, O>(&mut self.terms);
    }
}

// --- PolynomialLike impl ---

impl<F, O> PolynomialLike for DynamicPolynomial<F, O>
where
    F: Field + Clone + PartialEq,
    O: MonomialOrder<DynamicMonomial>,
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

    /// Leading term w.r.t. `O`.
    ///
    /// If you maintain normalization invariant (sorted descending), this is O(1).
    /// If you ever allow unsorted terms, swap to a scan-based implementation.
    #[inline]
    fn leading_term(&self) -> Option<&Self::Term> {
        self.terms
            .first()
    }
}

// --- PolynomialMut impl ---

impl<F, O> PolynomialMut for DynamicPolynomial<F, O>
where
    F: Field + Clone + PartialEq,
    O: MonomialOrder<DynamicMonomial>,
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

// --- Debug/Display ---

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

impl<F, O> fmt::Display for DynamicPolynomial<F, O>
where
    F: algebra_core::Field + fmt::Display,
    O: crate::monomial::MonomialOrder<crate::monomial::DynamicMonomial>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }

        for (i, t) in self
            .terms
            .iter()
            .enumerate()
        {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(f, "{t}")?;
        }
        Ok(())
    }
}
