use gbx_alg::{DivByZero, TryInverse};
use gbx_field::fp::{Fp, FpDyn, FpDynElem};

/// Field arithmetic provided by a *context* (not by the element).
///
/// For dynamic fields, the element is usually a compact integer (e.g. `FpDynElem(u32)`),
/// and the context supplies arithmetic.
///
/// For static fields, `Elem` can be `Fp<P>` and the context is a zero-sized adapter.
pub trait FieldCtx {
    /// Element type stored in polynomials.
    type Elem: Copy + Eq;

    /// Optional convenience constructor for contexts that can build elements from `u32`.
    ///
    /// - For `FpDyn`, this reduces mod `p`.
    /// - For `StaticFpCtx<P>`, this is `Fp::<P>::new(x)`.
    ///
    /// If you *don’t* want this in the trait, remove it and adjust `poly!` accordingly
    /// (but your macro currently assumes `ctx.field.new(u32)` exists).
    fn new(&self, x: u32) -> Self::Elem;

    /// Additive identity.
    fn zero(&self) -> Self::Elem;
    /// Multiplicative identity.
    fn one(&self) -> Self::Elem;
    /// Addition.
    fn add(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;
    /// Subtraction.
    fn sub(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;
    /// Additive inverse.
    fn neg(&self, a: Self::Elem) -> Self::Elem;
    /// Multiplication.
    fn mul(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem;

    /// Canonical representative, typically in `[0..p-1]` for Fp.
    fn repr_u32(&self, a: Self::Elem) -> u32;

    /// Returns the multiplicative inverse if `a != 0`.
    ///
    /// Must return `None` iff `a` is zero.
    fn try_inv(&self, a: Self::Elem) -> Option<Self::Elem>;

    /// Modulus/characteristic (used to show signed reps like `-1` instead of `p-1`).
    fn modulus_u32(&self) -> Option<u32> {
        None
    }

    /// Returns `true` if `a` is the additive identity.
    #[inline]
    fn is_zero(&self, a: Self::Elem) -> bool {
        a == self.zero()
    }

    /// Computes `a / b`, returning an error if `b == 0`.
    ///
    /// This uses [`try_inv`] and [`mul`]. Implementations may override for efficiency.
    #[inline]
    fn checked_div(&self, a: Self::Elem, b: Self::Elem) -> Result<Self::Elem, DivByZero> {
        self.try_inv(b).map(|inv| self.mul(a, inv)).ok_or(DivByZero)
    }
}

impl FieldCtx for FpDyn {
    type Elem = FpDynElem;

    #[inline]
    fn new(&self, x: u32) -> Self::Elem {
        (*self).new(x)
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

/// Zero-sized context adapter for static prime fields `Fp<P>`.
///
/// This allows algorithms to uniformly call `ctx.field.add(...)` etc.,
/// even when the underlying field is “element-style” (operations on `Fp<P>`).
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticFpCtx<const P: u32>;

impl<const P: u32> StaticFpCtx<P> {
    /// Creates a new static field context.
    ///
    /// This is equivalent to [`Default::default`] and incurs no runtime cost.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inherent constructor for coefficients used by macros (`ctx.field.new(u32)`).
    #[inline]
    pub fn elem(&self, x: u32) -> Fp<P> {
        Fp::<P>::new(x)
    }

    /// ALSO provide `new(&self, u32)` as inherent, to match your macro exactly.
    #[inline]
    pub fn new_elem(&self, x: u32) -> Fp<P> {
        Fp::<P>::new(x)
    }

    /// Alias for constructing a coefficient.
    ///
    /// Provided for readability in contexts where the semantic role
    /// is “coefficient construction”.
    #[inline]
    pub fn coeff(&self, x: u32) -> Fp<P> {
        Fp::<P>::new(x)
    }
}

impl<const P: u32> FieldCtx for StaticFpCtx<P> {
    type Elem = Fp<P>;

    #[inline]
    fn new(&self, x: u32) -> Self::Elem {
        Fp::<P>::new(x)
    }

    #[inline]
    fn zero(&self) -> Self::Elem {
        <Fp<P> as gbx_alg::Zero>::ZERO
    }

    #[inline]
    fn one(&self) -> Self::Elem {
        <Fp<P> as gbx_alg::One>::ONE
    }

    #[inline]
    fn add(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        a + b
    }

    #[inline]
    fn sub(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        a - b
    }

    #[inline]
    fn neg(&self, a: Self::Elem) -> Self::Elem {
        -a
    }

    #[inline]
    fn mul(&self, a: Self::Elem, b: Self::Elem) -> Self::Elem {
        a * b
    }

    #[inline]
    fn repr_u32(&self, a: Self::Elem) -> u32 {
        a.value()
    }

    #[inline]
    fn try_inv(&self, a: Self::Elem) -> Option<Self::Elem> {
        a.try_inv()
    }

    #[inline]
    fn modulus_u32(&self) -> Option<u32> {
        Some(P)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use gbx_field::fp::FpDyn;

    #[test]
    fn fp_dyn_ctx_arithmetic_works() {
        let f = FpDyn::prime(7).unwrap();
        let a = f.new(5);
        let b = f.new(6);

        // 5 + 6 = 11 = 4 mod 7
        let s = f.add(a, b);
        assert_eq!(f.value(s), 4);

        // 5 * 6 = 30 = 2 mod 7
        let m = f.mul(a, b);
        assert_eq!(f.value(m), 2);

        // inv(5) mod 7 is 3 because 5*3=15=1 mod 7
        let inv = f.try_inv(a).unwrap();
        assert_eq!(f.value(f.mul(a, inv)), 1);
    }

    #[test]
    fn static_fp_ctx_arithmetic_works() {
        let ctx = StaticFpCtx::<7>::default();
        let a = ctx.new(5);
        let b = ctx.new(6);

        let s = ctx.add(a, b);
        assert_eq!(s.value(), 4);

        let m = ctx.mul(a, b);
        assert_eq!(m.value(), 2);

        let inv = ctx.try_inv(a).unwrap();
        assert_eq!(ctx.mul(a, inv).value(), 1);
    }
}
