#![allow(clippy::needless_path_prefix)]
//! Rings: additive group + multiplicative monoid (+ distributivity).
//!
//! A `Ring` is a [`Semiring`](crate::Semiring) whose additive
//! monoid is actually an **abelian group**. That is:
//!
//! - (`+`) is an **abelian group**: associative, commutative, identity `0`,
//!   and every element has an inverse.
//! - (`*`) is a **monoid**: associative with identity `1`.
//! - Multiplication **distributes** over addition:
//!     - `a * (b + c) == a*b + a*c`
//!     - `(a + b) * c == a*c + b*c`
//!
//! We do not encode laws at the type level; implementations are expected to
//! behave as rings, and laws are checked via tests.

use crate::{AddAbelianGroup, Semiring};

/// A ring: a semiring with additive inverses (and additive commutativity).
///
/// This trait is purely a *marker*: it enforces no laws directly. Implementors
/// are expected to satisfy the ring axioms.
pub trait Ring: Semiring + AddAbelianGroup {}

/// Blanket impl: any type that is both a semiring and an additive abelian
/// group is considered a ring by convention.
///
/// This does *not* check distributivity or annihilation laws; those must be
/// verified in tests.
impl<T> Ring for T where T: Semiring + AddAbelianGroup {}

#[cfg(test)]
mod tests {
    use crate::{Additive, Multiplicative, Ring};

    #[test]
    fn distributivity_i32() {
        let (a, b, c) = (2i32, 3, 5);

        // left distributivity: a * (b + c) = a*b + a*c
        assert_eq!(
            a.mul(b.add(c)),
            a.mul(b)
                .add(a.mul(c))
        );

        // right distributivity: (a + b) * c = a*c + b*c
        assert_eq!(
            a.add(b)
                .mul(c),
            a.mul(c)
                .add(b.mul(c))
        );

        // trait bound check
        fn _is_ring<T: Ring>(_x: T) {}
        _is_ring(a);
    }
}
