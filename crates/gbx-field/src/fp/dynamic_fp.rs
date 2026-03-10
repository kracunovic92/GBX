use core::fmt;

use gbx_alg::DivByZero;

use crate::error::{FieldError, Result};
use crate::fp::prime::is_prime_u32;

/// Runtime descriptor for a prime field `𝔽_p` (prime modulus known at runtime).
///
/// `FpDyn` stores the prime modulus `p` and provides arithmetic for compact
/// field elements [`FpDynElem`]. Elements themselves do **not** store `p`,
/// which keeps them small (4 bytes) and cheap to copy.
///
/// # Design
///
/// - `FpDyn` is the *context* (carries `p`).
/// - [`FpDynElem`] is the *element* (a reduced representative in `[0, p)`).
///
/// This context-driven design is used throughout `gbx_*` so that algorithms can
/// operate uniformly over both static fields (`Fp<P>`) and runtime fields.
///
/// # Invariants
///
/// - `p` is prime and `p >= 2`.
/// - Any `FpDynElem` used with this context must satisfy `elem < p`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FpDyn {
    p: u32,
}

/// Element of `𝔽_p` for a runtime modulus.
///
/// This is a compact, transparent wrapper around a reduced `u32` residue.
/// The value is always intended to be in `[0, p)`, but note that `p` is not
/// stored in the element. Correctness depends on using an element with the
/// `FpDyn` context that created it (or another with the same `p`).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FpDynElem(u32);

impl FpDynElem {
    /// Returns the stored representative as `u32`.
    ///
    /// This is the canonical representative in `[0, p)` as produced by
    /// [`FpDyn::new`].
    pub fn repr_u32(self) -> u32 {
        self.0
    }
}

impl FpDyn {
    /// Constructs a prime-field context `𝔽_p` for the given modulus.
    ///
    /// # Errors
    ///
    /// - [`FieldError::InvalidModulus`] if `p < 2`
    /// - [`FieldError::ModulusNotPrime`] if `p` is not prime
    #[inline]
    pub fn prime(p: u32) -> Result<Self> {
        if p < 2 {
            return Err(FieldError::InvalidModulus { p });
        }
        if !is_prime_u32(p) {
            return Err(FieldError::ModulusNotPrime { p });
        }
        Ok(Self { p })
    }

    /// Returns the modulus `p`.
    #[must_use]
    #[inline]
    pub const fn p(self) -> u32 {
        self.p
    }

    /// Reduces `x` modulo `p` and returns the corresponding field element.
    ///
    /// The result is always in `[0, p)`.
    #[must_use]
    #[inline]
    pub fn new(self, x: u32) -> FpDynElem {
        FpDynElem(x % self.p)
    }

    /// Returns the additive identity `0`.
    #[must_use]
    #[inline]
    pub const fn zero(self) -> FpDynElem {
        FpDynElem(0)
    }

    /// Returns the multiplicative identity `1`.
    ///
    /// For valid `p >= 2`, this is always `1`.
    #[must_use]
    #[inline]
    pub fn one(self) -> FpDynElem {
        FpDynElem(1 % self.p)
    }

    /// Returns the underlying reduced representative in `[0, p)`.
    #[must_use]
    #[inline]
    pub const fn value(self, a: FpDynElem) -> u32 {
        a.0
    }

    /// Returns the modulus `p` (by shared reference).
    ///
    /// This is sometimes convenient in contexts that already hold `&FpDyn`.
    #[must_use]
    #[inline]
    pub fn modulus(&self) -> u32 {
        self.p
    }

    /// Adds two field elements.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that both inputs are reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn add(self, a: FpDynElem, b: FpDynElem) -> FpDynElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);
        let (s, carry) = a.0.overflowing_add(b.0);
        let s = if carry || s >= self.p { s.wrapping_sub(self.p) } else { s };
        FpDynElem(s)
    }

    /// Subtracts two field elements, computing `a - b`.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that both inputs are reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn sub(self, a: FpDynElem, b: FpDynElem) -> FpDynElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);
        let v = if a.0 >= b.0 { a.0 - b.0 } else { a.0 + (self.p - b.0) };
        FpDynElem(v)
    }

    /// Returns the additive inverse `-a`.
    ///
    /// `0` maps to `0`. Any nonzero `a` maps to `p - a`.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that the input is reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn neg(self, a: FpDynElem) -> FpDynElem {
        debug_assert!(a.0 < self.p);
        if a.0 == 0 { a } else { FpDynElem(self.p - a.0) }
    }

    /// Multiplies two field elements.
    ///
    /// Uses a widened `u64` intermediate to avoid `u32` overflow.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that both inputs are reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn mul(self, a: FpDynElem, b: FpDynElem) -> FpDynElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);
        let prod = (a.0 as u64) * (b.0 as u64);
        FpDynElem((prod % (self.p as u64)) as u32)
    }

    /// Returns the multiplicative inverse of `a`, if it exists.
    ///
    /// In a prime field, every nonzero element is invertible.
    ///
    /// - Returns `None` iff `a == 0`.
    /// - Uses the extended Euclidean algorithm on `(a, p)`.
    ///
    /// # Notes
    ///
    /// This implementation is defensive: if the gcd is not `1`, it returns `None`.
    #[must_use]
    #[inline]
    pub fn try_inv(self, a: FpDynElem) -> Option<FpDynElem> {
        if a.0 == 0 {
            return None;
        }

        // Extended Euclid on (a, p)
        let p = self.p as i64;
        let mut aa = a.0 as i64;
        let mut b = p;
        let mut x0: i64 = 1;
        let mut x1: i64 = 0;

        while b != 0 {
            let q = aa / b;
            (aa, b) = (b, aa - q * b);
            (x0, x1) = (x1, x0 - q * x1);
        }

        if aa != 1 {
            return None; // defensive
        }

        let inv = x0.rem_euclid(p) as u32;
        Some(FpDynElem(inv))
    }

    /// Computes `a / b`, returning [`DivByZero`] if `b == 0`.
    ///
    /// This is implemented as `a * inv(b)` using [`try_inv`].
    #[inline]
    pub fn checked_div(self, a: FpDynElem, b: FpDynElem) -> core::result::Result<FpDynElem, DivByZero> {
        self.try_inv(b).map(|inv| self.mul(a, inv)).ok_or(DivByZero)
    }
}

/* ===== formatting ===== */

impl fmt::Debug for FpDynElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FpDynElem({})", self.0)
    }
}

impl fmt::Display for FpDynElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn element_is_4_bytes() {
        assert_eq!(size_of::<FpDynElem>(), 4);
    }

    #[test]
    fn inverse_and_div_work() {
        let f = FpDyn::prime(7).unwrap();
        let a = f.new(5);
        let inv = f.try_inv(a).unwrap();
        assert_eq!(f.value(f.mul(a, inv)), 1);

        let b = f.new(3);
        let q = f.checked_div(a, b).unwrap();
        assert_eq!(f.value(f.mul(q, b)), f.value(a));
    }
}
