/// Term construction macros.
///
/// These macros are designed to be ergonomic in tests and user-facing APIs.
///
/// # Examples
/// ```
/// use gbx_poly::monomial::{FixedMonomial, DynamicMonomial};
/// use gbx_poly::term::Term;
/// use gbx_poly::term;
/// use crate::gbx_poly::monomial::MonomialView;
///
/// // Fixed arity inferred from the expected type.
/// let a: Term<u32, FixedMonomial<3>> = term![3u32, [1, 0, 2]];
///
/// // Dynamic arity inferred from the expected type.
/// let b: Term<u32, DynamicMonomial> = term![7u32, [0, 1, 0, 3]];
///
/// assert_eq!(*a.coeff_ref(), 3);
/// assert_eq!(a.mono_ref().exponents(), &[1, 0, 2]);
/// assert_eq!(*b.coeff_ref(), 7);
/// assert_eq!(b.mono_ref().exponents(), &[0, 1, 0, 3]);
/// ```
#[macro_export]
macro_rules! term {
    // term![coeff, [e0, e1, ...]]
    ($c:expr, [$($e:expr),* $(,)?]) => {{
        $crate::term::Term::new(
            ::core::convert::Into::into($c),
            ::core::convert::From::from([$( $e as u32 ),*]),
        )
    }};

    // term![coeff, monomial_expr]
    ($c:expr, $m:expr) => {{
        $crate::term::Term::new(::core::convert::Into::into($c), $m)
    }};
}

#[cfg(test)]
mod tests {
    use crate::monomial::{DynamicMonomial, FixedMonomial, MonomialView};
    use crate::term::Term;

    #[test]
    fn term_macro_builds_fixed_by_inference() {
        type T = Term<u32, FixedMonomial<3>>;
        let t: T = term![5u32, [1, 0, 2]];
        assert_eq!(*t.coeff_ref(), 5);
        assert_eq!(t.mono_ref().exponents(), &[1, 0, 2]);
    }

    #[test]
    fn term_macro_builds_dynamic_by_inference() {
        type T = Term<u32, DynamicMonomial>;
        let t: T = term![9u32, [0, 1, 0, 3]];
        assert_eq!(*t.coeff_ref(), 9);
        assert_eq!(t.mono_ref().exponents(), &[0, 1, 0, 3]);
    }

    #[test]
    fn term_macro_accepts_monomial_expr() {
        type T = Term<u32, FixedMonomial<2>>;
        let m = FixedMonomial::<2>::from_exponents([3, 4]);
        let t: T = term![2u32, m];
        assert_eq!(*t.coeff_ref(), 2);
        assert_eq!(t.mono_ref().exponents(), &[3, 4]);
    }
}
