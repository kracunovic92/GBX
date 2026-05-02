/// Constructs a term.
///
/// # Examples
///
/// ```
/// use gbx_poly::term;
/// use gbx_poly::monomial::MonomialView;
///
/// let t = term![7u32, [1, 0, 2]];
///
/// assert_eq!(*t.coeff(), 7);
/// assert_eq!(t.mono().exponents(), &[1, 0, 2]);
/// ```
#[macro_export]
macro_rules! term {
    ($c:expr, [$($e:expr),* $(,)?]) => {{
        $crate::term::Term::new(
            $c,
            $crate::monomial::Monomial::from_slice(&[$($e as u32),*]),
        )
    }};

    ($c:expr, $m:expr) => {{
        $crate::term::Term::new($c, $m)
    }};
}

#[cfg(test)]
mod tests {
    use crate::monomial::{Monomial, MonomialView};

    #[test]
    fn term_macro_builds_term() {
        let t = term![5u32, [1, 0, 2]];

        assert_eq!(*t.coeff(), 5);
        assert_eq!(t.mono().exponents(), &[1, 0, 2]);
    }

    #[test]
    fn term_macro_accepts_monomial_expr() {
        let m = Monomial::from_slice(&[3, 4]);
        let t = term![2u32, m];

        assert_eq!(*t.coeff(), 2);
        assert_eq!(t.mono().exponents(), &[3, 4]);
    }
}
