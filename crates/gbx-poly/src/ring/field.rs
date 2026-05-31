use gbx_alg::DivByZero;
use gbx_field::fp::{Fp, FpElem};

/// Field arithmetic used by polynomial and Gröbner basis algorithms.
///
/// Coefficients stored inside polynomials are intentionally small values.
/// The field context owns the arithmetic rules, such as the prime modulus.
///
/// For example, in `Fp`, a coefficient is an `FpElem`, while the
/// surrounding `Fp` context knows the modulus and performs addition,
/// multiplication, inversion, and normalization.
pub trait FieldCtx {
    /// Element type stored as a polynomial coefficient.
    type Elem: Copy + Eq;

    /// Constructs a field element from a raw `u32`.
    ///
    /// For prime fields, this should reduce `x` modulo the field modulus.
    fn elem(&self, x: u32) -> Self::Elem;

    /// Additive identity.
    fn zero(&self) -> Self::Elem;

    /// Multiplicative identity.
    fn one(&self) -> Self::Elem;

    /// Field addition.
    fn add(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Field subtraction.
    fn sub(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Additive inverse.
    fn neg(&self, a: Self::Elem) -> Self::Elem;

    /// Field multiplication.
    fn mul(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Canonical unsigned representative of a field element.
    ///
    /// For `GF(p)`, this is usually a value in `0..p`.
    fn repr_u32(&self, a: Self::Elem) -> u32;

    /// Multiplicative inverse.
    ///
    /// Returns `None` exactly when `a` is zero.
    fn try_inv(&self, a: Self::Elem) -> Option<Self::Elem>;

    /// Prime modulus, when known.
    ///
    /// This is useful for pretty-printing coefficients, for example printing
    /// `p - 1` as `-1`.
    fn modulus_u32(&self) -> Option<u32> {
        None
    }

    /// Returns true when `a` is the additive identity.
    #[inline]
    fn is_zero(&self, a: Self::Elem) -> bool {
        a == self.zero()
    }

    /// Computes `a / b`.
    ///
    /// # Errors
    ///
    /// Returns [`DivByZero`] if `b` has no inverse.
    #[inline]
    fn checked_div(&self, a: Self::Elem, b: Self::Elem) -> Result<Self::Elem, DivByZero> {
        self.try_inv(b).map(|inv| self.mul(a, inv)).ok_or(DivByZero)
    }
}

impl FieldCtx for Fp {
    type Elem = FpElem;

    #[inline]
    fn elem(&self, x: u32) -> Self::Elem {
        (*self).elem(x)
    }

    #[inline]
    fn zero(&self) -> Self::Elem {
        (*self).zero()
    }

    #[inline]
    fn one(&self) -> Self::Elem {
        (*self).one()
    }

    #[inline]
    fn add(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        (*self).add(a, b)
    }

    #[inline]
    fn sub(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        (*self).sub(a, b)
    }

    #[inline]
    fn neg(&self, a: Self::Elem) -> Self::Elem {
        (*self).neg(a)
    }

    #[inline]
    fn mul(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        (*self).mul(a, b)
    }

    #[inline]
    fn repr_u32(&self, a: Self::Elem) -> u32 {
        a.repr_u32()
    }

    #[inline]
    fn try_inv(&self, a: Self::Elem) -> Option<Self::Elem> {
        (*self).try_inv(a)
    }

    #[inline]
    fn modulus_u32(&self) -> Option<u32> {
        Some(self.modulus())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field_7() -> Fp {
        match Fp::prime(7) {
            Ok(field) => field,
            Err(err) => panic!("7 should be prime: {err}"),
        }
    }

    #[test]
    fn fp_dyn_ctx_arithmetic_works() {
        let field = field_7();

        let lhs = field.elem(5);
        let rhs = field.elem(6);

        let sum = field.add(lhs, rhs);
        assert_eq!(field.repr_u32(sum), 4);

        let product = field.mul(lhs, rhs);
        assert_eq!(field.repr_u32(product), 2);

        let Some(inverse) = field.try_inv(lhs) else {
            panic!("nonzero element should be invertible");
        };
        assert_eq!(field.repr_u32(field.mul(lhs, inverse)), 1);
    }

    #[test]
    fn checked_div_errors_on_zero() {
        let field = field_7();

        let value = field.elem(5);
        let zero = field.zero();

        assert!(field.checked_div(value, zero).is_err());
    }
}
