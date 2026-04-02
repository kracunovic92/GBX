use gbx_poly::monomial::{checked_lcm_degree, Monomial, MonomialAlgos, MonomialError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPair<M> {
    pub i: usize,
    pub j: usize,
    pub lcm: M,
    pub degree: u32,
    pub ti: M,
    pub tj: M,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairProjection<'a, M> {
    pub basis_index: usize,
    pub multiplier: &'a M,
}

impl<M> CriticalPair<M>
where
    M: Clone + Monomial + MonomialAlgos,
{
    pub fn from_lms(i: usize, j: usize, lm_i: &M, lm_j: &M) -> Result<Self, MonomialError> {
        debug_assert!(i != j);

        let (i, j, lm_i, lm_j) = if i <= j { (i, j, lm_i, lm_j) } else { (j, i, lm_j, lm_i) };

        let lcm = lm_i.checked_lcm(lm_j)?;
        let degree = checked_lcm_degree(lm_i, lm_j)?;
        let ti = lcm.checked_div_exact_by(lm_i)?;
        let tj = lcm.checked_div_exact_by(lm_j)?;

        Ok(Self { i, j, lcm, degree, ti, tj })
    }

    #[must_use]
    pub fn degree(&self) -> u32 {
        self.degree
    }

    #[must_use]
    pub fn left(&self) -> PairProjection<'_, M> {
        PairProjection { basis_index: self.i, multiplier: &self.ti }
    }

    #[must_use]
    pub fn right(&self) -> PairProjection<'_, M> {
        PairProjection { basis_index: self.j, multiplier: &self.tj }
    }

    #[must_use]
    pub fn projections(&self) -> [PairProjection<'_, M>; 2] {
        [self.left(), self.right()]
    }

    #[must_use]
    pub fn indices(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    #[must_use]
    pub fn multipliers(&self) -> (&M, &M) {
        (&self.ti, &self.tj)
    }
}
