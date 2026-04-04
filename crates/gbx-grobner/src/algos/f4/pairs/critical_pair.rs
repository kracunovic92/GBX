use gbx_poly::monomial::{checked_lcm_degree, Monomial, MonomialAlgos, MonomialError};

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
///
/// In F4, a critical pair is not reduced directly. Instead, it gives rise to
/// two aligned multiples, one from each side, which later enter symbolic
/// preprocessing and matrix construction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPair<M> {
    i: usize,
    j: usize,
    lcm: M,
    degree: u32,
    ti: M,
    tj: M,
}

/// One side of a critical pair.
///
/// This is a lightweight view describing which basis polynomial is used and
/// which monomial multiplier must be applied to align its leading monomial
/// with the pair lcm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairSide<'a, M> {
    /// Index of the basis polynomial.
    pub basis_index: usize,
    /// Monomial multiplier used on that basis polynomial.
    pub multiplier: &'a M,
}

impl<M> CriticalPair<M> {
    /// Return the smaller stored basis index.
    #[must_use]
    pub fn i(&self) -> usize {
        self.i
    }

    /// Return the larger stored basis index.
    #[must_use]
    pub fn j(&self) -> usize {
        self.j
    }

    /// Return both basis indices as `(i, j)` with `i < j`.
    #[must_use]
    pub fn indices(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    /// Return the pair lcm.
    #[must_use]
    pub fn lcm(&self) -> &M {
        &self.lcm
    }

    /// Return the total degree of the pair lcm.
    #[must_use]
    pub fn degree(&self) -> u32 {
        self.degree
    }

    /// Return the left multiplier `t_i`.
    #[must_use]
    pub fn ti(&self) -> &M {
        &self.ti
    }

    /// Return the right multiplier `t_j`.
    #[must_use]
    pub fn tj(&self) -> &M {
        &self.tj
    }

    /// Return both multipliers as `(&t_i, &t_j)`.
    #[must_use]
    pub fn multipliers(&self) -> (&M, &M) {
        (&self.ti, &self.tj)
    }

    /// Return the left side of the pair.
    #[must_use]
    pub fn left(&self) -> PairSide<'_, M> {
        PairSide { basis_index: self.i, multiplier: &self.ti }
    }

    /// Return the right side of the pair.
    #[must_use]
    pub fn right(&self) -> PairSide<'_, M> {
        PairSide { basis_index: self.j, multiplier: &self.tj }
    }

    /// Return both sides of the pair.
    #[must_use]
    pub fn sides(&self) -> [PairSide<'_, M>; 2] {
        [self.left(), self.right()]
    }
}
impl<M> CriticalPair<M>
where
    M: Clone + Monomial + MonomialAlgos,
{
    /// Construct a critical pair from two basis indices and their leading monomials.
    ///
    /// The indices are normalized so that the stored pair always satisfies `i < j`.
    ///
    /// The returned pair satisfies:
    ///
    /// - `lcm = lcm(lm_i, lm_j)`
    /// - `degree = deg(lcm)`
    /// - `ti * lm_i = lcm`
    /// - `tj * lm_j = lcm`
    ///
    /// # Errors
    ///
    /// Returns any monomial error produced while computing the lcm, its degree,
    /// or the exact quotient multipliers.
    pub fn from_lms(i: usize, j: usize, lm_i: &M, lm_j: &M) -> Result<Self, MonomialError> {
        assert_ne!(i, j, "critical pair requires distinct basis indices");

        let (i, j, lm_i, lm_j) = if i < j { (i, j, lm_i, lm_j) } else { (j, i, lm_j, lm_i) };

        let lcm = lm_i.checked_lcm(lm_j)?;
        let degree = checked_lcm_degree(lm_i, lm_j)?;
        let ti = lcm.checked_div_exact_by(lm_i)?;
        let tj = lcm.checked_div_exact_by(lm_j)?;

        Ok(Self { i, j, lcm, degree, ti, tj })
    }
}

#[cfg(test)]
impl<M> CriticalPair<M> {
    pub(crate) fn new_for_test(i: usize, j: usize, lcm: M, degree: u32, ti: M, tj: M) -> Self {
        let (i, j, ti, tj) = if i < j { (i, j, ti, tj) } else { (j, i, tj, ti) };
        Self { i, j, lcm, degree, ti, tj }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_ring;
    use gbx_field::fp::FpDynElem;
    use gbx_poly::monomial::checked_lcm_degree;
    use gbx_poly::order::Lex;
    use gbx_poly::poly;
    use gbx_poly::polynomial::{PolyDyn, PolynomialView};

    type P = PolyDyn<FpDynElem>;

    #[test]
    fn from_lms_normalizes_indices() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1

        let lm1 = f1.leading_mono().expect("f1 should be nonzero");
        let lm2 = f2.leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(5, 2, lm1, lm2).expect("pair construction should succeed");

        assert_eq!(pair.indices(), (2, 5));
        assert_eq!(pair.i(), 2);
        assert_eq!(pair.j(), 5);
    }

    #[test]
    fn from_lms_computes_expected_lcm_and_multipliers() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1

        let lm1 = f1.leading_mono().expect("f1 should be nonzero");
        let lm2 = f2.leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

        let left_aligned = pair
            .ti()
            .checked_mul(lm1)
            .expect("ti * lm1 should be defined");
        let right_aligned = pair
            .tj()
            .checked_mul(lm2)
            .expect("tj * lm2 should be defined");

        assert_eq!(&left_aligned, pair.lcm());
        assert_eq!(&right_aligned, pair.lcm());
    }

    #[test]
    fn degree_matches_lcm_degree() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1

        let lm1 = f1.leading_mono().expect("f1 should be nonzero");
        let lm2 = f2.leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

        let expected = checked_lcm_degree(lm1, lm2).expect("lcm degree should be defined");

        assert_eq!(pair.degree(), expected);
    }

    #[test]
    fn left_and_right_sides_match_stored_data() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f1: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let f2: P = poly![&ring; (1, [1, 1]), (1, [0, 0])].unwrap(); // x*y + 1

        let lm1 = f1.leading_mono().expect("f1 should be nonzero");
        let lm2 = f2.leading_mono().expect("f2 should be nonzero");

        let pair = CriticalPair::from_lms(0, 1, lm1, lm2).expect("pair construction should succeed");

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
    #[should_panic(expected = "critical pair requires distinct basis indices")]
    fn from_lms_rejects_equal_indices() {
        let ring = test_ring(7, 2, Lex).expect("test ring construction should succeed");

        let f: P = poly![&ring; (1, [2, 0]), (1, [0, 1])].unwrap(); // x^2 + y
        let lm = f.leading_mono().expect("f should be nonzero");

        let _ = CriticalPair::from_lms(0, 0, lm, lm);
    }
}
