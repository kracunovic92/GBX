use crate::algos::f4::pairs::critical_pair::CriticalPair;

/// Result of selecting one F4 batch from pending critical pairs.
#[derive(Debug, Clone)]
pub struct Selection<M> {
    pub selected: Vec<CriticalPair<M>>,
    pub remaining: Vec<CriticalPair<M>>,
}

/// Strategy for choosing the next batch of critical pairs.
pub trait PairSelector<M> {
    fn select(&mut self, pairs: Vec<CriticalPair<M>>) -> Selection<M>;
}

/// Select all critical pairs of minimal total degree, optionally capped by batch size.
#[derive(Debug, Clone, Copy)]
pub struct MinDegreeSelector {
    batch_size: usize,
}

impl MinDegreeSelector {
    #[must_use]
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }
}

impl Default for MinDegreeSelector {
    fn default() -> Self {
        Self { batch_size: usize::MAX }
    }
}

impl<M> PairSelector<M> for MinDegreeSelector {
    fn select(&mut self, pairs: Vec<CriticalPair<M>>) -> Selection<M> {
        if pairs.is_empty() {
            return Selection { selected: Vec::new(), remaining: Vec::new() };
        }

        let min_degree = pairs
            .iter()
            .map(|p| p.degree)
            .min()
            .expect("pairs is non-empty");

        let mut selected = Vec::new();
        let mut remaining = Vec::new();

        for pair in pairs {
            if pair.degree == min_degree && selected.len() < self.batch_size {
                selected.push(pair);
            } else {
                remaining.push(pair);
            }
        }

        Selection { selected, remaining }
    }
}
