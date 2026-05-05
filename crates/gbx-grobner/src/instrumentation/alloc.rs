//! Per-phase allocation + timing profiling helpers.

use std::time::Instant;

#[derive(Debug, Clone, Copy, Default)]
pub struct AllocProfile {
    pub allocations: usize,
    pub deallocations: usize,
    pub reallocations: usize,

    pub bytes_allocated: usize,
    pub bytes_deallocated: usize,
    pub bytes_reallocated: usize,
}

#[inline]
fn bytes_to_mib(bytes: usize) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}

#[inline]
fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

#[cfg(feature = "profiling-alloc")]
impl From<stats_alloc::Stats> for AllocProfile {
    fn from(stats: stats_alloc::Stats) -> Self {
        Self {
            allocations: stats.allocations,
            deallocations: stats.deallocations,
            reallocations: stats.reallocations,
            bytes_allocated: stats.bytes_allocated,
            bytes_deallocated: stats.bytes_deallocated,
            bytes_reallocated: stats.bytes_reallocated as usize,
        }
    }
}

/// Run `f` while recording allocation traffic and elapsed time for this phase.
#[cfg(feature = "profiling-alloc")]
pub fn with_alloc_profile<T>(phase: &'static str, f: impl FnOnce() -> T) -> T {
    use stats_alloc::{Region, INSTRUMENTED_SYSTEM};

    let started = Instant::now();
    let region = Region::new(&INSTRUMENTED_SYSTEM);

    let result = f();

    let elapsed_ms = elapsed_ms(started);
    let profile = AllocProfile::from(region.change());

    tracing::debug!(
        phase,
        elapsed_ms,
        mib_allocated = bytes_to_mib(profile.bytes_allocated),
        mib_deallocated = bytes_to_mib(profile.bytes_deallocated),
        mib_reallocated = bytes_to_mib(profile.bytes_reallocated),
        "F4 phase profile"
    );

    result
}

/// Timing-only version used when allocation profiling is disabled.
#[cfg(not(feature = "profiling-alloc"))]
#[inline]
pub fn with_alloc_profile<T>(phase: &'static str, f: impl FnOnce() -> T) -> T {
    let started = Instant::now();

    let result = f();

    result
}
