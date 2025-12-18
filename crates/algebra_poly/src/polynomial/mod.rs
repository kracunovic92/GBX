//! Sparse multivariate polynomials over a field.
//!
//! This module provides two concrete polynomial representations:
//!
//! - [`Polynomial<F, N, O>`]: compile-time number of variables (`N`)
//! - [`DynamicPolynomial<F, O>`]: runtime number of variables
//!
//! Both share a common read-only interface [`PolynomialLike`] used by algorithms.
//!
//! Design goals:
//! - Algorithms (division, reduction, Buchberger) depend only on `PolynomialLike`
//!   (read-only) and `PolynomialMut` (minimal mutation / constructors).
//! - Concrete types keep fast invariants: normalized sparse vector of terms.
//! - We do NOT require the order type `O` to be `Clone` or `PartialEq`.
//!   We implement `Clone/PartialEq` for polynomials manually and ignore PhantomData.

use crate::monomial::{MonomialError, MonomialLike, MonomialOrder};
use crate::term::{TermError, TermLike};
use algebra_core::{Additive, Field, Zero};

mod division;
pub mod dynamic;
mod polynomial;

pub use division::{divide, DivisionResult};
pub use dynamic::DynamicPolynomial;
pub use polynomial::Polynomial;

/// Errors that can occur when operating on polynomials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PolynomialError {
    /// Error originating from term-level operations.
    Term(TermError),
}

impl From<TermError> for PolynomialError {
    #[inline]
    fn from(err: TermError) -> Self {
        PolynomialError::Term(err)
    }
}

impl From<MonomialError> for PolynomialError {
    #[inline]
    fn from(err: MonomialError) -> Self {
        PolynomialError::Term(TermError::from(err))
    }
}

/// Common interface for sparse polynomials.
///
/// NOTE: this trait requires `Clone + PartialEq` because many algorithms
/// naturally clone polynomials (work on a mutable copy) and compare against zero.
///
/// IMPORTANT: `Clone + PartialEq` applies to the polynomial type `Self`,
/// NOT the order type `O`. Concrete polynomial types can implement `Clone/PartialEq`
/// manually so `O` does not need those bounds.
pub trait PolynomialLike: Clone + PartialEq {
    /// The underlying field of coefficients.
    type Field: Field;

    /// The monomial representation.
    type Mono: MonomialLike<Error = MonomialError> + Clone;

    /// The monomial order used by algorithms.
    type Order: MonomialOrder<Self::Mono>;

    /// The sparse term type used internally.
    type Term: TermLike<Field = Self::Field, Mono = Self::Mono, Error = TermError> + Clone;

    /// Returns `true` if this is the zero polynomial.
    fn is_zero(&self) -> bool;

    /// Returns an immutable view of internal terms.
    fn terms(&self) -> &[Self::Term];

    /// Leading term w.r.t. the monomial order (if non-zero).
    ///
    /// Concrete types may return `terms().first()` if they maintain the invariant
    /// that terms are sorted in decreasing order by `Order`.
    fn leading_term(&self) -> Option<&Self::Term>;

    /// Leading monomial (`lm`) w.r.t. the monomial order.
    #[inline]
    fn leading_monomial(&self) -> Option<&Self::Mono> {
        self.leading_term()
            .map(|t| t.mono())
    }

    /// Leading coefficient (`lc`) w.r.t. the monomial order.
    #[inline]
    fn leading_coefficient(&self) -> Option<&Self::Field> {
        self.leading_term()
            .map(|t| t.coeff())
    }

    /// Total degree of the polynomial, if non-zero.
    ///
    /// Defined as the degree of the leading monomial.
    #[inline]
    fn degree(&self) -> Option<u64> {
        self.leading_monomial()
            .map(|m| m.degree())
    }
}

/// Minimal mutation hooks needed for generic algorithms.
///
/// Algorithms need:
/// - `zero()`
/// - a way to construct from terms (`from_terms`), which normalizes
/// - a way to push and normalize (for incremental building)
pub trait PolynomialMut: PolynomialLike {
    /// Creates the zero polynomial.
    fn zero() -> Self;

    /// Builds a polynomial from raw terms (must normalize internally).
    fn from_terms(terms: Vec<Self::Term>) -> Self
    where
        Self::Field: Zero + Clone;

    /// Push a single term (may violate invariants).
    fn push_term(&mut self, term: Self::Term);

    /// Normalize in-place:
    /// - drop zero coeffs
    /// - combine like monomials
    /// - sort by `Order` (largest first)
    fn normalize_in_place(&mut self)
    where
        Self::Field: Zero + Clone;

