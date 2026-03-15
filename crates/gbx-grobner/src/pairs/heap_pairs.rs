use crate::pairs::traits::{normalize_pair_indices, Pair, PairQueue};
use crate::PairSetView;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap};

/// Heap entry for critical pairs.
///
/// Ordered so that `BinaryHeap` behaves like a min-heap:
/// 1. smaller key first
/// 2. smaller i first
/// 3. smaller j first
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    key: u32,
    i: usize,
    j: usize,
}

impl HeapItem {
    #[inline]
    fn from_pair((key, i, j): Pair) -> Self {
        Self { key, i, j }
    }

    #[inline]
    fn into_pair(self) -> Pair {
        (self.key, self.i, self.j)
    }

    #[inline]
    fn normalized_indices(self) -> (usize, usize) {
        normalize_pair_indices(self.i, self.j)
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.key.cmp(&self.key) {
            Ordering::Equal => match other.i.cmp(&self.i) {
                Ordering::Equal => other.j.cmp(&self.j),
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
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    #[inline]
    fn push(&mut self, pair: Pair) {
        let item = HeapItem::from_pair(pair);
        self.members.insert(item.normalized_indices());
        self.heap.push(item);
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        let item = self.heap.pop()?;
        self.members.remove(&item.normalized_indices());
        Some(item.into_pair())
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
    use super::{HeapPairs, PairQueue};
    use crate::PairSetView;

    #[test]
    fn heap_pairs_is_min_key_first() {
        let mut q = HeapPairs::new();
        q.push((10, 0, 1));
        q.push((3, 0, 2));
        q.push((7, 1, 2));

        assert_eq!(q.pop(), Some((3, 0, 2)));
        assert_eq!(q.pop(), Some((7, 1, 2)));
        assert_eq!(q.pop(), Some((10, 0, 1)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn heap_pairs_tie_breaks_by_i_then_j() {
        let mut q = HeapPairs::new();
        q.push((5, 2, 3));
        q.push((5, 1, 4));
        q.push((5, 1, 2));

        assert_eq!(q.pop(), Some((5, 1, 2)));
        assert_eq!(q.pop(), Some((5, 1, 4)));
        assert_eq!(q.pop(), Some((5, 2, 3)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn heap_pairs_tracks_membership() {
        let mut q = HeapPairs::new();
        q.push((5, 2, 3));
        q.push((1, 4, 1));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some((1, 4, 1)));
        assert!(!q.contains_pair(1, 4));
        assert!(q.contains_pair(2, 3));
    }
}
