//! Heap-based critical-pair queue.
//!
//! This queue orders pairs by ascending key, then ascending `i`, then ascending
//! `j`. Since Rust's [`BinaryHeap`] is a max-heap, the internal ordering is
//! reversed so the smallest key is popped first.
//!
//! Active pair membership is tracked separately using normalized endpoints, so
//! duplicate unordered pairs are not inserted.

use crate::pairs::{normalize_pair_indices, Pair, PairQueue};
use crate::PairSetView;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

/// Heap entry for critical pairs.
///
/// Ordered so that [`BinaryHeap`] behaves like a min-heap:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem(Pair<u32>);

impl HeapItem {
    #[inline]
    fn normalized_indices(self) -> (usize, usize) {
        normalize_pair_indices(self.0.i, self.0.j)
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.0.key.cmp(&self.0.key) {
            Ordering::Equal => match other.0.i.cmp(&self.0.i) {
                Ordering::Equal => other.0.j.cmp(&self.0.j),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Priority queue of pairs by key (smallest key first).
///
/// Active pair membership is tracked separately so that:
///
/// - `contains_pair(i, j)` is efficient,
/// - duplicate normalized pairs are not inserted,
/// - pair-state-aware criteria see a consistent pending-pair set.
#[derive(Debug, Default, Clone)]
pub struct HeapPairs {
    heap: BinaryHeap<HeapItem>,
    members: BTreeSet<(usize, usize)>,
}

impl HeapPairs {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PairQueue for HeapPairs {
    type Key = u32;

    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn push(&mut self, pair: Pair<Self::Key>) {
        let item = HeapItem(pair);
        let normalized = item.normalized_indices();

        if self.members.insert(normalized) {
            self.heap.push(item);
        }
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair<Self::Key>> {
        let item = self.heap.pop()?;
        self.members.remove(&item.normalized_indices());
        Some(item.0)
    }

    #[inline]
    fn peek(&self) -> Option<&Pair<Self::Key>> {
        self.heap.peek().map(|item| &item.0)
    }

    #[inline]
    fn len(&self) -> usize {
        self.heap.len()
    }
}

impl PairSetView for HeapPairs {
    #[inline]
    fn contains_pair(&self, i: usize, j: usize) -> bool {
        self.members.contains(&normalize_pair_indices(i, j))
    }
}

#[cfg(test)]
mod tests {
    use super::HeapPairs;
    use crate::pairs::{Pair, PairQueue};
    use crate::PairSetView;

    #[test]
    fn heap_pairs_is_min_key_first() {
        let mut q = HeapPairs::new();
        q.push(Pair::new(10, 0, 1));
        q.push(Pair::new(3, 0, 2));
        q.push(Pair::new(7, 1, 2));

        assert_eq!(q.pop(), Some(Pair::new(3, 0, 2)));
        assert_eq!(q.pop(), Some(Pair::new(7, 1, 2)));
        assert_eq!(q.pop(), Some(Pair::new(10, 0, 1)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn heap_pairs_tie_breaks_by_i_then_j() {
        let mut q = HeapPairs::new();
        q.push(Pair::new(5, 2, 3));
        q.push(Pair::new(5, 1, 4));
        q.push(Pair::new(5, 1, 2));

        assert_eq!(q.pop(), Some(Pair::new(5, 1, 2)));
        assert_eq!(q.pop(), Some(Pair::new(5, 1, 4)));
        assert_eq!(q.pop(), Some(Pair::new(5, 2, 3)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn heap_pairs_tracks_membership() {
        let mut q = HeapPairs::new();
        q.push(Pair::new(5, 2, 3));
        q.push(Pair::new(1, 4, 1));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some(Pair::new(1, 4, 1)));
        assert!(!q.contains_pair(1, 4));
        assert!(q.contains_pair(2, 3));
    }

    #[test]
    fn heap_pairs_deduplicates_normalized_pairs() {
        let mut q = HeapPairs::new();
        q.push(Pair::new(5, 2, 3));
        q.push(Pair::new(1, 3, 2)); // same normalized pair, ignored

        assert_eq!(q.len(), 1);
        assert!(q.contains_pair(2, 3));
        assert_eq!(q.pop(), Some(Pair::new(5, 2, 3)));
        assert_eq!(q.pop(), None);
    }
}
