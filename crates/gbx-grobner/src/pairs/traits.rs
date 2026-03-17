/// A keyed critical pair.
/// `key` is used for ordering (smaller = higher priority).
pub type Pair = (u32, usize, usize);

/// Normalize an unordered pair of indices so that `(i, j)` and `(j, i)`
/// are treated identically.
///
/// This helper assumes pair endpoints are distinct. Callers should avoid
/// constructing self-pairs `(i, i)`.
#[inline]
#[must_use]
pub const fn normalize_pair_indices(i: usize, j: usize) -> (usize, usize) {
    if i < j { (i, j) } else { (j, i) }
}

/// Pair queue abstraction.
///
/// Implementations are expected to treat pairs as unordered for membership
/// purposes and should avoid storing duplicate normalized pairs.
pub trait PairQueue {
    /// Construct an empty pair queue.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns `true` if no pairs remain.
    fn is_empty(&self) -> bool;

    /// Insert a pair if it is not already active.
    fn push(&mut self, pair: Pair);

    /// Pop the next pair to process.
    fn pop(&mut self) -> Option<Pair>;

    /// Return the number of active pairs currently stored.
    fn len(&self) -> usize;
}
