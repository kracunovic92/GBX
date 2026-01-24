use super::Exps;

/// Inline exponent vector `[E; N]`.
///
/// This is ideal for fixed-arity monomials: no heap allocation, good locality.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FixedExps<E, const N: usize>(pub [E; N]);

/// Convenience alias for the common case.
pub type FixedExps32<const N: usize> = FixedExps<u32, N>;

impl<E, const N: usize> FixedExps<E, N> {
    /// Wrap an array (helps when using type aliases like `FixedExps32`).
    #[inline]
    pub const fn new(values: [E; N]) -> Self {
        Self(values)
    }
    /// Size
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    #[inline]
    /// Get the Slice
    pub const fn as_slice(&self) -> &[E] {
        &self.0
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
        self.as_slice()
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
}
