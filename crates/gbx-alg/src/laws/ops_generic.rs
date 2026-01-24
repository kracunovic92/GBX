//! Generic (operation-parameterized) law checkers.
//!
//! These helpers take operations as `Fn` arguments, so they can be reused for
//! any algebraic structure (addition, multiplication, custom ops, etc.).
//!
//! The wrappers in `ops.rs` provide ergonomic versions for `+`, `*`, `-`, `0`, `1`.

use core::fmt::Debug;

/// Checks associativity: op(op(a,b),c) == op(a,op(b,c)) for all a,b,c in elems.
#[inline]
pub fn check_associative<T, Op>(elems: &[T], op: Op)
where
    T: Copy + Debug + PartialEq,
    Op: Fn(T, T) -> T,
{
    for &a in elems {
        for &b in elems {
            for &c in elems {
                let left = op(op(a, b), c);
                let right = op(a, op(b, c));
                assert_eq!(
                    left, right,
                    "associativity failed: a={a:?}, b={b:?}, c={c:?}"
                );
            }
        }
    }
}

/// Checks commutativity: op(a,b) == op(b,a) for all a,b.
#[inline]
pub fn check_commutative<T, Op>(elems: &[T], op: Op)
where
    T: Copy + Debug + PartialEq,
    Op: Fn(T, T) -> T,
{
    for &a in elems {
        for &b in elems {
            let ab = op(a, b);
            let ba = op(b, a);
            assert_eq!(ab, ba, "commutativity failed: a={a:?}, b={b:?}");
        }
    }
}

/// Checks identity element `e`: op(e,a) == a and op(a,e) == a for all a.
#[inline]
pub fn check_identity<T, Op>(elems: &[T], op: Op, e: T)
where
    T: Copy + Debug + PartialEq,
    Op: Fn(T, T) -> T,
{
    for &a in elems {
        assert_eq!(op(e, a), a, "left identity failed: e={e:?}, a={a:?}");
        assert_eq!(op(a, e), a, "right identity failed: e={e:?}, a={a:?}");
    }
}

/// Checks inverse operation `inv` w.r.t. identity `e`: op(a,inv(a)) == e and op(inv(a),a) == e.
#[inline]
pub fn check_inverse<T, Op, Inv>(elems: &[T], op: Op, inv: Inv, e: T)
where
    T: Copy + Debug + PartialEq,
    Op: Fn(T, T) -> T,
    Inv: Fn(T) -> T,
{
    for &a in elems {
        let ia = inv(a);
        assert_eq!(
            op(a, ia),
            e,
            "right inverse failed: a={a:?}, inv(a)={ia:?}, e={e:?}"
        );
        assert_eq!(
            op(ia, a),
            e,
            "left inverse failed: a={a:?}, inv(a)={ia:?}, e={e:?}"
        );
    }
}

/// Checks absorption: op(absorb,a) == absorb and op(a,absorb) == absorb for all a.
///
/// Typical use: `0` absorbing for multiplication in a semiring.
#[inline]
pub fn check_absorbing<T, Op>(elems: &[T], op: Op, absorb: T)
where
    T: Copy + Debug + PartialEq,
    Op: Fn(T, T) -> T,
{
    for &a in elems {
        assert_eq!(
            op(absorb, a),
            absorb,
            "left absorbing failed: absorb={absorb:?}, a={a:?}"
        );
        assert_eq!(
            op(a, absorb),
            absorb,
            "right absorbing failed: absorb={absorb:?}, a={a:?}"
        );
    }
}

/// Checks left distributivity: mul(a, add(b,c)) == add(mul(a,b), mul(a,c)) for all a,b,c.
#[inline]
pub fn check_left_distributive<T, AddOp, MulOp>(elems: &[T], add: AddOp, mul: MulOp)
where
    T: Copy + Debug + PartialEq,
    AddOp: Fn(T, T) -> T,
    MulOp: Fn(T, T) -> T,
{
    for &a in elems {
        for &b in elems {
            for &c in elems {
                let left = mul(a, add(b, c));
                let right = add(mul(a, b), mul(a, c));
                assert_eq!(
                    left, right,
                    "left distributivity failed: a={a:?}, b={b:?}, c={c:?}"
                );
            }
        }
    }
}

/// Checks right distributivity: mul(add(a,b), c) == add(mul(a,c), mul(b,c)) for all a,b,c.
#[inline]
pub fn check_right_distributive<T, AddOp, MulOp>(elems: &[T], add: AddOp, mul: MulOp)
where
    T: Copy + Debug + PartialEq,
    AddOp: Fn(T, T) -> T,
    MulOp: Fn(T, T) -> T,
{
    for &a in elems {
        for &b in elems {
            for &c in elems {
                let left = mul(add(a, b), c);
                let right = add(mul(a, c), mul(b, c));
                assert_eq!(
                    left, right,
                    "right distributivity failed: a={a:?}, b={b:?}, c={c:?}"
                );
            }
        }
    }
}

/// Checks both left and right distributivity.
#[inline]
pub fn check_distributive<T, AddOp, MulOp>(elems: &[T], add: AddOp, mul: MulOp)
where
    T: Copy + Debug + PartialEq,
    AddOp: Fn(T, T) -> T,
    MulOp: Fn(T, T) -> T,
{
    check_left_distributive(elems, &add, &mul);
    check_right_distributive(elems, add, mul);
}
