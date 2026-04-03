use tracing::info;

/// Initialize the tracing subscriber with appropriate settings
pub fn init_logging() {
    // Set up logging to:
    //   - Release: file only, INFO level
    //   - Debug: stdout + file, DEBUG level
    #[cfg(debug_assertions)]
    {
        use tracing_subscriber::{fmt, EnvFilter};
        fmt::Subscriber::builder()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::new("slapmac=debug")),
            )
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        use tracing_subscriber::{fmt, EnvFilter};
        // In release, log to file only
        let log_path = dirs::data_local_dir()
            .unwrap_or_default()
            .join("SlapMAC")
            .join("slapmac.log");
        
        let file_appender = tracing_appender::rolling::daily(
            log_path.parent().unwrap(),
            "slapmac.log",
        );
        
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::new("slapmac=info"))
            .with_writer(non_blocking)
            .without_time()
            .init();
    }

    info!("Logging initialized");
}
