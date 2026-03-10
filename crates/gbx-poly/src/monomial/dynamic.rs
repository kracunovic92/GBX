use core::fmt;

use gbx_storage::exponents::DynExps;

use crate::monomial::error::{MonomialError, Result};
use crate::monomial::{Monomial, MonomialDisplay, MonomialView};

#[allow(missing_docs)]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DynamicMonomial {
    exps: DynExps<u32>,
    degree: u32,
}

#[allow(missing_docs)]
impl DynamicMonomial {
    #[inline]
    pub fn one(n_vars: usize) -> Self {
        Self { exps: DynExps::new(vec![0u32; n_vars]), degree: 0 }
    }

    #[inline]
    pub fn try_from_slice(exponents: &[u32]) -> Result<Self> {
        let mut deg = 0u32;
        for &e in exponents {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(Self { exps: DynExps::new(exponents.to_vec()), degree: deg })
    }

    #[inline]
    #[allow(clippy::expect_used)]
    pub fn from_slice(exponents: &[u32]) -> Self {
        Self::try_from_slice(exponents).expect("DynamicMonomial::from_slice: total degree overflow")
    }

    #[inline]
    pub const fn degree(&self) -> u32 {
        self.degree
    }

    /// Create from an owned exponent vector without copying.
    #[inline]
    pub fn try_from_vec(exponents: Vec<u32>) -> Result<Self> {
        let mut deg = 0u32;
        for &e in &exponents {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(Self { exps: DynExps::new(exponents), degree: deg })
    }

    /// Exponent slice (same as `MonomialView::exponents()`).
    #[inline]
    pub fn as_slice(&self) -> &[u32] {
        self.exps.as_slice()
    }

    /// Panicking owned constructor (no copy).
    #[inline]
    #[allow(clippy::expect_used)]
    pub fn from_vec(exponents: Vec<u32>) -> Self {
        Self::try_from_vec(exponents).expect("DynamicMonomial::from_vec: total degree overflow")
    }
}

impl Default for DynamicMonomial {
    #[inline]
    fn default() -> Self {
        Self::one(0)
    }
}

impl MonomialView for DynamicMonomial {
    type Word = u32;

    #[inline]
    fn exponents(&self) -> &[u32] {
        self.exps.as_slice()
    }

    #[inline]
    fn degree_hint(&self) -> Option<u32> {
        Some(self.degree)
    }
}

impl Monomial for DynamicMonomial {
    #[inline]
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self>
    where
        I: IntoIterator<Item = Self::Word>,
    {
        let mut it = exps.into_iter();

        // Optional early check if iterator has an exact known length.
        let (lower, upper) = it.size_hint();
        if upper == Some(lower) && lower != n_vars {
            return Err(MonomialError::WrongLength { expected: n_vars, got: lower });
        }

        let mut v = Vec::with_capacity(n_vars);
        let mut deg = 0u32;

        for i in 0..n_vars {
            match it.next() {
                Some(e) => {
                    deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
                    v.push(e);
                }
                None => return Err(MonomialError::WrongLength { expected: n_vars, got: i }),
            }
        }

        if let Some(_) = it.next() {
            let extra_rest = it.count();
            return Err(MonomialError::WrongLength { expected: n_vars, got: n_vars + 1 + extra_rest });
        }

        Ok(Self { exps: DynExps::new(v), degree: deg })
    }

    #[inline]
    fn degree_checked(&self) -> Result<u32> {
        Ok(self.degree)
    }

    #[inline]
    fn checked_mul(&self, other: &Self) -> Result<Self> {
        let n = self.n_vars();
        if n != other.n_vars() {
            return Err(MonomialError::MismatchedArity { lhs: n, rhs: other.n_vars() });
        }

        let deg = self
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

        Ok(Self { exps: DynExps::new(out), degree: deg })
    }
}

impl fmt::Display for DynamicMonomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MonomialDisplay::product(self))
    }
}
impl<const N: usize> From<[u32; N]> for DynamicMonomial {
    #[inline]
    fn from(exps: [u32; N]) -> Self {
        DynamicMonomial::from_slice(&exps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{Monomial, MonomialError, MonomialView};

    #[test]
    fn degree_is_cached_and_correct() {
        let m = DynamicMonomial::from_slice(&[1, 2, 3]);
        assert_eq!(m.degree_hint(), Some(6));
        assert_eq!(m.degree_checked().unwrap(), 6);
    }

    #[test]
    fn try_from_exponents_iter_wrong_length_too_short() {
        let err = DynamicMonomial::try_from_exponents_iter(3, [1u32, 2]).unwrap_err();
        assert_eq!(err, MonomialError::WrongLength { expected: 3, got: 2 });
    }

    #[test]
    fn try_from_exponents_iter_wrong_length_too_long() {
        let err = DynamicMonomial::try_from_exponents_iter(2, [1u32, 2, 3]).unwrap_err();
        assert_eq!(err, MonomialError::WrongLength { expected: 2, got: 3 });
    }

    #[test]
    fn try_from_slice_detects_degree_overflow() {
        let err = DynamicMonomial::try_from_slice(&[u32::MAX, 1]).unwrap_err();
        assert_eq!(err, MonomialError::DegreeOverflow);
    }

    #[test]
    fn checked_mul_detects_mismatched_arity() {
        let a = DynamicMonomial::from_slice(&[1, 0]);
        let b = DynamicMonomial::from_slice(&[1, 0, 0]);

        let err = a.checked_mul(&b).unwrap_err();
        assert_eq!(err, MonomialError::MismatchedArity { lhs: 2, rhs: 3 });
    }

    #[test]
    fn display_formats_one_and_powers() {
        let one = DynamicMonomial::one(3);
        assert_eq!(one.to_string(), "1");

        let m = DynamicMonomial::from_slice(&[0, 1, 2, 0]);
        assert_eq!(m.to_string(), "x1*x2^2");
    }
}
