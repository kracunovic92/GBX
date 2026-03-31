/// A selected critical pair `(i, j)` together with its queue key.
///
/// The `key` is whatever ordering / priority information the queue uses.
/// The batch layer treats it as opaque metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriticalPair<K> {
    pub key: K,
    pub i: usize,
    pub j: usize,
}

impl<K> CriticalPair<K> {
    #[must_use]
    pub const fn new(key: K, i: usize, j: usize) -> Self {
        Self { key, i, j }
    }
}

/// A batch of selected critical pairs.
///
/// This type intentionally wraps the raw vector so later we can:
/// - attach metadata,
/// - strengthen invariants,
/// - change internal representation if needed.
#[derive(Debug, Clone)]
pub struct PairBatch<K> {
    pairs: Vec<CriticalPair<K>>,
}

impl<K> PairBatch<K> {
    #[must_use]
    pub fn new(pairs: Vec<CriticalPair<K>>) -> Self {
        Self { pairs }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self { pairs: Vec::new() }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    #[must_use]
    pub fn as_slice(&self) -> &[CriticalPair<K>] {
        &self.pairs
    }

    #[must_use]
    pub fn iter(&self) -> core::slice::Iter<'_, CriticalPair<K>> {
        self.pairs.iter()
    }

    pub fn into_vec(self) -> Vec<CriticalPair<K>> {
        self.pairs
    }
}

impl<K> Default for PairBatch<K> {
    fn default() -> Self {
        Self::empty()
    }
}
