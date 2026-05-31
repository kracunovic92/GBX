use crate::exponents::{Exps, ExpsError, ExpsMut};

use std::boxed::Box;
use std::vec::Vec;

/// Dynamic exponent vector stored as `Box<[E]>`.
///
/// This is the default runtime-sized exponent storage:
/// - heap allocated
/// - compact (no spare capacity, unlike `Vec`)
/// - cheap to clone as a slice container (but cloning copies the data)
///
/// This is a **storage** type. All arithmetic (mul, lcm, divides, etc.) belongs
/// in higher layers (`gbx-poly`).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct DynExps<E = u32>(Box<[E]>);

impl<E> DynExps<E> {
    /// Construct from a `Vec` (boxed to remove spare capacity).
    #[inline]
    #[must_use]
    pub fn new(v: Vec<E>) -> Self {
        Self(v.into_boxed_slice())
    }

    /// Construct directly from a boxed slice.
    #[inline]
    #[must_use]
    pub const fn new_boxed(b: Box<[E]>) -> Self {
        Self(b)
    }

    /// Number of variables (length of the exponent vector).
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the exponent vector has no entries.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the exponent vector as a slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[E] {
        &self.0
    }

    /// Mutably borrow the exponent vector as a slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [E] {
        &mut self.0
    }

    /// Consume self and return the boxed slice.
    #[inline]
    #[must_use]
    pub fn into_boxed_slice(self) -> Box<[E]> {
        self.0
    }
}

impl DynExps<u32> {
    /// Construct from a slice of `u64`, checking that all values fit in `u32`.
    ///
    /// # Errors
    /// Returns [`ExpsError::ValueOverflow`] if an input exponent does not fit in `u32`.
    #[inline]
    pub fn try_from_u64_slice(exps: &[u64]) -> Result<Self, ExpsError> {
        let mut out = Vec::with_capacity(exps.len());
        for &e in exps {
            let e32 = u32::try_from(e).map_err(|_| ExpsError::value_overflow(e))?;
            out.push(e32);
        }
        Ok(Self::new(out))
    }

    /// Construct from a slice of `i64`, interpreting negatives as an error.
    ///
    /// This is intentionally strict: exponent words are unsigned.
    ///
    /// # Errors
    /// Returns [`ExpsError::NegativeValue`] for negative inputs, or
    /// [`ExpsError::ValueOverflow`] if a nonnegative input does not fit in `u32`.
    #[inline]
    pub fn try_from_i64_slice(exps: &[i64]) -> Result<Self, ExpsError> {
        let mut out = Vec::with_capacity(exps.len());
        for &e in exps {
            if e < 0 {
                return Err(ExpsError::negative_value(e));
            }
            let unsigned = u64::try_from(e).map_err(|_| ExpsError::negative_value(e))?;
            let e32 = u32::try_from(unsigned).map_err(|_| ExpsError::value_overflow(unsigned))?;
            out.push(e32);
        }
        Ok(Self::new(out))
    }
}

impl<E: Copy + Eq> Exps for DynExps<E> {
    type Word = E;

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn as_slice(&self) -> &[E] {
        &self.0
    }
}

impl<E: Copy + Eq> ExpsMut for DynExps<E> {
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [E] {
        &mut self.0
    }
}

impl<E> From<Vec<E>> for DynExps<E> {
    #[inline]
    fn from(v: Vec<E>) -> Self {
        Self::new(v)
    }
}

impl<E> From<Box<[E]>> for DynExps<E> {
    #[inline]
    fn from(b: Box<[E]>) -> Self {
        Self::new_boxed(b)
    }
}

impl<E: Copy> From<&[E]> for DynExps<E> {
    #[inline]
    fn from(s: &[E]) -> Self {
        Self::new(s.to_vec())
    }
}

impl<E> AsRef<[E]> for DynExps<E> {
    #[inline]
    fn as_ref(&self) -> &[E] {
        &self.0
    }
}

impl<E> AsMut<[E]> for DynExps<E> {
    #[inline]
    fn as_mut(&mut self) -> &mut [E] {
        &mut self.0
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
    fn dyn_exps_mut_slice() {
        let mut e = DynExps::new(vec![1u32, 2, 3]);
        e.as_mut_slice()[0] = 9;
        assert_eq!(e.as_slice(), &[9, 2, 3]);
    }

    #[test]
    fn dyn_exps_try_from_u64_slice_checks() {
        let e = match DynExps::<u32>::try_from_u64_slice(&[1, 2, 3]) {
            Ok(exps) => exps,
            Err(err) => panic!("small exponents should fit in u32: {err}"),
        };
        assert_eq!(e.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn dyn_exps_from_slice_copies() {
        let src = [4u32, 5, 6];
        let e: DynExps<u32> = (&src[..]).into();
        assert_eq!(e.as_slice(), &[4, 5, 6]);
    }
}
