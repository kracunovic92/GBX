use super::{Exps, ExpsMut};

/// Inline exponent vector `[E; N]`.
///
/// Ideal for fixed-arity monomials:
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FixedExps<E, const N: usize>(pub [E; N]);

/// Convenience alias for the common case (`u32` exponents).
pub type FixedExps32<const N: usize> = FixedExps<u32, N>;

impl<E, const N: usize> FixedExps<E, N> {
    /// Wrap an array (useful when working with type aliases).
    #[inline]
    pub const fn new(values: [E; N]) -> Self {
        Self(values)
    }

    /// Length (number of variables), known at compile time.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns `true` when the fixed exponent vector has zero arity.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Borrow as a slice.
    #[inline]
    #[must_use]
    pub const fn as_slice(&self) -> &[E] {
        &self.0
    }

    /// Mutably borrow as a slice.
    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [E] {
        &mut self.0
    }
}

impl<E: Copy + Eq, const N: usize> Exps for FixedExps<E, N> {
    type Word = E;

    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn as_slice(&self) -> &[E] {
        &self.0
    }
}

impl<E: Copy + Eq, const N: usize> ExpsMut for FixedExps<E, N> {
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [E] {
        &mut self.0
    }
}

impl<E, const N: usize> From<[E; N]> for FixedExps<E, N> {
    #[inline]
    fn from(values: [E; N]) -> Self {
        Self(values)
    }
}

impl<E, const N: usize> AsRef<[E]> for FixedExps<E, N> {
    #[inline]
    fn as_ref(&self) -> &[E] {
        &self.0
    }
}

impl<E, const N: usize> AsMut<[E]> for FixedExps<E, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [E] {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_exps_basics() {
        let e = FixedExps::<u32, 3>::new([1, 2, 3]);
        assert_eq!(e.len(), 3);
        assert_eq!(e.as_slice(), &[1, 2, 3]);

        let e2 = FixedExps32::<3>::new([4, 5, 6]);
        assert_eq!(e2.as_slice(), &[4, 5, 6]);
    }

    #[test]
    fn fixed_exps_mut_slice() {
        let mut e = FixedExps::<u32, 3>::new([1, 2, 3]);
        e.as_mut_slice()[1] = 9;
        assert_eq!(e.as_slice(), &[1, 9, 3]);
    }

    #[test]
    fn fixed_exps_from_array() {
        let e: FixedExps<u32, 2> = [7, 8].into();
        assert_eq!(e.as_slice(), &[7, 8]);
    }
}
