//! Structured F4 profiling events.
//!
//! These events are intended for experiment artifacts, not interactive logging.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::Instant;

#[cfg(feature = "profiling-alloc")]
use stats_alloc::{INSTRUMENTED_SYSTEM, Region};

/// Allocation traffic recorded for one profiled region.
#[cfg_attr(feature = "profile-dump", derive(serde::Serialize))]
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
pub struct AllocProfile {
    pub bytes_allocated: u64,
    pub bytes_deallocated: u64,
    pub bytes_reallocated: u64,
}

/// One structured profiling event emitted by F4.
#[cfg_attr(feature = "profile-dump", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct ProfileEvent {
    pub phase: &'static str,
    pub elapsed_ms: f64,
    pub allocations: Option<AllocProfile>,
    pub counters: BTreeMap<&'static str, u64>,
}

/// Numeric metadata attached to a profiling event.
pub type ProfileCounters = BTreeMap<&'static str, u64>;

thread_local! {
    static EVENTS: RefCell<Vec<ProfileEvent>> = const { RefCell::new(Vec::new()) };
}

/// Creates an empty counter map.
#[inline]
#[must_use]
pub fn counters() -> ProfileCounters {
    BTreeMap::new()
}

/// Converts a `usize` counter into the event counter type.
#[inline]
#[must_use]
pub fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

/// Runs `f` while recording elapsed time and optional allocation traffic.
#[inline]
pub fn with_profile_phase<T>(phase: &'static str, counters: ProfileCounters, f: impl FnOnce() -> T) -> T {
    #[cfg(feature = "profiling-alloc")]
    {
        let started = Instant::now();
        let region = Region::new(&INSTRUMENTED_SYSTEM);

        let result = f();

        let elapsed_ms = elapsed_ms(started);
        let allocations = Some(AllocProfile::from(region.change()));

        record_profile_event(ProfileEvent { phase, elapsed_ms, allocations, counters });

        result
    }

    #[cfg(not(feature = "profiling-alloc"))]
    {
        let started = Instant::now();

        let result = f();

        let elapsed_ms = elapsed_ms(started);

        record_profile_event(ProfileEvent { phase, elapsed_ms, allocations: None, counters });

        result
    }
}

/// Records an already-measured event.
pub fn record_profile_event(event: ProfileEvent) {
    EVENTS.with(|events| events.borrow_mut().push(event));
}

/// Drains all profile events recorded on the current thread.
#[must_use]
pub fn take_profile_events() -> Vec<ProfileEvent> {
    EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
}

/// Clears all profile events recorded on the current thread.
pub fn clear_profile_events() {
    EVENTS.with(|events| events.borrow_mut().clear());
}

#[inline]
fn elapsed_ms(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

#[cfg(feature = "profiling-alloc")]
impl From<stats_alloc::Stats> for AllocProfile {
    fn from(stats: stats_alloc::Stats) -> Self {
        let bytes_allocated = u64::try_from(stats.bytes_allocated).unwrap_or(u64::MAX);
        let bytes_deallocated = u64::try_from(stats.bytes_deallocated).unwrap_or(u64::MAX);
        let bytes_reallocated = u64::try_from(stats.bytes_reallocated).unwrap_or(u64::MAX);

        Self { bytes_allocated, bytes_deallocated, bytes_reallocated }
    }
}
