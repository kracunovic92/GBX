//! Field law checker.
//!
//! A field is (intended to be):
//! - a commutative ring (so ring laws + commutativity of `*`)
//! - every nonzero element has a multiplicative inverse

use core::fmt::Debug;
use core::ops::{Add, Mul, Neg};

use crate::{One, TryInverse, Zero};

use super::ops::check_mul_commutative;
use super::ring::check_ring;

/// Checks the field laws on a finite sample set.
///
/// Note: This asserts *total invertibility for all nonzero elements in `elems`*.
/// For correctness, your sample set should include all elements (finite field),
/// or at least a diverse subset (for larger fields).
#[inline]
pub fn check_field<T>(elems: &[T])
where
    T: Copy + Debug + PartialEq + Zero + One + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + TryInverse<Output = T>,
{
    check_ring(elems);
    check_mul_commutative(elems);

    let z = T::zero();
    let o = T::one();

    for &a in elems {
        if a == z {
            continue;
        }
        let inv = a
            .try_inv()
            .expect("nonzero element must be invertible in a field");
        assert_eq!(a * inv, o, "inverse law failed: a*inv(a) != 1 for a={a:?}");
        assert_eq!(inv * a, o, "inverse law failed: inv(a)*a != 1 for a={a:?}");
    }
}
