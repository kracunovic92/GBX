use std::sync::{Arc, Mutex, MutexGuard};

/// Shared thread-safe handle for mutable trace state.
///
/// This is suitable for both single-threaded and multi-threaded algorithms.
/// Instrumentation wrappers and engine code can all share the same tracer.
#[derive(Debug)]
pub struct TraceHandle<T> {
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for TraceHandle<T> {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl<T> TraceHandle<T> {
    /// Creates a new shared handle.
    #[must_use]
    #[inline]
    pub fn new(value: T) -> Self {
        Self { inner: Arc::new(Mutex::new(value)) }
    }

    /// Locks and returns mutable access to the underlying tracer.
    #[inline]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().expect("trace mutex poisoned")
    }
}
