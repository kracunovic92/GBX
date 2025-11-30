//! Semiring: two associative ops (`+`, `*`) with identities `0` and `1`.
//!
//! In a semiring:
//! - `+` forms a **commutative monoid**: associative, commutative, with `0`.
//! - `*` forms a (not necessarily commutative) **monoid** with `1`.
//! - `*` **distributes** over `+`:
//!   - `a * (b + c) == a * b + a * c`
//!   - `(a + b) * c == a * c + b * c`
//! - `0` is **multiplicatively absorbing**: `0 * a == 0 == a * 0`.
//!
//! These laws are documented but **not** enforced by the type system. They
//! should be verified by law tests.

use crate::{AddMonoid, MulMonoid};

/// Basic semiring marker.
///
/// Implementing this trait signals that a type is intended to behave as a
/// semiring with respect to its additive and multiplicative operations.
pub trait Semiring: AddMonoid + MulMonoid {}

/// Blanket impl: any type that is both an additive monoid and a multiplicative
/// monoid is considered a semiring by convention.
///
/// This does *not* guarantee the distributive laws or annihilation laws; those
/// must be checked in tests.
impl<T> Semiring for T where T: AddMonoid + MulMonoid {}

#[cfg(test)]
mod tests {
    use crate::{Additive, Multiplicative, Semiring, Zero};

    #[test]
    fn semiring_spotcheck_u32() {
        let (a, b, c) = (2u32, 3, 5);

        assert_eq!(
            a.mul(b.add(c)),
            a.mul(b)
                .add(a.mul(c))
        );

        let z = u32::zero();
        assert_eq!(z.mul(a), z);
        assert_eq!(a.mul(z), z);

        fn _is_semiring<T: Semiring>(_x: T) {}
        _is_semiring(a);
    }
}
