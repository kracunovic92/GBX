use crate::pairs::traits::{normalize_pair_indices, Pair, PairQueue};
use crate::PairSetView;
use std::collections::VecDeque;

/// Default pair queue: LIFO stack.
#[derive(Debug, Default, Clone)]
pub struct StackPairs(Vec<Pair>);

impl PairQueue for StackPairs {
    #[inline]
    fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn push(&mut self, pair: Pair) {
        self.0.push(pair);
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        self.0.pop()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl PairSetView for StackPairs {
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        let want = normalize_pair_indices(i, j);
        self.0
            .iter()
            .any(|&(_, a, b)| normalize_pair_indices(a, b) == want)
    }
}

/// FIFO pair queue.
#[derive(Debug, Default, Clone)]
pub struct FifoPairs(VecDeque<Pair>);

impl PairQueue for FifoPairs {
    #[inline]
    fn new() -> Self {
        Self(VecDeque::new())
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn push(&mut self, p: Pair) {
        self.0.push_back(p);
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        self.0.pop_front()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl PairSetView for FifoPairs {
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        let want = normalize_pair_indices(i, j);
        self.0
            .iter()
            .any(|&(_, a, b)| normalize_pair_indices(a, b) == want)
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
}
