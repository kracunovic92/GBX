use crate::criteria::PairKey;
use crate::GrobnerBasis;

/// Constant key `0` for all pairs.
///
/// This key does not distinguish between pairs.
/// It is useful for simple queue behaviors where pair ordering is delegated
/// entirely to the queue implementation itself.
///
/// # Typical use
///
/// - FIFO-like behavior with a plain queue
/// - LIFO-like behavior with a stack-based queue
/// - debugging / baseline comparisons where pair ordering should be neutral
///
/// # Notes
///
/// Since this key never fails, it always returns `Some(0)`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ZeroPairKey;

impl<P> PairKey<P> for ZeroPairKey {
    #[inline]
    fn key_for_pair(&mut self, _gb: &GrobnerBasis<P>, _i: usize, _j: usize) -> Option<u32> {
        Some(0)
    }
}
