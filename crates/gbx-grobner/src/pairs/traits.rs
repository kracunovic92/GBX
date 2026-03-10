/// A keyed critical pair.
/// `key` is used for ordering (smaller = higher priority).
pub type Pair = (u32, usize, usize);

/// Pair queue abstraction.
pub trait PairQueue {
    /// Construct an empty pair queue.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns `true` if no pairs remain.
    fn is_empty(&self) -> bool;

    /// Insert a pair.
    fn push(&mut self, pair: Pair);

    /// Pop the next pair to process.
    fn pop(&mut self) -> Option<Pair>;

    /// Returning size of active queue.
    fn len(&self) -> usize;
}
