#[cfg(feature = "instrumentation")]
macro_rules! f4_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*);
    };
}

#[cfg(not(feature = "instrumentation"))]
macro_rules! f4_info {
    ($($arg:tt)*) => {};
}

pub(crate) use f4_info;
#[cfg(feature = "instrumentation")]
macro_rules! f4_span {
    ($name:literal) => {
        let _span = tracing::info_span!($name).entered();
    };
    ($name:literal, $($field:tt)*) => {
        let _span = tracing::info_span!($name, $($field)*).entered();
    };
}

#[cfg(not(feature = "instrumentation"))]
macro_rules! f4_span {
    ($name:literal) => {};
    ($name:literal, $($field:tt)*) => {};
}

pub(crate) use f4_span;

#[cfg(feature = "instrumentation")]
macro_rules! f4_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*);
    };
}

#[cfg(not(feature = "instrumentation"))]
macro_rules! f4_debug {
    ($($arg:tt)*) => {};
}

pub(crate) use f4_debug;
