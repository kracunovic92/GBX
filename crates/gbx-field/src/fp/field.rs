use core::fmt;

use gbx_alg::DivByZero;

use crate::error::{FieldError, FieldResult};
use crate::fp::prime::is_prime_u32;

/// Runtime prime field `𝔽_p`.
///
/// `Fp` stores the prime modulus `p` and provides arithmetic for compact
/// [`FpElem`] values.
///
/// Elements do not store the modulus. This keeps coefficients small and makes
/// them cheap to copy. Arithmetic must always be performed through the `Fp`
/// context that created the element, or through another `Fp` with the same
/// modulus.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fp {
    p: u32,
}

/// Element of a runtime prime field.
///
/// This stores a reduced representative in `[0, p)`.
/// The modulus itself is not stored in the element.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct FpElem(u32);

impl FpElem {
    /// Returns the stored representative.
    #[must_use]
    #[inline]
    pub const fn repr_u32(self) -> u32 {
        self.0
    }
}

impl Fp {
    /// Constructs the prime field `𝔽_p`.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`FieldError::InvalidModulus`] if `p < 2`
    /// - [`FieldError::ModulusNotPrime`] if `p` is not prime
    #[inline]
    pub fn prime(p: u32) -> FieldResult<Self> {
        if p < 2 {
            return Err(FieldError::InvalidModulus { p });
        }

        if !is_prime_u32(p) {
            return Err(FieldError::ModulusNotPrime { p });
        }

        Ok(Self { p })
    }

    /// Returns the modulus.
    #[must_use]
    #[inline]
    pub const fn modulus(self) -> u32 {
        self.p
    }

    /// Alias for [`Fp::modulus`].
    #[must_use]
    #[inline]
    pub const fn p(self) -> u32 {
        self.p
    }

    /// Constructs a field element by reducing `x` modulo `p`.
    #[must_use]
    #[inline]
    pub fn new(self, x: u32) -> FpElem {
        FpElem(x % self.p)
    }

    /// Returns the additive identity.
    #[must_use]
    #[inline]
    pub const fn zero(self) -> FpElem {
        FpElem(0)
    }

    /// Returns the multiplicative identity.
    #[must_use]
    #[inline]
    pub const fn one(self) -> FpElem {
        FpElem(1)
    }

    /// Returns the canonical representative of `a`.
    #[must_use]
    #[inline]
    pub const fn repr_u32(self, a: FpElem) -> u32 {
        a.0
    }

    /// Backward-compatible alias for [`Fp::repr_u32`].
    #[must_use]
    #[inline]
    pub const fn value(self, a: FpElem) -> u32 {
        a.0
    }

    /// Adds two field elements.
    #[must_use]
    #[inline]
    pub fn add(self, a: FpElem, b: FpElem) -> FpElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);

        let (s, carry) = a.0.overflowing_add(b.0);
        let s = if carry || s >= self.p { s.wrapping_sub(self.p) } else { s };

        FpElem(s)
    }

    /// Subtracts two field elements.
    #[must_use]
    #[inline]
    pub fn sub(self, a: FpElem, b: FpElem) -> FpElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);

        let v = if a.0 >= b.0 { a.0 - b.0 } else { a.0 + (self.p - b.0) };

        FpElem(v)
    }

    /// Returns the additive inverse of `a`.
    #[must_use]
    #[inline]
    pub fn neg(self, a: FpElem) -> FpElem {
        debug_assert!(a.0 < self.p);

        if a.0 == 0 { a } else { FpElem(self.p - a.0) }
    }

    /// Multiplies two field elements.
    #[must_use]
    #[inline]
    pub fn mul(self, a: FpElem, b: FpElem) -> FpElem {
        debug_assert!(a.0 < self.p && b.0 < self.p);

        let prod = (a.0 as u64) * (b.0 as u64);
        FpElem((prod % (self.p as u64)) as u32)
    }

    /// Returns the multiplicative inverse of `a`.
    ///
    /// Returns `None` iff `a == 0`.
    #[must_use]
    #[inline]
    pub fn try_inv(self, a: FpElem) -> Option<FpElem> {
        debug_assert!(a.0 < self.p);

        if a.0 == 0 {
            return None;
        }

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
            return None;
        }

        let inv = x0.rem_euclid(p) as u32;
        Some(FpElem(inv))
    }

    /// Computes `a / b`.
    ///
    /// Returns [`DivByZero`] if `b == 0`.
    #[inline]
    pub fn checked_div(self, a: FpElem, b: FpElem) -> core::result::Result<FpElem, DivByZero> {
        self.try_inv(b).map(|inv| self.mul(a, inv)).ok_or(DivByZero)
    }
}

impl fmt::Debug for FpElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FpElem({})", self.0)
    }
}

impl fmt::Display for FpElem {
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
        assert_eq!(size_of::<FpElem>(), 4);
    }

    #[test]
    fn construction_reduces_mod_p() {
        let f = Fp::prime(7).unwrap();

        assert_eq!(f.repr_u32(f.new(0)), 0);
        assert_eq!(f.repr_u32(f.new(7)), 0);
        assert_eq!(f.repr_u32(f.new(8)), 1);
    }

    #[test]
    fn arithmetic_works() {
        let f = Fp::prime(7).unwrap();

        let a = f.new(5);
        let b = f.new(6);

        assert_eq!(f.repr_u32(f.add(a, b)), 4);
        assert_eq!(f.repr_u32(f.sub(a, b)), 6);
        assert_eq!(f.repr_u32(f.neg(a)), 2);
        assert_eq!(f.repr_u32(f.mul(a, b)), 2);
    }

    #[test]
    fn inverse_and_division_work() {
        let f = Fp::prime(7).unwrap();

        let a = f.new(5);
        let inv = f.try_inv(a).unwrap();

        assert_eq!(f.repr_u32(f.mul(a, inv)), 1);

        let b = f.new(3);
        let q = f.checked_div(a, b).unwrap();

        assert_eq!(f.repr_u32(f.mul(q, b)), f.repr_u32(a));
    }

    #[test]
    fn inverse_of_zero_is_none() {
        let f = Fp::prime(7).unwrap();

        assert_eq!(f.try_inv(f.zero()), None);
    }

    #[test]
    fn division_by_zero_is_error() {
        let f = Fp::prime(7).unwrap();

        let a = f.new(5);
        let z = f.zero();

        assert!(f.checked_div(a, z).is_err());
    }
}
