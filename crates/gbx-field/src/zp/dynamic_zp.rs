use core::fmt;

use crate::error::{FieldError, Result};

/// Runtime ring descriptor for `ℤ/pℤ` (p >= 2).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ZpDyn {
    modulus: u32,
}

/// Element of `ℤ/pℤ` for runtime modulus.
/// 4 bytes: just the reduced representative.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ZpDynElem(u32);

impl ZpDyn {
    /// Returns runtime Modulus or FieldError
    #[inline]
    pub fn modulus(modulus: u32) -> Result<Self> {
        if modulus < 2 {
            return Err(FieldError::InvalidModulus { p: modulus });
        }
        Ok(Self { modulus })
    }

    /// Returns the modulus `p`.
    #[must_use]
    #[inline]
    pub const fn val(self) -> u32 {
        self.modulus
    }

    /// Reduces `x` modulo `p` and returns the corresponding residue.
    ///
    /// The result always satisfies `0 <= value < p`.
    #[must_use]
    #[inline]
    pub fn new(self, x: u32) -> ZpDynElem {
        ZpDynElem(x % self.modulus)
    }

    /// Constructs an element from a residue assumed to already be reduced.
    ///
    /// # Safety / Contract
    ///
    /// The caller must guarantee `x < self.modulus`.
    ///
    /// This is useful in tight loops or iterators where reduction is known
    /// to be unnecessary.
    #[must_use]
    #[inline]
    pub const fn from_reduced_unchecked(self, x: u32) -> ZpDynElem {
        // caller guarantees x < p
        ZpDynElem(x)
    }

    /// Returns the additive identity `0`.
    #[must_use]
    #[inline]
    pub const fn zero(self) -> ZpDynElem {
        ZpDynElem(0)
    }

    /// Returns the multiplicative identity `1 (mod p)`.
    ///
    /// Note: for valid moduli `p >= 2`, this is always `1`.
    #[must_use]
    #[inline]
    pub fn one(self) -> ZpDynElem {
        ZpDynElem(1 % self.modulus)
    }

    /// Returns the underlying reduced representative in `[0, p)`.
    #[must_use]
    #[inline]
    pub const fn value(self, a: ZpDynElem) -> u32 {
        a.0
    }

    /// Adds two residues modulo `p`.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that `a` and `b` are reduced (`< p`).
    ///
    /// # Complexity
    ///
    /// O(1).
    #[must_use]
    #[inline]
    pub fn add(self, a: ZpDynElem, b: ZpDynElem) -> ZpDynElem {
        debug_assert!(a.0 < self.modulus && b.0 < self.modulus);
        let (s, carry) = a.0.overflowing_add(b.0);
        let s = if carry || s >= self.modulus { s.wrapping_sub(self.modulus) } else { s };
        ZpDynElem(s)
    }

    /// Subtracts two residues modulo `p`.
    ///
    /// Computes `a - b (mod p)`.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that `a` and `b` are reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn sub(self, a: ZpDynElem, b: ZpDynElem) -> ZpDynElem {
        debug_assert!(a.0 < self.modulus && b.0 < self.modulus);
        let v = if a.0 >= b.0 { a.0 - b.0 } else { a.0 + (self.modulus - b.0) };
        ZpDynElem(v)
    }

    /// Returns the additive inverse `-a (mod p)`.
    ///
    /// `0` maps to `0`. Any nonzero `a` maps to `p - a`.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that `a` is reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn neg(self, a: ZpDynElem) -> ZpDynElem {
        debug_assert!(a.0 < self.modulus);
        if a.0 == 0 { a } else { ZpDynElem(self.modulus - a.0) }
    }

    /// Multiplies two residues modulo `p`.
    ///
    /// This uses a widened `u64` intermediate to avoid `u32` overflow.
    ///
    /// # Debug Assertions
    ///
    /// In debug builds, asserts that `a` and `b` are reduced (`< p`).
    #[must_use]
    #[inline]
    pub fn mul(self, a: ZpDynElem, b: ZpDynElem) -> ZpDynElem {
        debug_assert!(a.0 < self.modulus && b.0 < self.modulus);
        let prod = (a.0 as u64) * (b.0 as u64);
        ZpDynElem((prod % (self.modulus as u64)) as u32)
    }

    /// Computes `a^e (mod p)` using exponentiation by squaring.
    ///
    /// - `e = 0` returns `1`.
    /// - Runs in `O(log e)` multiplications.
    #[must_use]
    #[inline]
    pub fn pow(self, a: ZpDynElem, mut e: u128) -> ZpDynElem {
        let mut acc = self.one();
        if e == 0 {
            return acc;
        }
        let mut base = a;
        while e != 0 {
            if (e & 1) != 0 {
                acc = self.mul(acc, base);
            }
            e >>= 1;
            if e != 0 {
                base = self.mul(base, base);
            }
        }
        acc
    }

    /// Iterate all residues 0..p (for small p tests).
    #[inline]
    pub fn iter_all(self) -> impl Iterator<Item = ZpDynElem> {
        (0..self.modulus).map(move |x| self.from_reduced_unchecked(x))
    }
}

impl fmt::Debug for ZpDynElem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZpDynElem({})", self.0)
    }
}

impl fmt::Display for ZpDynElem {
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
        assert_eq!(size_of::<ZpDynElem>(), 4);
    }

    #[test]
    fn ops_work() {
        let r = ZpDyn::modulus(7).unwrap();
        let a = r.new(6);
        let b = r.new(6);
        assert_eq!(r.value(r.add(a, b)), 5);
        assert_eq!(r.value(r.mul(a, b)), 1);
        assert_eq!(r.value(r.neg(r.new(2))), 5);
        assert_eq!(r.value(r.pow(r.new(3), 2)), 2);
    }
}
