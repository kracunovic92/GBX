use crate::pairing::PairCriterion;
use crate::GrobnerBasis;

/// Criterion that performs no pair elimination.
///
/// This criterion keeps every candidate pair `(i, j)`.
///
/// # Purpose
///
/// This is useful for:
///
/// - correctness baselines
/// - benchmarking optimized pairing
/// - debugging pair-update logic
///
/// # Behavior
///
/// Always returns `true`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoPairCriterion;

impl<P> PairCriterion<P> for NoPairCriterion {
    #[inline]
    fn keep_pair(&mut self, _gb: &GrobnerBasis<P>, _i: usize, _j: usize) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gbx_poly::order::Lex;
    use gbx_poly::ring::{Ring, StaticFpCtx};

    #[test]
    fn always_keeps_pairs() {
        let ring = Ring::builder()
            .field(StaticFpCtx::<7>::new())
            .order(Lex)
            .nvars(2)
            .build()
            .unwrap();

        let mut c = NoPairCriterion;
        let gb = GrobnerBasis::new(ring.id(), Vec::<()>::new());

        assert!(c.keep_pair(&gb, 0, 1));
        assert!(c.keep_pair(&gb, 10, 20));
    }
}
