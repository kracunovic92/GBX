use crate::pairs::traits::{Pair, PairQueue};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
        self.heap.push(HeapItem::from_pair(pair));
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair> {
        self.heap.pop().map(HeapItem::into_pair)
    }

    #[inline]
    fn len(&self) -> usize {
        self.heap.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{HeapPairs, PairQueue};

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
}
