use std::time::{Duration, Instant};

/// Wall-clock totals for Buchberger's main phases.
#[derive(Debug, Default, Clone, Copy)]
pub struct PhaseTimes {
    pub init: Duration,
    pub seed: Duration,
    pub while_loop: Duration,
    pub post: Duration,
}

/// Wall-clock totals for work performed inside Buchberger's main loop.
#[derive(Debug, Default, Clone, Copy)]
pub struct WhileTimes {
    pub s_poly: Duration,
    pub normal_form: Duration,
    pub remainder_normalize: Duration,
    pub pair_update: Duration,
}

/// Measure the duration of `f`, add it to `out`, and return the result.
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
