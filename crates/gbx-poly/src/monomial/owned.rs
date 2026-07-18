use core::fmt;

use crate::monomial::display::MonomialDisplay;
use crate::monomial::error::{MonomialError, MonomialResult};
use crate::monomial::traits::MonomialView;

/// Runtime-sized monomial.
///
/// Stores an exponent vector and a cached total degree.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Monomial {
    exps: Box<[u32]>,
    degree: u32,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MonomialId(u32);

impl Monomial {
    /// Creates the multiplicative identity monomial `1` with `n_vars` variables.
    #[inline]
    #[must_use]
    pub fn one(n_vars: usize) -> Self {
        Self { exps: vec![0u32; n_vars].into_boxed_slice(), degree: 0 }
    }

    /// Constructs a monomial from a borrowed exponent slice.
    ///
    /// # Errors
    ///
    /// Returns [`MonomialError::DegreeOverflow`] if the total degree overflows `u32`.
    #[inline]
    pub fn try_from_slice(exponents: &[u32]) -> MonomialResult<Self> {
        let degree = checked_degree(exponents)?;

        Ok(Self { exps: exponents.into(), degree })
    }

    /// Constructs a monomial from a borrowed exponent slice.
    ///
    /// # Panics
    ///
    /// Panics if the total degree overflows `u32`.
    #[inline]
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn from_slice(exponents: &[u32]) -> Self {
        Self::try_from_slice(exponents).expect("Monomial::from_slice failed: total degree overflow")
    }

    /// Constructs a monomial from an owned exponent vector without copying.
    ///
    /// # Errors
    ///
    /// Returns [`MonomialError::DegreeOverflow`] if the total degree overflows `u32`.
    #[inline]
    pub fn try_from_vec(exponents: Vec<u32>) -> MonomialResult<Self> {
        let degree = checked_degree(&exponents)?;

        Ok(Self { exps: exponents.into_boxed_slice(), degree })
    }

    /// Constructs a monomial from an owned exponent vector without copying.
    ///
    /// # Panics
    ///
    /// Panics if the total degree overflows `u32`.
    #[inline]
    #[must_use]
    #[allow(clippy::expect_used)]
    pub fn from_vec(exponents: Vec<u32>) -> Self {
        Self::try_from_vec(exponents).expect("Monomial::from_vec failed: total degree overflow")
    }

    /// Constructs a monomial from exactly `n_vars` exponents.
    ///
    /// # Errors
    ///
    /// Returns [`MonomialError::WrongLength`] if the iterator does not yield exactly
    /// `n_vars` exponents, or [`MonomialError::DegreeOverflow`] if the total degree
    /// overflows `u32`.
    #[inline]
    pub fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> MonomialResult<Self>
    where
        I: IntoIterator<Item = u32>,
    {
        let mut it = exps.into_iter();

        let (lower, upper) = it.size_hint();
        if upper == Some(lower) && lower != n_vars {
            return Err(MonomialError::WrongLength { expected: n_vars, got: lower });
        }

        let mut v = Vec::with_capacity(n_vars);
        let mut degree = 0u32;

        for i in 0..n_vars {
            match it.next() {
                Some(e) => {
                    degree = degree.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
                    v.push(e);
                }
                None => {
                    return Err(MonomialError::WrongLength { expected: n_vars, got: i });
                }
            }
        }

        if it.next().is_some() {
            let extra_rest = it.count();

            return Err(MonomialError::WrongLength { expected: n_vars, got: n_vars + 1 + extra_rest });
        }

        Ok(Self { exps: v.into_boxed_slice(), degree })
    }

    /// Returns the exponent slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[u32] {
        &self.exps
    }

    /// Returns the number of variables.
    #[inline]
    #[must_use]
    pub fn n_vars(&self) -> usize {
        self.exps.len()
    }

    /// Returns the cached total degree.
    #[inline]
    #[must_use]
    pub const fn degree(&self) -> u32 {
        self.degree
    }

    /// Returns `true` if this is the multiplicative identity monomial.
    #[inline]
    #[must_use]
    pub const fn is_one(&self) -> bool {
        self.degree == 0
    }

    /// Checked multiplication.
    ///
    /// # Errors
    ///
    /// Returns [`MonomialError::MismatchedArity`] when the monomials have different
    /// variable counts, [`MonomialError::DegreeOverflow`] when the total degree
    /// overflows, or [`MonomialError::ExponentOverflow`] when an individual exponent
    /// addition overflows.
    #[inline]
    pub fn checked_mul(&self, other: &Self) -> MonomialResult<Self> {
        let n = self.n_vars();

        if n != other.n_vars() {
            return Err(MonomialError::MismatchedArity { lhs: n, rhs: other.n_vars() });
        }

        let degree = self
            .degree
            .checked_add(other.degree)
            .ok_or(MonomialError::DegreeOverflow)?;

        let mut out = Vec::with_capacity(n);

        for (i, (&a, &b)) in self
            .exponents()
            .iter()
            .zip(other.exponents().iter())
            .enumerate()
        {
            out.push(
                a.checked_add(b)
                    .ok_or(MonomialError::ExponentOverflow { index: i, lhs: a, rhs: b })?,
            );
        }

        Ok(Self { exps: out.into_boxed_slice(), degree })
    }
}

impl MonomialView for Monomial {
    #[inline]
    fn exponents(&self) -> &[u32] {
        &self.exps
    }

    #[inline]
    fn degree(&self) -> u32 {
        self.degree
    }
}

impl Default for Monomial {
    #[inline]
    fn default() -> Self {
        Self::one(0)
    }
}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MonomialDisplay::product(self).fmt(f)
    }
}

impl<const N: usize> From<[u32; N]> for Monomial {
    #[inline]
    fn from(exps: [u32; N]) -> Self {
        Self::from_slice(&exps)
    }
}

#[inline]
fn checked_degree(exponents: &[u32]) -> MonomialResult<u32> {
    let mut degree = 0u32;

    for &e in exponents {
        degree = degree.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
    }

    Ok(degree)
}
