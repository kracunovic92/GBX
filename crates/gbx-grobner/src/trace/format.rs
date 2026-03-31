use std::time::Duration;

/// Formats a byte count in a compact human-readable style.
#[must_use]
pub fn fmt_bytes(n: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let x = n as f64;
    if x >= GB {
        format!("{:.2} GiB", x / GB)
    } else if x >= MB {
        format!("{:.2} MiB", x / MB)
    } else if x >= KB {
        format!("{:.2} KiB", x / KB)
    } else {
        format!("{n} B")
    }
}

/// Formats a duration in whole milliseconds.
#[must_use]
pub fn fmt_duration_ms(d: Duration) -> String {
    format!("{}ms", d.as_millis())
}
