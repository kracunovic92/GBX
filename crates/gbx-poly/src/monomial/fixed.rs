use core::fmt;

use gbx_storage::exponents::FixedExps32;

use crate::monomial::error::{MonomialError, Result};
use crate::monomial::{Monomial, MonomialDisplay, MonomialView};

/// A monomial in exactly `N` variables, stored as a fixed-size exponent array.
///
/// This is the "fast path" monomial representation:
/// - no heap allocation
/// - cached total degree (`u32`)
/// - operations can be heavily optimized by the compiler
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedMonomial<const N: usize> {
    exps: FixedExps32<N>,
    degree: u32,
}

impl<const N: usize> FixedMonomial<N> {
    /// Returns the multiplicative identity monomial `1` (all exponents are zero).
    #[inline]
    pub const fn one() -> Self {
        Self { exps: FixedExps32::new([0u32; N]), degree: 0 }
    }

    /// Constructs a monomial from a full exponent array, validating that the cached
    /// total degree fits in `u32`.
    ///
    /// This is the preferred constructor for library/algorithm code.
    #[inline]
    pub fn try_from_exponents(exponents: [u32; N]) -> Result<Self> {
        let mut deg = 0u32;
        for &e in exponents.iter() {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(Self { exps: FixedExps32::new(exponents), degree: deg })
    }

    /// Constructs a monomial from a full exponent array and panics if the total
    /// degree overflows `u32`.
    ///
    /// # Panics
    /// Panics if the total degree exceeds `u32::MAX`.
    ///
    /// Prefer [`try_from_exponents`] in library code.
    #[inline]
    #[allow(clippy::expect_used)]
    pub fn from_exponents(exponents: [u32; N]) -> Self {
        Self::try_from_exponents(exponents).expect("FixedMonomial::from_exponents: total degree overflow")
    }

    /// Returns a reference to the internal fixed-size exponent array.
    ///
    /// Useful when you need `[u32; N]`-shaped access (e.g. serialization).
    #[inline]
    pub const fn exponents_array(&self) -> &[u32; N] {
        &self.exps.0
    }

    /// Returns the cached total degree (sum of all exponents).
    #[inline]
    pub const fn degree(&self) -> u32 {
        self.degree
    }
}

impl<const N: usize> Default for FixedMonomial<N> {
    /// `Default` is the multiplicative identity `1`.
    #[inline]
    fn default() -> Self {
        Self::one()
    }
}

impl<const N: usize> MonomialView for FixedMonomial<N> {
    type Word = u32;

    /// Exponent slice view.
    #[inline]
    fn exponents(&self) -> &[u32] {
        &self.exps.0
    }

    /// Cached degree is always available for `FixedMonomial`.
    #[inline]
    fn degree_hint(&self) -> Option<u32> {
        Some(self.degree)
    }
}

impl<const N: usize> Monomial for FixedMonomial<N> {
    /// Constructs from an iterator, validating:
    /// - requested `n_vars` matches `N`
    /// - iterator yields exactly `N` exponents
    /// - total degree fits in `u32`
    #[inline]
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self>
    where
        I: IntoIterator<Item = Self::Word>,
    {
        if n_vars != N {
            return Err(MonomialError::MismatchedArity { lhs: N, rhs: n_vars });
        }

        let mut out = [0u32; N];
        let mut it = exps.into_iter();

        for i in 0..N {
            match it.next() {
                Some(v) => out[i] = v,
                None => return Err(MonomialError::WrongLength { expected: N, got: i }),
            }
        }

        if let Some(_) = it.next() {
            let extra_rest = it.count();
            return Err(MonomialError::WrongLength { expected: N, got: N + 1 + extra_rest });
        }

        Self::try_from_exponents(out)
    }

    /// Returns the cached degree.
    #[inline]
    fn degree_checked(&self) -> Result<u32> {
        Ok(self.degree)
    }

    /// Checked multiplication: component-wise exponent addition + checked degree update.
    #[inline]
    fn checked_mul(&self, other: &Self) -> Result<Self> {
        let mut out = [0u32; N];

        // Compute exponent sums first so we can report ExponentOverflow precisely.
        for i in 0..N {
            let a = self.exps.0[i];
            let b = other.exps.0[i];
            out[i] = a
                .checked_add(b)
                .ok_or(MonomialError::ExponentOverflow { index: i, lhs: a, rhs: b })?;
        }

        // Now compute resulting degree from the resulting exponents.
        let mut deg = 0u32;
        for &e in out.iter() {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }

        Ok(Self { exps: FixedExps32::new(out), degree: deg })
    }
}

impl<const N: usize> fmt::Debug for FixedMonomial<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FixedMonomial")
            .field("exponents", &self.exps.0)
            .field("degree", &self.degree)
            .finish()
    }
}

impl<const N: usize> fmt::Display for FixedMonomial<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", MonomialDisplay::product(self))
    }
}

impl<const N: usize> From<[u32; N]> for FixedMonomial<N> {
    #[inline]
    fn from(exps: [u32; N]) -> Self {
        FixedMonomial::from_exponents(exps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monomial::{MonomialError, MonomialView};

    #[test]
    fn degree_is_cached_and_correct() {
        let m = FixedMonomial::<3>::from_exponents([1, 2, 3]);
        assert_eq!(m.degree(), 6);
        assert_eq!(m.degree_hint(), Some(6));
    }

    #[test]
    fn try_from_exponents_detects_degree_overflow() {
        let r = FixedMonomial::<2>::try_from_exponents([u32::MAX, 1]);
        assert_eq!(r.unwrap_err(), MonomialError::DegreeOverflow);
    }

    #[test]
    fn checked_mul_detects_exponent_overflow() {
        let a = FixedMonomial::<1>::from_exponents([u32::MAX]);
        let b = FixedMonomial::<1>::from_exponents([1]);

        let err = a.checked_mul(&b).unwrap_err();
        assert!(matches!(
            err,
            MonomialError::ExponentOverflow { index: 0, .. }
        ));
    }

    #[test]
    fn display_formats_one_and_powers() {
        let one = FixedMonomial::<3>::one();
        assert_eq!(one.to_string(), "1");

        let m = FixedMonomial::<4>::from_exponents([0, 1, 2, 0]);
        assert_eq!(m.to_string(), "x1*x2^2");
    }
}
