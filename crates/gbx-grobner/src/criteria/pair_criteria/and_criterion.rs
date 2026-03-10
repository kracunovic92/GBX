use crate::criteria::PairCriterion;
use crate::GrobnerBasis;

/// Logical AND of two pair criteria.
///
/// The pair `(i, j)` is kept if and only if **both** inner criteria keep it.
///
/// # Purpose
///
/// This combinator allows simple composition of local pair criteria without
/// creating a dedicated concrete type for every combination.
///
/// # Examples
///
/// A combined criterion might keep a pair only if:
///
/// - Buchberger's product criterion keeps it, and
/// - some additional experimental local heuristic also keeps it
///
#[derive(Debug, Default, Clone, Copy)]
pub struct AndCriterion<A, B> {
    pub left: A,
    pub right: B,
}

impl<A, B> AndCriterion<A, B> {
    /// Creates a new conjunction of two pair criteria.
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
