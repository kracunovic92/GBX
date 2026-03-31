use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GmUpdatePhase {
    Scan,
    Dedup,
    Prune,
    Push,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GmProductLcmDegreeUpdateCounters {
    pub update_calls: u64,

    pub candidates_considered: u64,
    pub rejected_missing_lm: u64,
    pub rejected_by_product: u64,
    pub rejected_lcm_failure: u64,
    pub rejected_key_failure: u64,

    pub dedup_collisions: u64,
    pub survivors_after_dedup: u64,
    pub survivors_after_prune: u64,
    pub pairs_pushed: u64,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GmProductLcmDegreeUpdateTimes {
    pub scan: Duration,
    pub dedup: Duration,
    pub prune: Duration,
    pub push: Duration,
}

impl GmProductLcmDegreeUpdateTimes {
    #[inline]
    pub fn add(&mut self, phase: GmUpdatePhase, dt: Duration) {
        match phase {
            GmUpdatePhase::Scan => self.scan += dt,
            GmUpdatePhase::Dedup => self.dedup += dt,
            GmUpdatePhase::Prune => self.prune += dt,
            GmUpdatePhase::Push => self.push += dt,
        }
    }

    #[must_use]
    #[inline]
    pub fn total(&self) -> Duration {
        self.scan + self.dedup + self.prune + self.push
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GmProductLcmDegreeUpdateTrace {
    pub counters: GmProductLcmDegreeUpdateCounters,
    pub times: GmProductLcmDegreeUpdateTimes,
}
