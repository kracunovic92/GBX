/// A keyed critical pair.
///
/// The `key` determines queue priority according to the queue policy.
/// Smaller keys are typically treated as higher priority, but the exact
/// ordering semantics are determined by the queue implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pair<K> {
    pub key: K,
    pub i: usize,
    pub j: usize,
}

impl<K> Pair<K> {
    /// Construct a keyed critical pair.
    ///
    /// This does not reorder the endpoints. If normalized endpoint order is
    /// required, use [`Pair::new_normalized`].
    #[must_use]
    pub const fn new(key: K, i: usize, j: usize) -> Self {
        Self { key, i, j }
    }

    /// Construct a keyed critical pair with normalized endpoints.
    ///
    /// The returned pair satisfies `i <= j`.
    #[must_use]
    pub fn new_normalized(key: K, i: usize, j: usize) -> Self {
        let (i, j) = normalize_pair_indices(i, j);
        Self { key, i, j }
    }

    /// Return the endpoints as a normalized unordered pair.
    #[must_use]
    pub const fn normalized_indices(&self) -> (usize, usize) {
        normalize_pair_indices(self.i, self.j)
    }
}

/// Normalize an unordered pair of indices so that `(i, j)` and `(j, i)`
/// have the same canonical representation.
///
/// The returned pair satisfies `i <= j`.
///
/// This helper does not reject self-pairs. Callers are responsible for
/// avoiding invalid pairs of the form `(i, i)` when required by the algorithm.
#[inline]
#[must_use]
pub const fn normalize_pair_indices(i: usize, j: usize) -> (usize, usize) {
    if i < j { (i, j) } else { (j, i) }
}
