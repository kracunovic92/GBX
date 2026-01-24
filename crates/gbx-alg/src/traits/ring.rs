//! Rings: additive abelian group + multiplicative monoid + distributivity.
//!
//! Intended laws:
//! - `(R, +)` is an **abelian group** (associative, commutative, identity `0`, inverses)
//! - `(R, *)` is a **monoid** (associative, identity `1`)
//! - distributivity:
//!   - `a * (b + c) == a*b + a*c`
//!   - `(a + b) * c == a*c + b*c`
//!
//! These laws are **not enforced** by the type system; they are expected to hold
//! and should be validated by tests (optionally via `gbx_alg::laws` helpers).

use crate::{AddAbelianGroup, Semiring};

/// Marker trait for rings.
///
/// This is a marker-only trait; it does not add methods.
pub trait Ring: Semiring + AddAbelianGroup {}

/// Blanket impl: any type that is a semiring and an additive abelian group
/// is considered a ring by convention.
impl<T> Ring for T where T: Semiring + AddAbelianGroup {}

#[cfg(test)]
mod tests {
    use crate::{Additive, Multiplicative};

    #[test]
    fn distributivity_i32() {
        let (a, b, c) = (2i32, 3, 5);

        assert_eq!(a.mul(b.add(c)), a.mul(b).add(a.mul(c)));
        assert_eq!(a.add(b).mul(c), a.mul(c).add(b.mul(c)));
    }
}
