//! Additive structure law checkers (semigroup/monoid/group/abelian group).
//!
//! These are *compositions* of atomic operation laws from `laws::ops`.
//! Use these in tests for concrete types.

use core::fmt::Debug;
use core::ops::{Add, Neg};

use crate::Zero;

use super::ops::{check_add_associative, check_add_commutative, check_add_identity, check_add_inverse};

/// Checks additive semigroup laws: associativity of `+`.
#[inline]
pub fn check_add_semigroup<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T>,
{
    check_add_associative(elems);
}

/// Checks additive monoid laws: associativity + identity `0`.
#[inline]
pub fn check_add_monoid<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Add<Output = T>,
{
    check_add_associative(elems);
    check_add_identity(elems);
}

/// Checks additive group laws: additive monoid + inverses (`-a`).
#[inline]
pub fn check_add_group<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Add<Output = T> + Neg<Output = T>,
{
    check_add_monoid(elems);
    check_add_inverse(elems);
}

/// Checks additive abelian group laws: additive group + commutativity.
#[inline]
pub fn check_add_abelian_group<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Add<Output = T> + Neg<Output = T>,
{
    check_add_group(elems);
    check_add_commutative(elems);
}
