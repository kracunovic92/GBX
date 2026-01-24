use core::fmt;

/// Integers modulo a type-level modulus `P`.
///
/// Stored in canonical reduced form: `0 <= value < P`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Zp<const P: u32>(pub(crate) u32);

impl<const P: u32> Zp<P> {
    /// The modulus `P`.
    #[must_use]
    #[inline]
    pub const fn modulus() -> u32 {
        P
    }

    /// Construct `x (mod P)` reduced to `[0, P)`.
    ///
    /// # Debug
    /// Asserts `P >= 2`.
    #[must_use]
    #[inline]
    pub const fn new(x: u32) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        Self(x % P)
    }

    /// Non-const constructor (convenient API stability).
    #[must_use]
    #[inline]
    pub fn new_checked(x: u32) -> Self {
        Self::new(x)
    }

    /// Construct from `u64`, reduced mod `P`.
    #[must_use]
    #[inline]
    pub fn from_u64(x: u64) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        Self((x % (P as u64)) as u32)
    }

    /// Construct from already-reduced representative.
    ///
    /// # Debug
    /// Asserts `P >= 2` and `x < P`.
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(x: u32) -> Self {
        debug_assert!(P >= 2, "Zp<P>: modulus must be >= 2");
        debug_assert!(x < P, "Zp::from_reduced_unchecked: x must be < P");
        Self(x)
    }

    /// Canonical representative in `[0, P)`.
    #[must_use]
    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Exponentiation mod `P` using binary exponentiation.
    ///
    /// Defined for all exponents, including `0^0 == 1` by convention.
    #[must_use]
    #[inline]
    pub fn pow(self, mut e: u128) -> Self {
        let mut acc = <Self as gbx_alg::One>::ONE;
        if e == 0 {
            return acc;
        }

        let mut base = self;
        while e != 0 {
            if (e & 1) != 0 {
                acc *= base;
            }
            e >>= 1;
            if e != 0 {
                base *= base;
            }
        }
        acc
    }

    /// Iterate all residues `0..P` (useful for small-modulus tests).
    #[inline]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..P).map(Self::from_reduced_unchecked)
    }
}

impl<const P: u32> fmt::Debug for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Zp<{}>({})", P, self.0)
    }
}

impl<const P: u32> fmt::Display for Zp<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
