use crate::algos::f4::batch::selector::PairBatchSelector;
use crate::algos::f4::batch::types::{CriticalPair, PairBatch};
use crate::algos::f4::batch::validate::debug_validate_batch;
use crate::pairs::PairQueue;

/// Selects up to a fixed number of critical pairs with the same key.
///
/// The first selected pair determines the key of the batch. Additional pairs
/// are taken while they have the same key and the batch limit is not reached.
///
/// # Example
///
/// ```ignore
/// let mut selector = SameKeyBatchSelector::new(64);
/// let batch = selector.select_batch(&mut queue);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SameKeyBatchSelector {
    batch_size: usize,
}

impl SameKeyBatchSelector {
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

impl<Q> PairBatchSelector<Q> for SameKeyBatchSelector
where
    Q: PairQueue,
    Q::Key: PartialEq,
{
    fn select_batch(&mut self, queue: &mut Q) -> PairBatch<Q::Key> {
        let Some(first) = queue.pop() else {
            return PairBatch::empty();
        };

        debug_assert!(first.i != first.j, "critical pair must satisfy i != j");

        let mut pairs = Vec::with_capacity(self.batch_size);
        pairs.push(CriticalPair::new(first.key, first.i, first.j));

        while pairs.len() < self.batch_size {
            let Some(next) = queue.peek() else {
                break;
            };

            if next.key != pairs[0].key {
                break;
            }

            let next = queue.pop().expect("peeked pair must remain available");
            debug_assert!(next.i != next.j, "critical pair must satisfy i != j");
            pairs.push(CriticalPair::new(next.key, next.i, next.j));
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

    #[test]
    fn returns_empty_batch_for_empty_fifo_queue() {
        let mut queue = FifoPairs::new();
        let mut selector = SameKeyBatchSelector::new(4);

        let batch = selector.select_batch(&mut queue);

        assert!(batch.is_empty());
    }

    #[test]
    fn takes_single_pair_when_queue_has_one_pair() {
        let mut queue = FifoPairs::new();
        queue.push(Pair::new(10, 0, 1));

        let mut selector = SameKeyBatchSelector::new(4);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 1);
        assert_eq!(batch.as_slice()[0], CriticalPair::new(10, 0, 1));
        assert!(queue.is_empty());
    }

    #[test]
    fn fifo_takes_only_initial_same_key_group() {
        let mut queue = FifoPairs::new();
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(10, 0, 2));
        queue.push(Pair::new(20, 1, 2));

        let mut selector = SameKeyBatchSelector::new(10);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert!(batch.iter().all(|pair| pair.key == 10));
        assert_eq!(batch.as_slice()[0], CriticalPair::new(10, 0, 1));
        assert_eq!(batch.as_slice()[1], CriticalPair::new(10, 0, 2));

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek(), Some(Pair::new(20, 1, 2)).as_ref());
    }

    #[test]
    fn stack_takes_only_top_same_key_group() {
        let mut queue = StackPairs::new();
        queue.push(Pair::new(30, 2, 3));
        queue.push(Pair::new(20, 1, 3));
        queue.push(Pair::new(20, 1, 2));
        queue.push(Pair::new(10, 0, 1));

        let mut selector = SameKeyBatchSelector::new(10);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 1);
        assert_eq!(batch.as_slice()[0], CriticalPair::new(10, 0, 1));

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(Pair::new(20, 1, 2)).as_ref());
    }

    #[test]
    fn heap_groups_smallest_key_pairs() {
        let mut queue = HeapPairs::new();
        queue.push(Pair::new(20, 1, 2));
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(10, 0, 2));
        queue.push(Pair::new(30, 2, 3));

        let mut selector = SameKeyBatchSelector::new(10);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert!(batch.iter().all(|pair| pair.key == 10));
        assert_eq!(batch.as_slice()[0], CriticalPair::new(10, 0, 1));
        assert_eq!(batch.as_slice()[1], CriticalPair::new(10, 0, 2));

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(Pair::new(20, 1, 2)).as_ref());
    }

    #[test]
    fn respects_batch_size_cap() {
        let mut queue = FifoPairs::new();
        queue.push(Pair::new(10, 0, 1));
        queue.push(Pair::new(10, 0, 2));
        queue.push(Pair::new(10, 1, 2));
        queue.push(Pair::new(20, 2, 3));

        let mut selector = SameKeyBatchSelector::new(2);
        let batch = selector.select_batch(&mut queue);

        assert_eq!(batch.len(), 2);
        assert!(batch.iter().all(|pair| pair.key == 10));
        assert_eq!(batch.as_slice()[0], CriticalPair::new(10, 0, 1));
        assert_eq!(batch.as_slice()[1], CriticalPair::new(10, 0, 2));

        assert_eq!(queue.len(), 2);
        assert_eq!(queue.peek(), Some(Pair::new(10, 1, 2)).as_ref());
    }

    #[test]
    #[should_panic]
    fn new_panics_on_zero_batch_size() {
        let _ = SameKeyBatchSelector::new(0);
    }
}