    /// Add a term and normalize.
    #[inline]
    fn add_term_normalized(&mut self, term: Self::Term)
    where
        Self::Field: Zero + Clone,
    {
        self.push_term(term);
        self.normalize_in_place();
    }

    /// Build from a single term (or 0 if term is zero).
    #[inline]
    fn from_term(term: Self::Term) -> Self
    where
        Self::Field: Zero + Clone,
    {
        if term.is_zero() { Self::zero() } else { Self::from_terms(vec![term]) }
    }

    /// Generic addition `a + b` implemented by concatenating term lists and normalizing.
    ///
    /// This is algorithm-friendly and avoids requiring `Add` on the polynomial type.
    #[inline]
    fn add_poly(a: &Self, b: &Self) -> Self
    where
        Self::Field: Zero + Clone,
    {
        if a.is_zero() {
            return b.clone();
        }
        if b.is_zero() {
            return a.clone();
        }

        let mut terms = Vec::with_capacity(
            a.terms()
                .len()
                + b.terms()
                    .len(),
        );
        for t in a.terms() {
            terms.push(t.clone());
        }
        for t in b.terms() {
            terms.push(t.clone());
        }
        Self::from_terms(terms)
    }

    /// Negate a polynomial term-wise and normalize.
    #[inline]
    fn neg_poly(p: &Self) -> Self
    where
        Self::Field: Zero + Clone + core::ops::Neg<Output = Self::Field>,
    {
        if p.is_zero() {
            return Self::zero();
        }

        let mut terms = Vec::with_capacity(
            p.terms()
                .len(),
        );
        for t in p.terms() {
            terms.push(Self::Term::from_parts(
                -t.coeff()
                    .clone(),
                t.mono()
                    .clone(),
            ));
        }
        Self::from_terms(terms)
    }

    /// Generic subtraction `a - b` implemented by appending `-b` terms to `a`
    /// and normalizing.
    #[inline]
    fn sub_poly(a: &Self, b: &Self) -> Self
    where
        Self::Field: Zero + Clone + core::ops::Neg<Output = Self::Field>,
    {
        if b.is_zero() {
            return a.clone();
        }
        Self::add_poly(a, &Self::neg_poly(b))
    }
}

/// Shared term normalization for both static and dynamic polynomials.
///
/// This removes nearly all duplicated code between the two implementations.
///
/// Steps:
/// 1) drop zero terms
/// 2) sort by monomial order (largest first)
/// 3) merge consecutive equal monomials (because after sorting they are adjacent)
pub(crate) fn normalize_terms<TermT, MonoT, F, O>(terms: &mut Vec<TermT>)
where
    F: Zero + Clone + Additive,
    MonoT: MonomialLike + Clone,
    O: MonomialOrder<MonoT>,
    TermT: TermLike<Field = F, Mono = MonoT> + Clone,
{
    // 1) drop zeros
    terms.retain(|t| !t.is_zero());
    if terms.is_empty() {
        return;
    }

    // 2) sort by order descending (largest first)
    terms.sort_by(|a, b| O::cmp(a.mono(), b.mono()).reverse());

    // 3) merge equal monomials (adjacent after sort)
    let mut out: Vec<TermT> = Vec::with_capacity(terms.len());

    for t in terms.drain(..) {
        if let Some(last) = out.last_mut() {
            if last.mono() == t.mono() {
                // sum := last.coeff + t.coeff  (trait-based, no `+`)
                let sum = Additive::add(
                    last.coeff()
                        .clone(),
                    t.coeff()
                        .clone(),
                );

                if sum.is_zero() {
                    // coefficients cancelled
                    out.pop();
                } else {
                    // rebuild the term with the same monomial
                    let mono = last
                        .mono()
                        .clone();
                    *last = TermT::from_parts(sum, mono);
                }
                continue;
            }
        }
        out.push(t);
    }

    *terms = out;
}

/// Generic helper: multiply a term `m` into a polynomial `g`.
///
/// Returns `m * g`.
pub fn mul_term_poly<P>(m: &P::Term, g: &P) -> Result<P, PolynomialError>
where
    P: PolynomialMut,
    P::Field: Zero + Clone,
{
    if g.is_zero() || m.is_zero() {
        return Ok(P::zero());
    }

    let mut out = Vec::with_capacity(
        g.terms()
            .len(),
    );

    for t in g.terms() {
        let prod = t
            .checked_mul_term(m)
            .map_err(PolynomialError::from)?;
        out.push(prod);
    }

    Ok(P::from_terms(out))
}
