/// Constructs a dynamic monomial.
///
/// # Example
///
/// ```
/// use gbx_poly::mono;
/// use gbx_poly::monomial::{Monomial, MonomialView};
///
/// let m = mono![1, 0, 2];
///
/// assert_eq!(m.exponents(), &[1, 0, 2]);
/// ```
#[macro_export]
macro_rules! mono {
    ($($e:expr),* $(,)?) => {{
        $crate::monomial::Monomial::from_slice(&[$($e as u32),*])
    }};
}

#[cfg(test)]
mod tests {
    use crate::monomial::MonomialView;

    #[test]
    fn mono_works() {
        let m = mono![1, 0, 2];

        assert_eq!(m.exponents(), &[1, 0, 2]);
        assert_eq!(m.degree(), 3);
    }
}
