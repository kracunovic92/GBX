use crate::exponents::{Exps, ExpsError};

/// Dynamic exponent vector stored as `Box<[E]>`.
///
/// This is the default runtime-sized exponent storage.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DynExps<E = u32>(Box<[E]>);

impl<E> DynExps<E> {
    #[inline]
    pub fn new(v: Vec<E>) -> Self {
        Self(v.into_boxed_slice())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn as_slice(&self) -> &[E] {
        &self.0
    }
}

impl DynExps<u32> {
    /// Construct from a slice of `u64`, checking that all values fit in `u32`.
    #[inline]
    pub fn try_from_u64_slice(exps: &[u64]) -> Result<Self, ExpsError> {
        let mut out = Vec::with_capacity(exps.len());
        for &e in exps {
            let e32 = u32::try_from(e).map_err(|_| ExpsError::ValueOverflow { value: e })?;
            out.push(e32);
        }
        Ok(Self::new(out))
    }
}

impl<E: Copy + Eq> Exps for DynExps<E> {
    type Word = E;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn as_slice(&self) -> &[E] {
        self.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_exps_basics() {
        let e = DynExps::new(vec![1u32, 2, 3]);
        assert_eq!(e.len(), 3);
        assert_eq!(e.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn dyn_exps_try_from_u64_slice_checks() {
        let e = DynExps::<u32>::try_from_u64_slice(&[1, 2, 3]).unwrap();
        assert_eq!(e.as_slice(), &[1, 2, 3]);
    }
}
