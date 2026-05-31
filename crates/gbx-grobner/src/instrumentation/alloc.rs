//! Per-phase allocation + timing profiling helpers.

use crate::f4_debug;
use std::time::Instant;

/// Allocation traffic recorded for one profiled region.
#[cfg(feature = "profiling-alloc")]
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
pub struct AllocProfile {
    pub bytes_allocated: usize,
    pub bytes_deallocated: usize,
    pub bytes_reallocated: usize,
}

#[cfg(feature = "profiling-alloc")]
#[inline]
#[allow(clippy::cast_precision_loss)]
fn bytes_to_mib(bytes: usize) -> f64 {
    bytes as f64 / 1024.0 / 1024.0
}
#[cfg(feature = "profiling-alloc")]
#[inline]
fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

#[cfg(feature = "profiling-alloc")]
impl From<stats_alloc::Stats> for AllocProfile {
    fn from(stats: stats_alloc::Stats) -> Self {
        let bytes_reallocated = usize::try_from(stats.bytes_reallocated).unwrap_or_default();

        Self { bytes_allocated: stats.bytes_allocated, bytes_deallocated: stats.bytes_deallocated, bytes_reallocated }
    }
}

/// Run `f` while recording allocation traffic and elapsed time for this phase.
#[cfg(feature = "profiling-alloc")]
pub fn with_alloc_profile<T>(phase: &'static str, f: impl FnOnce() -> T) -> T {
    use stats_alloc::{INSTRUMENTED_SYSTEM, Region};

    let started = Instant::now();
    let region = Region::new(&INSTRUMENTED_SYSTEM);

    let result = f();

    let elapsed_ms = elapsed_ms(started);
    let profile = AllocProfile::from(region.change());

    f4_debug!(
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
pub fn with_alloc_profile<T>(_phase: &'static str, f: impl FnOnce() -> T) -> T {
    let _started = Instant::now();

    let result = f();

    result
}
