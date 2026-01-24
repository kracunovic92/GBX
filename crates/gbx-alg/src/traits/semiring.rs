//! Semiring: two associative operations (`+`, `*`) with identities `0` and `1`.
//!
//! A semiring is intended to satisfy:
//! - `+` forms a monoid with identity `0`
//! - `*` forms a monoid with identity `1`
//! - `*` distributes over `+`:
//!   - `a * (b + c) == (a * b) + (a * c)`
//!   - `(a + b) * c == (a * c) + (b * c)`
//! - `0` is multiplicatively absorbing:
//!   - `0 * a == 0`
//!   - `a * 0 == 0`
//!
//! These laws are **not enforced** by the type system; they are expected to hold
//! and should be validated by tests (optionally via `gbx_alg::laws` helpers).

use crate::{AddMonoid, MulMonoid};

/// Marker trait for semirings.
pub trait Semiring: AddMonoid + MulMonoid {}

/// Blanket impl: any type that is both an additive monoid and multiplicative monoid
/// is considered a semiring by convention.
impl<T> Semiring for T where T: AddMonoid + MulMonoid {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Additive, Multiplicative, Zero};

    #[test]
    fn semiring_spotcheck_u32_distributivity_and_absorption() {
        let (a, b, c) = (2u32, 3, 5);

        assert_eq!(a.mul(b.add(c)), a.mul(b).add(a.mul(c)));
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));

        let z = u32::zero();
        assert_eq!(z.mul(a), z);
        assert_eq!(a.mul(z), z);

        fn requires_semiring<T: Semiring>(_x: T) {}
        requires_semiring(1u32);
    }
}
