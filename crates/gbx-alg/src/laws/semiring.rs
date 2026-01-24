//! Semiring law checker.
//!
//! A semiring is (intended to be):
//! - additive monoid: associative `+` with identity `0`
//! - multiplicative monoid: associative `*` with identity `1`
//! - distributivity of `*` over `+`
//! - `0` absorbing for multiplication (optional but standard in most definitions)

use core::fmt::Debug;
use core::ops::{Add, Mul};

use crate::{One, Zero};

use super::additive::check_add_monoid;
use super::multiplicative::check_mul_monoid;
use super::ops::check_distributive;

/// Checks the semiring laws on a finite sample set.
#[inline]
pub fn check_semiring<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + One + Add<Output = T> + Mul<Output = T>,
{
    check_add_monoid(elems);
    check_mul_monoid(elems);
    check_distributive(elems);
}
