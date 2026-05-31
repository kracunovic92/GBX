//! Operation-specific law checkers for `+`, `*`, `-`, `0`, `1`.
//!
//! These are thin wrappers around the internal `ops_generic` helpers, with cleaner bounds and
//! more ergonomic usage in ring/field checkers.
//!
//! Each checker validates the relevant axiom(s) on a **finite sample set** (`&[T]`).
//! They are intended for use in **tests** (not production code).

use core::fmt::Debug;
use core::ops::{Add, Mul, Neg};

use crate::{One, Zero};

use super::ops_generic as g;

/// Checks associativity of addition: `(a+b)+c == a+(b+c)` for all `a,b,c` in `elems`.
#[inline]
pub fn check_add_associative<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T>,
{
    g::check_associative(elems, |a, b| a + b);
}

/// Checks associativity of multiplication: `(a*b)*c == a*(b*c)` for all `a,b,c` in `elems`.
#[inline]
pub fn check_mul_associative<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Mul<Output = T>,
{
    g::check_associative(elems, |a, b| a * b);
}

/// Checks commutativity of addition: `a+b == b+a` for all `a,b` in `elems`.
#[inline]
pub fn check_add_commutative<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T>,
{
    g::check_commutative(elems, |a, b| a + b);
}

/// Checks commutativity of multiplication: `a*b == b*a` for all `a,b` in `elems`.
#[inline]
pub fn check_mul_commutative<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Mul<Output = T>,
{
    g::check_commutative(elems, |a, b| a * b);
}

/// Checks additive identity: `a+0 == a` and `0+a == a` for all `a` in `elems`.
#[inline]
pub fn check_add_identity<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Add<Output = T>,
{
    g::check_identity(elems, |a, b| a + b, T::zero());
}

/// Checks multiplicative identity: `a*1 == a` and `1*a == a` for all `a` in `elems`.
#[inline]
pub fn check_mul_identity<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + One + Mul<Output = T>,
{
    g::check_identity(elems, |a, b| a * b, T::one());
}

/// Checks additive inverses: `a + (-a) == 0` and `(-a) + a == 0` for all `a` in `elems`.
#[inline]
pub fn check_add_inverse<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Add<Output = T> + Neg<Output = T>,
{
    g::check_inverse(elems, |a, b| a + b, |a| -a, T::zero());
}

/// Checks **left** distributivity: `a*(b+c) == a*b + a*c` for all `a,b,c` in `elems`.
#[inline]
pub fn check_distributive_left<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    g::check_left_distributive(elems, |a, b| a + b, |a, b| a * b);
}

/// Checks **right** distributivity: `(a+b)*c == a*c + b*c` for all `a,b,c` in `elems`.
#[inline]
pub fn check_distributive_right<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    g::check_right_distributive(elems, |a, b| a + b, |a, b| a * b);
}

/// Checks both left and right distributivity.
#[inline]
pub fn check_distributive<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Add<Output = T> + Mul<Output = T>,
{
    g::check_distributive(elems, |a, b| a + b, |a, b| a * b);
}

/// Checks the (optional, semiring) axiom that `0` is absorbing for multiplication:
/// `0*a == 0` and `a*0 == 0` for all `a` in `elems`.
#[inline]
pub fn check_zero_absorbing_mul<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + Mul<Output = T>,
{
    g::check_absorbing(elems, |a, b| a * b, T::zero());
}
