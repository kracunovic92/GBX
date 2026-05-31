#[cfg(feature = "instrumentation")]
pub fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{EnvFilter, fmt};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,gbx_grobner=debug,graph_experiments=debug"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .init();
}

#[cfg(not(feature = "instrumentation"))]
pub fn init_tracing() {}
