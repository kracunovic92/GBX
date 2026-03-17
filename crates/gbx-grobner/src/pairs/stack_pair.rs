use crate::pairs::traits::{normalize_pair_indices, Pair, PairQueue};
use crate::PairSetView;
use std::collections::{BTreeSet, VecDeque};

/// Default pair queue: LIFO stack.
///
/// Active pair membership is tracked separately so duplicate normalized pairs
/// are not inserted and membership queries are efficient.
#[derive(Debug, Default, Clone)]
pub struct StackPairs {
    pairs: Vec<Pair>,
    members: BTreeSet<(usize, usize)>,
}

impl PairQueue for StackPairs {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    #[inline]
    fn push(&mut self, pair: Pair) {
        let (_, i, j) = pair;
        let normalized = normalize_pair_indices(i, j);

        if self.members.insert(normalized) {
            self.pairs.push(pair);
        }
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        let pair = self.pairs.pop()?;
        let (_, i, j) = pair;
        self.members.remove(&normalize_pair_indices(i, j));
        Some(pair)
    }

    #[inline]
    fn len(&self) -> usize {
        self.pairs.len()
    }
}

impl PairSetView for StackPairs {
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        self.members.contains(&normalize_pair_indices(i, j))
    }
}

/// FIFO pair queue.
///
/// Active pair membership is tracked separately so duplicate normalized pairs
/// are not inserted and membership queries are efficient.
#[derive(Debug, Default, Clone)]
pub struct FifoPairs {
    pairs: VecDeque<Pair>,
    members: BTreeSet<(usize, usize)>,
}

impl PairQueue for FifoPairs {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    #[inline]
    fn push(&mut self, pair: Pair) {
        let (_, i, j) = pair;
        let normalized = normalize_pair_indices(i, j);

        if self.members.insert(normalized) {
            self.pairs.push_back(pair);
        }
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        let pair = self.pairs.pop_front()?;
        let (_, i, j) = pair;
        self.members.remove(&normalize_pair_indices(i, j));
        Some(pair)
    }

    #[inline]
    fn len(&self) -> usize {
        self.pairs.len()
    }
}

impl PairSetView for FifoPairs {
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        self.members.contains(&normalize_pair_indices(i, j))
    }
}

#[cfg(test)]
mod tests {
    use super::{FifoPairs, PairQueue, StackPairs};
    use crate::PairSetView;

    #[test]
    fn stack_pairs_is_lifo() {
        let mut q = StackPairs::new();
        q.push((0, 0, 1));
        q.push((0, 0, 2));
        q.push((0, 1, 2));

        assert_eq!(q.pop(), Some((0, 1, 2)));
        assert_eq!(q.pop(), Some((0, 0, 2)));
        assert_eq!(q.pop(), Some((0, 0, 1)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn fifo_pairs_is_fifo() {
        let mut q = FifoPairs::new();
        q.push((0, 0, 1));
        q.push((0, 0, 2));
        q.push((0, 1, 2));

        assert_eq!(q.pop(), Some((0, 0, 1)));
        assert_eq!(q.pop(), Some((0, 0, 2)));
        assert_eq!(q.pop(), Some((0, 1, 2)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn stack_pairs_tracks_membership() {
        let mut q = StackPairs::new();
        q.push((0, 2, 3));
        q.push((0, 1, 4));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some((0, 1, 4)));
        assert!(!q.contains_pair(1, 4));
        assert!(q.contains_pair(2, 3));
    }

    #[test]
    fn fifo_pairs_tracks_membership() {
        let mut q = FifoPairs::new();
        q.push((0, 2, 3));
        q.push((0, 1, 4));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some((0, 2, 3)));
        assert!(!q.contains_pair(2, 3));
        assert!(q.contains_pair(1, 4));
    }

    #[test]
    fn stack_pairs_deduplicates_normalized_pairs() {
        let mut q = StackPairs::new();
        q.push((0, 2, 3));
        q.push((1, 3, 2));

        assert_eq!(q.len(), 1);
        assert!(q.contains_pair(2, 3));
        assert_eq!(q.pop(), Some((0, 2, 3)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn fifo_pairs_deduplicates_normalized_pairs() {
        let mut q = FifoPairs::new();
        q.push((0, 2, 3));
        q.push((1, 3, 2));

        assert_eq!(q.len(), 1);
        assert!(q.contains_pair(2, 3));
        assert_eq!(q.pop(), Some((0, 2, 3)));
        assert_eq!(q.pop(), None);
    }
}
