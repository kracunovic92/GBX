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
//! Additive inverses are _not_ required (unlike rings), which lets semirings
//! model many “weighted” structures: shortest paths, tropical algebra, and so on.

use crate::{AddMonoid, MulMonoid};

/// Basic semiring marker.
///
/// The laws listed in the module-level documentation are **not** enforced by
/// the type system; they are expected to hold by convention and to be checked
/// by law tests. Implementing this trait signals that a type is *intended* to
/// behave as a semiring.
pub trait Semiring: AddMonoid + MulMonoid {}

// Signed and unsigned integers are semirings.
// (Signed integers are also rings; every ring is a semiring.)
macro_rules! impl_semiring_for_ints {
    ($($t:ty),* $(,)?) => {
        $( impl Semiring for $t {} )*
    };
}

impl_semiring_for_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

#[cfg(test)]
mod tests {
    use crate::{Additive, Multiplicative, Semiring, Zero};

    #[test]
    fn semiring_spotcheck_u32() {
        let (a, b, c) = (2u32, 3, 5);

        // Sanity-check left distributivity:
        assert_eq!(a.mul(b.add(c)), a.mul(b).add(a.mul(c)));

        // And annihilation by zero: 0 * a = 0, a * 0 = 0
        let z = u32::zero();
        assert_eq!(z.mul(a), z);
        assert_eq!(a.mul(z), z);

        // Just to exercise the trait bound:
        fn _is_semiring<T: Semiring>(_x: T) {}
        _is_semiring(a);
    }
}
