use crate::zp::Zp;

#[cfg(feature = "prime-check")]
use super::validate::validate_prime_once;

/// Element of the prime field `𝔽_p` with modulus `P`.
///
/// Internally stored as a reduced `Zp<P>`, but the type conveys the stronger API.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Fp<const P: u32>(pub(crate) Zp<P>);

impl<const P: u32> Fp<P> {
    /// Returns the modulus `P`.
    ///
    /// This is a type-level constant carried by the `Fp<P>` type.
    #[must_use]
    #[inline]
    pub const fn modulus() -> u32 {
        P
    }

    /// Constructs `x (mod P)`.
    ///
    /// If the `prime-check` feature is enabled, this validates once (per concrete `P`)
    /// that `P` is prime and at least 2.
    #[must_use]
    #[inline]
    pub fn new(x: u32) -> Self {
        #[cfg(feature = "prime-check")]
        validate_prime_once::<P>();
        Self(Zp::new_checked(x))
    }

    /// Constructs from a `u64`, reducing modulo `P`.
    ///
    /// Useful for parsing or when intermediate computations exceed `u32`.
    ///
    /// If the `prime-check` feature is enabled, this validates once (per concrete `P`)
    /// that `P` is prime and at least 2.
    #[must_use]
    #[inline]
    pub fn from_u64(x: u64) -> Self {
        #[cfg(feature = "prime-check")]
        validate_prime_once::<P>();
        Self(Zp::from_u64(x))
    }

    /// Constructs from an already-reduced representative.
    ///
    /// # Safety / correctness
    /// The caller must ensure `x < P`. This is intended for trusted fast paths
    /// (e.g. iterators/tests).
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(x: u32) -> Self {
        Self(Zp::from_reduced_unchecked(x))
    }
    /// Returns the canonical representative in `[0, P)`.
    #[must_use]
    #[inline]
    pub const fn value(self) -> u32 {
        self.0.value()
    }

    /// Iterates all field elements `0..P`.
    ///
    /// Intended for tests and small primes.
    #[inline]
    pub fn iter_all() -> impl Iterator<Item = Self> {
        (0..P).map(Self::from_reduced_unchecked)
    }
}
