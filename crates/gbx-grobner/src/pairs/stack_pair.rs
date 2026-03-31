//! Linear critical-pair queues.
//!
//! This module provides two simple queue implementations:
//!
//! - [`StackPairs`]: LIFO order
//! - [`FifoPairs`]: FIFO order
//!
//! Both track active membership using normalized pair endpoints so duplicate
//! unordered pairs are not inserted.

//! Linear critical-pair queues.
//!
//! This module provides two queue implementations:
//!
//! - [`StackPairs`]: last-in, first-out order
//! - [`FifoPairs`]: first-in, first-out order

use crate::pairs::{normalize_pair_indices, Pair, PairQueue};
use crate::PairSetView;
use std::collections::{BTreeSet, VecDeque};

/// Critical-pair queue with last-in, first-out order.
///
/// # Example
///
/// ```ignore
/// let mut queue = StackPairs::new();
/// queue.push(Pair::new(3, 0, 1));
/// queue.push(Pair::new(2, 0, 2));
///
/// assert_eq!(queue.pop(), Some(Pair::new(2, 0, 2)));
/// ```
#[derive(Debug, Default, Clone)]
pub struct StackPairs {
    pairs: Vec<Pair<u32>>,
    members: BTreeSet<(usize, usize)>,
}

impl StackPairs {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PairQueue for StackPairs {
    type Key = u32;

    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn push(&mut self, pair: Pair<Self::Key>) {
        let normalized = normalize_pair_indices(pair.i, pair.j);

        if self.members.insert(normalized) {
            self.pairs.push(pair);
        }
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair<Self::Key>> {
        let pair = self.pairs.pop()?;
        self.members.remove(&normalize_pair_indices(pair.i, pair.j));
        Some(pair)
    }

    #[inline]
    fn peek(&self) -> Option<&Pair<Self::Key>> {
        self.pairs.last()
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

/// Critical-pair queue with first-in, first-out order.
///
/// # Example
///
/// ```ignore
/// let mut queue = FifoPairs::new();
/// queue.push(Pair::new(3, 0, 1));
/// queue.push(Pair::new(2, 0, 2));
///
/// assert_eq!(queue.pop(), Some(Pair::new(3, 0, 1)));
/// ```
#[derive(Debug, Default, Clone)]
pub struct FifoPairs {
    pairs: VecDeque<Pair<u32>>,
    members: BTreeSet<(usize, usize)>,
}

impl FifoPairs {
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl PairQueue for FifoPairs {
    type Key = u32;

    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn push(&mut self, pair: Pair<Self::Key>) {
        let normalized = normalize_pair_indices(pair.i, pair.j);

        if self.members.insert(normalized) {
            self.pairs.push_back(pair);
        }
    }

    #[inline]
    fn pop(&mut self) -> Option<Pair<Self::Key>> {
        let pair = self.pairs.pop_front()?;
        self.members.remove(&normalize_pair_indices(pair.i, pair.j));
        Some(pair)
    }

    #[inline]
    fn peek(&self) -> Option<&Pair<Self::Key>> {
        self.pairs.front()
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
    use super::{FifoPairs, StackPairs};
    use crate::pairs::{Pair, PairQueue};
    use crate::PairSetView;

    #[test]
    fn stack_pairs_is_lifo() {
        let mut q = StackPairs::new();
        q.push(Pair::new(0, 0, 1));
        q.push(Pair::new(0, 0, 2));
        q.push(Pair::new(0, 1, 2));

        assert_eq!(q.pop(), Some(Pair::new(0, 1, 2)));
        assert_eq!(q.pop(), Some(Pair::new(0, 0, 2)));
        assert_eq!(q.pop(), Some(Pair::new(0, 0, 1)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn fifo_pairs_is_fifo() {
        let mut q = FifoPairs::new();
        q.push(Pair::new(0, 0, 1));
        q.push(Pair::new(0, 0, 2));
        q.push(Pair::new(0, 1, 2));

        assert_eq!(q.pop(), Some(Pair::new(0, 0, 1)));
        assert_eq!(q.pop(), Some(Pair::new(0, 0, 2)));
        assert_eq!(q.pop(), Some(Pair::new(0, 1, 2)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn stack_pairs_tracks_membership() {
        let mut q = StackPairs::new();
        q.push(Pair::new(0, 2, 3));
        q.push(Pair::new(0, 1, 4));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some(Pair::new(0, 1, 4)));
        assert!(!q.contains_pair(1, 4));
        assert!(q.contains_pair(2, 3));
    }

    #[test]
    fn fifo_pairs_tracks_membership() {
        let mut q = FifoPairs::new();
        q.push(Pair::new(0, 2, 3));
        q.push(Pair::new(0, 1, 4));

        assert!(q.contains_pair(2, 3));
        assert!(q.contains_pair(3, 2));
        assert!(q.contains_pair(1, 4));
        assert!(q.contains_pair(4, 1));
        assert!(!q.contains_pair(0, 1));

        assert_eq!(q.pop(), Some(Pair::new(0, 2, 3)));
        assert!(!q.contains_pair(2, 3));
        assert!(q.contains_pair(1, 4));
    }

    #[test]
    fn stack_pairs_deduplicates_normalized_pairs() {
        let mut q = StackPairs::new();
        q.push(Pair::new(0, 2, 3));
        q.push(Pair::new(1, 3, 2));

        assert_eq!(q.len(), 1);
        assert!(q.contains_pair(2, 3));
        assert_eq!(q.pop(), Some(Pair::new(0, 2, 3)));
        assert_eq!(q.pop(), None);
    }

    #[test]
    fn fifo_pairs_deduplicates_normalized_pairs() {
        let mut q = FifoPairs::new();
        q.push(Pair::new(0, 2, 3));
        q.push(Pair::new(1, 3, 2));

        assert_eq!(q.len(), 1);
        assert!(q.contains_pair(2, 3));
        assert_eq!(q.pop(), Some(Pair::new(0, 2, 3)));
        assert_eq!(q.pop(), None);
    }
}
