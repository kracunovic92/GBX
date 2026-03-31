use std::time::Duration;

/// Fine-grained timing buckets within F4's main loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhileKind {
    BatchSelect,
    SeedRows,
    Symbolic,
    MatrixBuild,
    RowReduce,
    Extract,
    PairUpdate,
}

/// Wall-clock totals for work performed inside F4's main loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct WhileTimes {
    pub batch_select: Duration,
    pub seed_rows: Duration,
    pub symbolic: Duration,
    pub matrix_build: Duration,
    pub row_reduce: Duration,
    pub extract: Duration,
    pub pair_update: Duration,
}

impl WhileTimes {
    #[inline]
    pub fn add(&mut self, kind: WhileKind, dt: Duration) {
        match kind {
            WhileKind::BatchSelect => self.batch_select += dt,
            WhileKind::SeedRows => self.seed_rows += dt,
            WhileKind::Symbolic => self.symbolic += dt,
            WhileKind::MatrixBuild => self.matrix_build += dt,
            WhileKind::RowReduce => self.row_reduce += dt,
            WhileKind::Extract => self.extract += dt,
            WhileKind::PairUpdate => self.pair_update += dt,
        }
    }

    #[must_use]
    #[inline]
    pub fn total(&self) -> Duration {
        self.batch_select + self.seed_rows + self.symbolic + self.matrix_build + self.row_reduce + self.extract + self.pair_update
    }
}
