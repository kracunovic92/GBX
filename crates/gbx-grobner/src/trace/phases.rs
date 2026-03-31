use std::time::Duration;

/// Top-level algorithm phases shared by Gröbner engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorePhaseKind {
    Init,
    Seed,
    WhileLoop,
    Post,
}

/// Wall-clock totals for top-level algorithm phases.
#[derive(Debug, Default, Clone, Copy)]
pub struct CorePhaseTimes {
    pub init: Duration,
    pub seed: Duration,
    pub while_loop: Duration,
    pub post: Duration,
}

impl CorePhaseTimes {
    #[inline]
    pub fn add(&mut self, kind: CorePhaseKind, dt: Duration) {
        match kind {
            CorePhaseKind::Init => self.init += dt,
            CorePhaseKind::Seed => self.seed += dt,
            CorePhaseKind::WhileLoop => self.while_loop += dt,
            CorePhaseKind::Post => self.post += dt,
        }
    }
}
