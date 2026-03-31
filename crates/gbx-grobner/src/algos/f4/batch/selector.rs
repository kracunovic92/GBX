use crate::algos::f4::batch::types::PairBatch;
use crate::pairs::PairQueue;

/// Selects a batch of critical pairs from a pair queue.
///
/// A selector decides which pairs are taken from the queue to form the next
/// batch used by the F4 algorithm.
///
/// # Example
///
/// ```ignore
/// let mut selector = FixedSizeBatchSelector::new(100);
///
/// let batch = selector.select_batch(&mut queue);
///
/// for pair in &batch {
///     println!("pair: ({}, {})", pair.i, pair.j);
/// }
/// ```
pub trait PairBatchSelector<Q>
where
    Q: PairQueue,
{
    /// Removes pairs from `queue` and returns them as a batch.
    fn select_batch(&mut self, queue: &mut Q) -> PairBatch<Q::Key>;
}
