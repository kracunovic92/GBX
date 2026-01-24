use crate::monomial::{Monomial, MonomialError, MonomialView};
use core::fmt;
use gbx_storage::prelude::{FixedExps, FixedExps32};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedMonomial<const N: usize> {
    exps: FixedExps32<N>,
    degree: u32,
}

impl<const N: usize> FixedMonomial<N> {
    #[inline]
    pub const fn one() -> Self {
        Self { exps: FixedExps::<u32, N>([0u32; N]), degree: 0 }
    }

    #[inline]
    pub fn try_from_exponents(exponents: [u32; N]) -> Result<Self, MonomialError> {
        let mut deg = 0u32;
        for &e in exponents.iter() {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(Self { exps: FixedExps::<u32, N>(exponents), degree: deg })
    }

    #[inline]
    #[allow(clippy::expect_used)]
    pub fn from_exponents(exponents: [u32; N]) -> Self {
        Self::try_from_exponents(exponents).expect("FixedMonomial::from_exponents degree overflow")
    }

    #[inline]
    pub const fn exponents_array(&self) -> &[u32; N] {
        &self.exps.0
    }

    #[inline]
    pub const fn degree(&self) -> u32 {
        self.degree
    }
}

impl<const N: usize> MonomialView for FixedMonomial<N> {
    #[inline]
    fn n_vars(&self) -> usize {
        N
    }

    #[inline]
    fn exponents(&self) -> &[u32] {
        self.exps.as_slice()
    }
}

impl<const N: usize> Monomial for FixedMonomial<N> {
    type Error = MonomialError;

    #[inline]
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = u32>,
    {
        if n_vars != N {
            return Err(MonomialError::MismatchedVariableCount { lhs: N, rhs: n_vars });
        }

        let mut out = [0u32; N];
        let mut it = exps.into_iter();

        for i in 0..N {
            match it.next() {
                Some(v) => out[i] = v,
                None => return Err(MonomialError::WrongLength { expected: N, got: i }),
            }
        }

        let extra = it.count();
        if extra != 0 {
            return Err(MonomialError::WrongLength { expected: N, got: N + extra });
        }

        Self::try_from_exponents(out)
    }

    #[inline]
    fn degree_checked(&self) -> Result<u32, Self::Error> {
        Ok(self.degree)
    }

    #[inline]
    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error> {
        let deg = self
            .degree
            .checked_add(other.degree)
            .ok_or(MonomialError::DegreeOverflow)?;

        let mut out = [0u32; N];
        for i in 0..N {
            let a = self.exps.0[i];
            let b = other.exps.0[i];
            out[i] = a
                .checked_add(b)
                .ok_or(MonomialError::ExponentOverflow { index: i, lhs: a, rhs: b })?;
        }

        Ok(Self { exps: FixedExps::<u32, N>(out), degree: deg })
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
        let mut first = true;
        for (i, &e) in self.exps.0.iter().enumerate() {
            if e == 0 {
                continue;
            }
            if !first {
                write!(f, "*")?;
            }
            first = false;

            if e == 1 {
                write!(f, "x{i}")?;
            } else {
                write!(f, "x{i}^{e}")?;
            }
        }
        if first { write!(f, "1") } else { Ok(()) }
    }
}
