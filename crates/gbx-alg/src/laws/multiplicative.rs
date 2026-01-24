//! Multiplicative structure checkers.
//!
//! These compose primitive operation laws from [`crate::laws::ops`].

use core::fmt::Debug;

use crate::{Multiplicative, One};

use super::ops;

/// Checks that multiplication forms a monoid:
/// associativity + identity element `1`.
pub fn check_mul_monoid<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Multiplicative + One,
{
    ops::check_mul_associative(elems);
    ops::check_mul_identity(elems);
}

/// Checks that multiplication is commutative: `a*b == b*a`.
pub fn check_mul_abelian<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Multiplicative,
{
    ops::check_mul_commutative(elems);
}
