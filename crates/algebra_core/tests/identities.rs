#![allow(missing_docs)]

use algebra_core::{One, Zero};
use core::fmt::Debug;

fn check_identities<T>(zero: T, one: T)
where
    T: Zero + One + Copy + Eq + Debug,
{
    // const/associated value checks
    assert_eq!(T::ZERO, zero);
    assert_eq!(T::ONE, one);

    // constructor helpers
    assert_eq!(T::zero(), zero);
    assert_eq!(T::one(), one);

    // predicates
    assert!(zero.is_zero());
    assert!(!one.is_zero());

    assert!(one.is_one());
    assert!(!zero.is_one());
}

#[test]
fn integers_have_correct_zero_and_one() {
    check_identities::<u32>(0, 1);
    check_identities::<i64>(0, 1);
    check_identities::<usize>(0, 1);
}
