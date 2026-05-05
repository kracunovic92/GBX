#![allow(missing_docs, unused_imports)]
pub mod alloc;
pub mod dump;
pub mod snapshot;

#[cfg(feature = "instrumentation")]
pub use tracing::{debug, info, trace, warn};

#[cfg(feature = "instrumentation")]
#[macro_export]
macro_rules! f4_span {
    ($name:expr) => {
        let _f4_span_guard = tracing::debug_span!($name).entered();
    };

    ($name:expr, $($field:tt)*) => {
        let _f4_span_guard = tracing::debug_span!($name, $($field)*).entered();
    };
}

#[cfg(not(feature = "instrumentation"))]
#[macro_export]
macro_rules! f4_span {
    ($name:expr) => {};
    ($name:expr, $($field:tt)*) => {};
}

#[cfg(feature = "instrumentation")]
#[macro_export]
macro_rules! f4_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[cfg(not(feature = "instrumentation"))]
#[macro_export]
macro_rules! f4_debug {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "instrumentation")]
#[macro_export]
macro_rules! f4_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[cfg(not(feature = "instrumentation"))]
#[macro_export]
macro_rules! f4_info {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "instrumentation")]
#[macro_export]
macro_rules! f4_timed_span {
    ($name:expr) => {
        let _f4_span_guard = tracing::debug_span!($name).entered();
    };

    ($name:expr, $($field:tt)*) => {
        let _f4_span_guard = tracing::debug_span!($name, $($field)*).entered();
    };
}

#[cfg(not(feature = "instrumentation"))]
#[macro_export]
macro_rules! f4_timed_span {
    ($name:expr) => {};
    ($name:expr, $($field:tt)*) => {};
}
