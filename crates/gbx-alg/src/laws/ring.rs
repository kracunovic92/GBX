//! Ring law checker.
//!
//! A ring is (intended to be):
//! - additive abelian group: `+` associative/commutative, identity `0`, inverse `-a`
//! - multiplicative monoid: `*` associative with identity `1`
//! - distributivity of `*` over `+`

use core::fmt::Debug;
use core::ops::{Add, Mul, Neg};

use crate::{One, Zero};

use super::additive::check_add_abelian_group;
use super::multiplicative::check_mul_monoid;
use super::ops::check_distributive;

/// Checks the ring laws on a finite sample set.
#[inline]
pub fn check_ring<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + One + Add<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    check_add_abelian_group(elems);
    check_mul_monoid(elems);
    check_distributive(elems);
}
