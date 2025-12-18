#![allow(missing_docs)]
use algebra_core::{Additive, CheckedDiv, Multiplicative, One, Zero};
use algebra_field::Zp;
use proptest::prelude::*;

type F7 = Zp<7>;

prop_compose! {
    fn arb_f7()(x in 0u64..1000u64) -> F7 {
        F7::new(x)
    }
}

proptest! {
    #[test]
    fn add_associative(a in arb_f7(), b in arb_f7(), c in arb_f7()) {
        let lhs = a.add(b).add(c);
        let rhs = a.add(b.add(c));
        prop_assert_eq!(lhs.value(), rhs.value());
    }

    #[test]
    fn add_identity(a in arb_f7()) {
        let z = F7::zero();
        prop_assert_eq!(z.add(a).value(), a.value());
        prop_assert_eq!(a.add(z).value(), a.value());
    }

    #[test]
    fn mul_associative(a in arb_f7(), b in arb_f7(), c in arb_f7()) {
        let lhs = a.mul(b).mul(c);
        let rhs = a.mul(b.mul(c));
        prop_assert_eq!(lhs.value(), rhs.value());
    }

    #[test]
    fn mul_identity(a in arb_f7()) {
        let o = F7::one();
        prop_assert_eq!(o.mul(a).value(), a.value());
        prop_assert_eq!(a.mul(o).value(), a.value());
    }

    #[test]
    fn distributivity(a in arb_f7(), b in arb_f7(), c in arb_f7()) {
        let left = a.mul(b.add(c));
        let right = a.mul(b).add(a.mul(c));
        prop_assert_eq!(left.value(), right.value());
    }

    #[test]
    fn zero_is_absorbing(a in arb_f7()) {
        let z = F7::zero();
        prop_assert_eq!(z.mul(a).value(), z.value());
        prop_assert_eq!(a.mul(z).value(), z.value());
    }

    #[test]
    fn checked_div_matches_inverse_for_nonzero(a in arb_f7(), b in arb_f7()) {
        // filter out zero denominator
        prop_assume!(b.value() != 0);

        let q = a.checked_div(b).expect("b != 0 so division should succeed");
        let back = q.mul(b);
        prop_assert_eq!(back.value(), a.value());
    }
}
