/// Construct a monomial from an exponent list.
///
/// This macro is **context-sensitive**:
/// - If the expected type is `FixedMonomial<N>`, it constructs a fixed monomial.
/// - If the expected type is `DynamicMonomial`, it constructs a dynamic monomial.
///
/// # Examples
/// ```
/// use gbx_poly::monomial::{FixedMonomial, DynamicMonomial, MonomialView};
/// use gbx_poly::mono;
/// let a: FixedMonomial<3> = mono![1, 0, 2];
/// let b: DynamicMonomial  = mono![1, 0, 2];
///
/// assert_eq!(a.exponents(), b.exponents());
/// ```
#[macro_export]
macro_rules! mono {
    ($($e:expr),* $(,)?) => {{
        ::core::convert::From::from([$( $e as u32 ),*])
    }};
}

/// Construct a `DynamicMonomial` explicitly.
///
/// # Examples
/// ```
/// use gbx_poly::monomial::{DynamicMonomial, MonomialView};
/// use gbx_poly::mono_dyn;
///
/// let m: DynamicMonomial = mono_dyn![0, 1, 0, 3];
/// assert_eq!(m.exponents(), &[0, 1, 0, 3]);
/// ```
#[macro_export]
macro_rules! mono_dyn {
    ($($e:expr),* $(,)?) => {{
        $crate::monomial::DynamicMonomial::from_slice(&[$( $e as u32 ),*])
    }};
}

/// Construct a `FixedMonomial<N>` explicitly, with a compile-time arity check.
///
/// Usage: `mono_fixed![3; 1, 0, 2]`
///
/// # Examples
/// ```
/// use gbx_poly::monomial::{FixedMonomial, MonomialView};
/// use gbx_poly::mono_fixed;
///
/// let m: FixedMonomial<3> = mono_fixed![3; 1, 0, 2];
/// assert_eq!(m.exponents(), &[1, 0, 2]);
/// ```
#[macro_export]
macro_rules! mono_fixed {
    ($n:literal; $($e:expr),* $(,)?) => {{
        let tmp = [$( $e as u32 ),*];
        // compile-time check that len(tmp) == $n
        let _: [u32; $n] = tmp;
        ::core::convert::From::from(tmp)
    }};
}

#[cfg(test)]
mod tests {
    use crate::monomial::{DynamicMonomial, FixedMonomial, MonomialView};

    #[test]
    fn mono_infers_fixed() {
        let m: FixedMonomial<3> = mono![1, 0, 2];
        assert_eq!(m.exponents(), &[1, 0, 2]);
        assert_eq!(m.degree(), 3);
    }

    #[test]
    fn mono_infers_dynamic() {
        let m: DynamicMonomial = mono![1, 0, 2];
        assert_eq!(m.exponents(), &[1, 0, 2]);
        assert_eq!(m.degree_hint(), Some(3));
    }

    #[test]
    fn mono_dyn_works() {
        let m = mono_dyn![0, 1, 0, 3];
        assert_eq!(m.exponents(), &[0, 1, 0, 3]);
        assert_eq!(m.degree_hint(), Some(4));
    }

    #[test]
    fn mono_fixed_works() {
        let m: FixedMonomial<3> = mono_fixed![3; 1, 0, 2];
        assert_eq!(m.exponents(), &[1, 0, 2]);
        assert_eq!(m.degree(), 3);
    }
}
