use std::time::Duration;

/// Fine-grained timing buckets within Buchberger's main loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhileKind {
    SPolynomial,
    NormalForm,
    RemainderNormalize,
    PairUpdate,
    PairFilter,
}

/// Wall-clock totals for work performed inside Buchberger's main loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct WhileTimes {
    pub s_poly: Duration,
    pub normal_form: Duration,
    pub remainder_normalize: Duration,
    pub pair_update: Duration,
    pub pair_filter: Duration,
}

impl WhileTimes {
    #[inline]
    pub fn add(&mut self, kind: WhileKind, dt: Duration) {
        match kind {
            WhileKind::SPolynomial => self.s_poly += dt,
            WhileKind::NormalForm => self.normal_form += dt,
            WhileKind::RemainderNormalize => self.remainder_normalize += dt,
            WhileKind::PairUpdate => self.pair_update += dt,
            WhileKind::PairFilter => self.pair_filter += dt,
        }
    }
}
