//! Rings: additive group + multiplicative monoid (+ distributivity).
//!
//! A `Ring` is a [`Semiring`](crate::Semiring) whose additive
//! monoid is actually an **additive group**. That is:
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

use crate::{AddGroup, Semiring};

/// A ring: a semiring with additive inverses.
///
/// This trait is purely a *marker*: it enforces no laws directly. Implementors
/// are expected to satisfy the ring axioms.
pub trait Ring: Semiring + AddGroup {}

// Primitive signed integers form rings (mod 2ⁿ arithmetic).
macro_rules! impl_ring_for_ints {
    ($($t:ty),* $(,)?) => {
        $( impl Ring for $t {} )*
    };
}

impl_ring_for_ints!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod tests {
    use crate::{Additive, Multiplicative, Ring};

    #[test]
    fn distributivity_i32() {
        let (a, b, c) = (2i32, 3, 5);

        // left distributivity: a * (b + c) = a*b + a*c
        assert_eq!(a.mul(b.add(c)), a.mul(b).add(a.mul(c)));

        // right distributivity: (a + b) * c = a*c + b*c
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));

        // trait bound check
        fn _is_ring<T: Ring>(_x: T) {}
        _is_ring(a);
    }
}
