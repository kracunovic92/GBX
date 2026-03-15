use crate::pairing::PairCriterion;
use crate::GrobnerBasis;

/// Logical conjunction of two local pair pairing.
///
/// A pair `(i, j)` is kept if and only if **both** inner pairing keep it.
///
/// # Purpose
///
/// This combinator allows simple composition of sound local pairing without
/// introducing a dedicated concrete type for every combination.
#[derive(Debug, Default, Clone, Copy)]
pub struct AndCriterion<A, B> {
    pub left: A,
    pub right: B,
}

impl<A, B> AndCriterion<A, B> {
    /// Creates a new conjunction of two pair pairing.
    #[must_use]
    #[inline]
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<P, A, B> PairCriterion<P> for AndCriterion<A, B>
where
    A: PairCriterion<P>,
    B: PairCriterion<P>,
{
    #[inline]
    fn keep_pair(&mut self, gb: &GrobnerBasis<P>, i: usize, j: usize) -> bool {
        self.left.keep_pair(gb, i, j) && self.right.keep_pair(gb, i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::NoPairCriterion;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[derive(Debug, Default, Clone, Copy)]
    struct RejectAll;

    impl<P> PairCriterion<P> for RejectAll {
        fn keep_pair(&mut self, _gb: &GrobnerBasis<P>, _i: usize, _j: usize) -> bool {
            false
        }
    }

    #[test]
    fn keeps_only_if_both_keep() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let gb = GrobnerBasis::new(ring.id(), Vec::<()>::new());

        let mut both_yes = AndCriterion::new(NoPairCriterion, NoPairCriterion);
        assert!(both_yes.keep_pair(&gb, 0, 1));

        let mut left_no = AndCriterion::new(RejectAll, NoPairCriterion);
        assert!(!left_no.keep_pair(&gb, 0, 1));

        let mut right_no = AndCriterion::new(NoPairCriterion, RejectAll);
        assert!(!right_no.keep_pair(&gb, 0, 1));
    }
}
