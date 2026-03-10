use crate::criteria::PairCriterion;
use crate::GrobnerBasis;

/// Baseline Buchberger pair criterion.
///
/// This criterion performs **no pair elimination** and keeps every candidate
/// pair `(i, j)`.
///
/// # Purpose
///
/// This represents the classical baseline Buchberger algorithm where
/// every S-pair is considered.
///
/// It is useful for:
///
/// - correctness baselines
/// - benchmarking optimized criteria
/// - debugging pair-update logic
///
/// # Behavior
///
/// Always returns `true`, meaning every pair is kept.
#[derive(Debug, Default, Clone, Copy)]
pub struct BaselineCriterion;

impl<P> PairCriterion<P> for BaselineCriterion {
    #[inline]
    fn keep_pair(&mut self, _gb: &GrobnerBasis<P>, _i: usize, _j: usize) -> bool {
        true
    }
}
