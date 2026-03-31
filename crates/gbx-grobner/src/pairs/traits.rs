use crate::pairs::pairs::Pair;

/// Queue of active critical pairs.
///
/// Implementations are expected to treat pair endpoints as unordered for
/// membership purposes. In particular, `(i, j)` and `(j, i)` should refer
/// to the same logical critical pair.
///
/// Implementations should avoid storing duplicate active pairs with the same
/// normalized endpoints.
pub trait PairQueue {
    /// Key used to prioritize pairs in the queue.
    type Key;

    /// Construct an empty pair queue.
    fn new() -> Self
    where
        Self: Sized;

    /// Returns `true` if the queue is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert a pair if it is not already active.
    ///
    /// Queue implementations may normalize endpoints internally before
    /// checking membership.
    fn push(&mut self, pair: Pair<Self::Key>);

    /// Returns the next critical pair without removing it.
    fn peek(&self) -> Option<&Pair<Self::Key>>;

    /// Pop the next pair to process.
    fn pop(&mut self) -> Option<Pair<Self::Key>>;

    /// Return the number of active pairs currently stored.
    fn len(&self) -> usize;
}
