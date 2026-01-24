#[cfg(test)]
mod tests {
    use crate::monomial::MonomialView;
    use crate::term::{DynamicTerm, FixedTerm, TermOps};
    use gbx_field::fp::Fp;

    type F7 = Fp<7>;

    #[test]
    fn fixed_mul_term() {
        let a = FixedTerm::<F7, 2>::from_coeff_and_exponents(F7::new(3), [1, 2]);
        let b = FixedTerm::<F7, 2>::from_coeff_and_exponents(F7::new(5), [2, 1]);
        let c = a.mul_term(&b);
        assert_eq!(c.mono.exponents(), &[3, 3]);
    }

    #[test]
    fn dynamic_mul_term() {
        let a = DynamicTerm::<F7>::from_coeff_and_slice(F7::new(3), &[1, 2]);
        let b = DynamicTerm::<F7>::from_coeff_and_slice(F7::new(5), &[2, 1]);
        let c = a.mul_term(&b);
        assert_eq!(c.mono.exponents(), &[3, 3]);
    }
}
