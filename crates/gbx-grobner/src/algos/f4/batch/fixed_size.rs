use crate::algos::f4::batch::selector::PairBatchSelector;
use crate::algos::f4::batch::types::{CriticalPair, PairBatch};
use crate::algos::f4::batch::validate::debug_validate_batch;
use crate::pairs::PairQueue;

/// Selects up to a fixed number of critical pairs from the queue.
///
/// Pairs are taken in queue order until either the batch is full or the queue
/// becomes empty.
///
/// # Example
///
/// ```ignore
/// let mut selector = FixedSizeBatchSelector::new(128);
/// let batch = selector.select_batch(&mut queue);
///
/// assert!(batch.len() <= 128);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedSizeBatchSelector {
    batch_size: usize,
}

impl FixedSizeBatchSelector {
    /// Creates a selector with the given batch size.
    ///
    /// # Panics
    ///
    /// Panics if `batch_size == 0`.
    #[must_use]
    pub const fn new(batch_size: usize) -> Self {
        assert!(batch_size > 0, "batch_size must be positive");
        Self { batch_size }
    }

    /// Returns the maximum number of pairs selected in one batch.
    #[must_use]
    pub const fn batch_size(&self) -> usize {
        self.batch_size
    }
}

impl<Q> PairBatchSelector<Q> for FixedSizeBatchSelector
where
    Q: PairQueue,
{
    fn select_batch(&mut self, queue: &mut Q) -> PairBatch<Q::Key> {
        let mut pairs = Vec::with_capacity(self.batch_size);

        for _ in 0..self.batch_size {
            let Some(pair) = queue.pop() else {
                break;
            };

            debug_assert!(pair.i != pair.j, "critical pair must satisfy i != j");
            pairs.push(CriticalPair::new(pair.key, pair.i, pair.j));
        }

        let batch = PairBatch::new(pairs);
        debug_validate_batch(&batch);
        batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pairs::{FifoPairs, HeapPairs, Pair, PairQueue, StackPairs};

    fn collect_indices<K>(batch: &PairBatch<K>) -> Vec<(usize, usize)> {
        batch.iter().map(|p| (p.i, p.j)).collect()
    }

    #[test]
    fn fixed_size_returns_empty_for_empty_fifo_queue() {
        let mut queue = FifoPairs::new();
        let mut selector = FixedSizeBatchSelector::new(3);

        let batch = selector.select_batch(&mut queue);

        assert!(batch.is_empty());
    }

    #[test]
    fn fixed_size_takes_all_when_queue_has_fewer_pairs() {
        let mut queue = FifoPairs::new();
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(20, 0, 2));

        let mut selector = FixedSizeBatchSelector::new(4);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert_eq!(collect_indices(&batch), vec![(0, 1), (0, 2)]);
    }

    #[test]
    fn fixed_size_respects_fifo_order() {
        let mut queue = FifoPairs::new();
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(20, 0, 2));
        queue.push(Pair::new(30, 1, 2));

        let mut selector = FixedSizeBatchSelector::new(2);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert_eq!(batch.as_slice()[0].key, 10);
        assert_eq!(batch.as_slice()[1].key, 20);
        assert_eq!(collect_indices(&batch), vec![(0, 1), (0, 2)]);

        let remaining = queue.pop().unwrap();
        assert_eq!(remaining.key, 30);
        assert_eq!((remaining.i, remaining.j), (1, 2));
    }

    #[test]
    fn fixed_size_respects_stack_order() {
        let mut queue = StackPairs::new();
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(20, 0, 2));
        queue.push(Pair::new(30, 1, 2));

        let mut selector = FixedSizeBatchSelector::new(2);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert_eq!(batch.as_slice()[0].key, 30);
        assert_eq!(batch.as_slice()[1].key, 20);
        assert_eq!(collect_indices(&batch), vec![(1, 2), (0, 2)]);
    }

    #[test]
    fn fixed_size_respects_heap_priority() {
        let mut queue = HeapPairs::new();
        queue.push(Pair::new(30, 1, 2));
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(20, 0, 2));

        let mut selector = FixedSizeBatchSelector::new(3);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 3);
        assert_eq!(batch.as_slice()[0].key, 10);
        assert_eq!(batch.as_slice()[1].key, 20);
        assert_eq!(batch.as_slice()[2].key, 30);
    }

    #[test]
    #[should_panic]
    fn fixed_size_new_panics_on_zero() {
        let _ = FixedSizeBatchSelector::new(0);
    }
}
