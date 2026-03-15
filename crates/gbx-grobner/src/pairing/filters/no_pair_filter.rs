use crate::pairing::filters::{PairFilter, PairSetView};
use crate::GrobnerBasis;

/// Filter that performs no pair elimination.
///
/// This filter keeps every candidate pair.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoPairFilter;

impl<P> PairFilter<P> for NoPairFilter {
    #[inline]
    fn keep_pair<S: PairSetView>(&mut self, _gb: &GrobnerBasis<P>, _state: &S, _i: usize, _j: usize) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairing::filters::PairSetView;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[derive(Debug, Default)]
    struct EmptyState;

    impl PairSetView for EmptyState {
        fn contains_pair(&self, _i: usize, _j: usize) -> bool {
            false
        }
    }

    #[test]
    fn always_keeps_pairs() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let gb = crate::GrobnerBasis::new(ring.id(), Vec::<()>::new());
        let state = EmptyState;
        let mut f = NoPairFilter;

        assert!(f.keep_pair(&gb, &state, 0, 1));
        assert!(f.keep_pair(&gb, &state, 10, 20));
    }
}
