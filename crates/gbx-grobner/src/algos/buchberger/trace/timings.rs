use std::time::{Duration, Instant};

/// Top-level Buchberger phase kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    Init,
    Seed,
    WhileLoop,
    Post,
}

/// Fine-grained timing buckets within Buchberger's main loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhileKind {
    SPolynomial,
    NormalForm,
    RemainderNormalize,
    PairUpdate,
}

/// Wall-clock totals for Buchberger's main phases.
#[derive(Debug, Default, Clone, Copy)]
pub struct PhaseTimes {
    pub init: Duration,
    pub seed: Duration,
    pub while_loop: Duration,
    pub post: Duration,
}

impl PhaseTimes {
    #[inline]
    pub fn add(&mut self, kind: PhaseKind, dt: Duration) {
        match kind {
            PhaseKind::Init => self.init += dt,
            PhaseKind::Seed => self.seed += dt,
            PhaseKind::WhileLoop => self.while_loop += dt,
            PhaseKind::Post => self.post += dt,
        }
    }
}

/// Wall-clock totals for work performed inside Buchberger's main loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct WhileTimes {
    pub s_poly: Duration,
    pub normal_form: Duration,
    pub remainder_normalize: Duration,
    pub pair_update: Duration,
}

impl WhileTimes {
    #[inline]
    pub fn add(&mut self, kind: WhileKind, dt: Duration) {
        match kind {
            WhileKind::SPolynomial => self.s_poly += dt,
            WhileKind::NormalForm => self.normal_form += dt,
            WhileKind::RemainderNormalize => self.remainder_normalize += dt,
            WhileKind::PairUpdate => self.pair_update += dt,
        }
    }
}

/// Measures the duration of `f`, adds it to `out`, and returns the result.
#[inline]
pub fn measure_duration<T, F>(out: &mut Duration, f: F) -> T
where
    F: FnOnce() -> T,
{
    let t0 = Instant::now();
    let value = f();
    *out += t0.elapsed();
    value
}
