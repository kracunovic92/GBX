use gbx_poly::monomial::{checked_lcm, checked_lcm_degree, checked_quotient, Monomial, MonomialError};

/// One critical pair used by Buchberger-style algorithms and by F4.
///
/// A critical pair is determined by two basis elements `f_i` and `f_j`.
/// It stores:
///
/// - the normalized basis indices `i < j`,
/// - the least common multiple of their leading monomials,
/// - the total degree of that lcm,
/// - the monomial multipliers `t_i` and `t_j` such that
///   `t_i * LM(f_i) = t_j * LM(f_j) = lcm`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPair {
    i: usize,
    j: usize,
    lcm: Monomial,
    degree: u32,
    ti: Monomial,
    tj: Monomial,
}

/// One side of a critical pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairSide<'a> {
    /// Index of the basis polynomial.
    pub basis_index: usize,

    /// Monomial multiplier used on that basis polynomial.
    pub multiplier: &'a Monomial,
}

impl CriticalPair {
    #[must_use]
    pub fn i(&self) -> usize {
        self.i
    }

    #[must_use]
    pub fn j(&self) -> usize {
        self.j
    }

    #[must_use]
    pub fn indices(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    #[must_use]
    pub fn lcm(&self) -> &Monomial {
        &self.lcm
    }

    #[must_use]
    pub fn degree(&self) -> u32 {
        self.degree
    }

    #[must_use]
    pub fn ti(&self) -> &Monomial {
        &self.ti
    }

    #[must_use]
    pub fn tj(&self) -> &Monomial {
        &self.tj
    }

    #[must_use]
    pub fn multipliers(&self) -> (&Monomial, &Monomial) {
        (&self.ti, &self.tj)
    }

    #[must_use]
    pub fn left(&self) -> PairSide<'_> {
        PairSide { basis_index: self.i, multiplier: &self.ti }
    }

    #[must_use]
    pub fn right(&self) -> PairSide<'_> {
        PairSide { basis_index: self.j, multiplier: &self.tj }
    }

    #[must_use]
    pub fn sides(&self) -> [PairSide<'_>; 2] {
        [self.left(), self.right()]
    }

    /// Construct a critical pair from two basis indices and their leading monomials.
    ///
    /// Indices are normalized so the stored pair always satisfies `i < j`.
    pub fn from_lms(i: usize, j: usize, lm_i: &Monomial, lm_j: &Monomial) -> Result<Self, MonomialError> {
        assert_ne!(i, j, "critical pair requires distinct basis indices");

        let (i, j, lm_i, lm_j) = if i < j { (i, j, lm_i, lm_j) } else { (j, i, lm_j, lm_i) };

        let lcm = checked_lcm(lm_i, lm_j)?;
        let degree = checked_lcm_degree(lm_i, lm_j)?;

        let ti = checked_quotient(lm_i, &lcm)?.ok_or(MonomialError::NotDivisible)?;

        let tj = checked_quotient(lm_j, &lcm)?.ok_or(MonomialError::NotDivisible)?;

        Ok(Self { i, j, lcm, degree, ti, tj })
    }
}

#[cfg(test)]
impl CriticalPair {
    pub(crate) fn new_for_test(i: usize, j: usize, lcm: Monomial, degree: u32, ti: Monomial, tj: Monomial) -> Self {
        let (i, j, ti, tj) = if i < j { (i, j, ti, tj) } else { (j, i, tj, ti) };

        Self { i, j, lcm, degree, ti, tj }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_ring;

    use gbx_field::fp::FpElem;
    use gbx_poly::monomial::{checked_lcm_degree, MonomialView};
    use gbx_poly::order::{Grevlex, Lex};
    use gbx_poly::poly;
    use gbx_poly::polynomial::{Polynomial, PolynomialView};

    type P = Polynomial<FpElem>;

    #[test]
    fn from_lms_normalizes_indices() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let lm1 = f1.leading_mono().unwrap();
        let lm2 = f2.leading_mono().unwrap();

        let pair = CriticalPair::from_lms(5, 2, lm1, lm2).unwrap();

        assert_eq!(pair.indices(), (2, 5));
        assert_eq!(pair.i(), 2);
        assert_eq!(pair.j(), 5);
    }

    #[test]
    fn from_lms_computes_expected_lcm_and_multipliers() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let lm1 = f1.leading_mono().unwrap();
        let lm2 = f2.leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        let left_aligned = pair.ti().checked_mul(lm1).unwrap();
        let right_aligned = pair.tj().checked_mul(lm2).unwrap();

        assert_eq!(&left_aligned, pair.lcm());
        assert_eq!(&right_aligned, pair.lcm());
    }

    #[test]
    fn degree_matches_lcm_degree() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let lm1 = f1.leading_mono().unwrap();
        let lm2 = f2.leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();
        let expected = checked_lcm_degree(lm1, lm2).unwrap();

        assert_eq!(pair.degree(), expected);
    }

    #[test]
    fn left_and_right_sides_match_stored_data() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap();

        let lm1 = f1.leading_mono().unwrap();
        let lm2 = f2.leading_mono().unwrap();

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).unwrap();

        let left = pair.left();
        let right = pair.right();
        let [a, b] = pair.sides();

        assert_eq!(left.basis_index, 0);
        assert_eq!(right.basis_index, 1);

        assert_eq!(left.multiplier, pair.ti());
        assert_eq!(right.multiplier, pair.tj());

        assert_eq!(a, left);
        assert_eq!(b, right);
    }

    #[test]
    fn pair_construction_uses_order_dependent_leading_monomials_lex() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        // Lex: LM(x^2 + y^5) = x^2
        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2: P = poly![&ring; (1, [1, 0])].unwrap();

        let pair = CriticalPair::from_lms(0, 1, f1.leading_mono().unwrap(), f2.leading_mono().unwrap()).unwrap();

        assert_eq!(pair.lcm().exponents(), &[2, 0]);
        assert_eq!(pair.degree(), 2);
    }

    #[test]
    fn pair_construction_uses_order_dependent_leading_monomials_grevlex() {
        let ring = test_ring(7, 2, Grevlex).expect("test ring construction should succeed");

        // Grevlex: LM(x^2 + y^5) = y^5
        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 5])].unwrap();
        let f2: P = poly![&ring; (1, [1, 0])].unwrap();

        let pair = CriticalPair::from_lms(0, 1, f1.leading_mono().unwrap(), f2.leading_mono().unwrap()).unwrap();

        assert_eq!(pair.lcm().exponents(), &[1, 5]);
        assert_eq!(pair.degree(), 6);
    }

    #[test]
    #[should_panic(expected = "critical pair requires distinct basis indices")]
    fn from_lms_rejects_equal_indices() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap();
        let lm = f.leading_mono().unwrap();

        let _ = CriticalPair::from_lms(0, 0, lm, lm);
    }
}
