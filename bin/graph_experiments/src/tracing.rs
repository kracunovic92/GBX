pub fn init_tracing() {
    #[cfg(feature = "instrumentation")]
    {
        use std::sync::Once;
        use tracing_subscriber::{filter::LevelFilter, fmt};

        static INIT: Once = Once::new();

        INIT.call_once(|| {
            fmt()
                .with_max_level(LevelFilter::TRACE)
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .compact()
                .init();
        });
    }
}
