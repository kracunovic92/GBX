use std::time::{Duration, Instant};

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
