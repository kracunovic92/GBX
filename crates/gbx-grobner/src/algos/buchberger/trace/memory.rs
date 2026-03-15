use std::fs;

/// Best-effort process memory snapshot.
///
/// Values are process-level measurements and include allocator/runtime effects.
/// They are useful for trend observation, not exact per-data-structure
/// accounting.
#[derive(Debug, Default, Clone, Copy)]
pub struct MemorySnapshot {
    /// Resident set size in bytes, if available.
    pub rss_bytes: u64,
}

/// Returns a best-effort snapshot of current process memory usage.
///
/// This implementation is Linux-oriented and reads `/proc/self/status`.
/// If unavailable, returns `None`.
#[must_use]
pub fn current_memory_snapshot() -> Option<MemorySnapshot> {
    let status = fs::read_to_string("/proc/self/status").ok()?;
    let mut rss_kb = None;

    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kb = rest
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<u64>().ok())?;
            rss_kb = Some(kb);
            break;
        }
    }

    Some(MemorySnapshot { rss_bytes: rss_kb? * 1024 })
}
