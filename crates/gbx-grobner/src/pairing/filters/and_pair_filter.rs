use crate::pairing::filters::{PairFilter, PairSetView};
use crate::GrobnerBasis;

/// Logical conjunction of two pair filters.
///
/// A pair survives if and only if both inner filters keep it.
#[derive(Debug, Default, Clone, Copy)]
pub struct AndPairFilter<A, B> {
    pub left: A,
    pub right: B,
}

impl<A, B> AndPairFilter<A, B> {
    #[must_use]
    #[inline]
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

impl<P, A, B> PairFilter<P> for AndPairFilter<A, B>
where
    A: PairFilter<P>,
    B: PairFilter<P>,
{
    #[inline]
    fn keep_pair<S: PairSetView>(&mut self, gb: &GrobnerBasis<P>, state: &S, i: usize, j: usize) -> bool {
        self.left.keep_pair(gb, state, i, j) && self.right.keep_pair(gb, state, i, j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::filters::NoPairFilter;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[derive(Debug, Default)]
    struct EmptyState;

    impl PairSetView for EmptyState {
        fn contains_pair(&self, _i: usize, _j: usize) -> bool {
            false
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    struct RejectAll;

    impl<P> PairFilter<P> for RejectAll {
        fn keep_pair<S: PairSetView>(&mut self, _gb: &GrobnerBasis<P>, _state: &S, _i: usize, _j: usize) -> bool {
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
        let state = EmptyState;

        let mut both_yes = AndPairFilter::new(NoPairFilter, NoPairFilter);
        assert!(both_yes.keep_pair(&gb, &state, 0, 1));

        let mut left_no = AndPairFilter::new(RejectAll, NoPairFilter);
        assert!(!left_no.keep_pair(&gb, &state, 0, 1));

        let mut right_no = AndPairFilter::new(NoPairFilter, RejectAll);
        assert!(!right_no.keep_pair(&gb, &state, 0, 1));
    }
}
