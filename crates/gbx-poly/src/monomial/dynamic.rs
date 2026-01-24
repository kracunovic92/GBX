use crate::monomial::{Monomial, MonomialError, MonomialView};
use gbx_storage::exponents::DynExps;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DynamicMonomial {
    exps: DynExps<u32>,
    degree: u32,
}

impl DynamicMonomial {
    #[inline]
    pub fn one(n_vars: usize) -> Self {
        Self { exps: DynExps::new(vec![0u32; n_vars]), degree: 0 }
    }

    #[inline]
    pub fn try_from_slice(exponents: &[u32]) -> Result<Self, MonomialError> {
        let mut deg = 0u32;
        for &e in exponents {
            deg = deg.checked_add(e).ok_or(MonomialError::DegreeOverflow)?;
        }
        Ok(Self { exps: DynExps::new(exponents.to_vec()), degree: deg })
    }

    #[inline]
    #[allow(clippy::expect_used)]
    pub fn from_slice(exponents: &[u32]) -> Self {
        Self::try_from_slice(exponents).expect("DynamicMonomial::from_slice degree overflow")
    }

    #[inline]
    pub const fn degree(&self) -> u32 {
        self.degree
    }
}

impl MonomialView for DynamicMonomial {
    #[inline]
    fn n_vars(&self) -> usize {
        self.exps.len()
    }

    #[inline]
    fn exponents(&self) -> &[u32] {
        self.exps.as_slice()
    }
}

impl Monomial for DynamicMonomial {
    type Error = MonomialError;

    #[inline]
    fn try_from_exponents_iter<I>(n_vars: usize, exps: I) -> Result<Self, Self::Error>
    where
        I: IntoIterator<Item = u32>,
    {
        let v: Vec<u32> = exps.into_iter().collect();
        if v.len() != n_vars {
            return Err(MonomialError::WrongLength { expected: n_vars, got: v.len() });
        }
        Self::try_from_slice(&v)
    }

    #[inline]
    fn degree_checked(&self) -> Result<u32, MonomialError> {
        Ok(self.degree)
    }

    #[inline]
    fn checked_mul(&self, other: &Self) -> Result<Self, Self::Error> {
        let n = self.n_vars();
        if n != other.n_vars() {
            return Err(MonomialError::MismatchedVariableCount { lhs: n, rhs: other.n_vars() });
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
            let s = a
                .checked_add(b)
                .ok_or(MonomialError::ExponentOverflow { index: i, lhs: a, rhs: b })?;
            out.push(s);
        }

        Ok(Self { exps: DynExps::new(out), degree: deg })
    }
}
