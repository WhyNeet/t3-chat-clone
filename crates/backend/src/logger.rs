#[cfg(not(debug_assertions))]
pub struct Logger(tracing_appender::non_blocking::WorkerGuard);
#[cfg(debug_assertions)]
pub struct Logger();

impl Logger {
    pub fn init() -> anyhow::Result<Self> {
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::EnvFilter;
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;

        #[cfg(not(debug_assertions))]
        {
            if !std::fs::exists(".log")? {
                std::fs::create_dir(".log")?;
            }
        };

        #[cfg(not(debug_assertions))]
        let (file_layer, guard) = {
            let file_appender = tracing_appender::rolling::daily(".log", &format!("t3-clone.log"));
            let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

            (
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking_appender)
                    .with_ansi(false),
                guard,
            )
        };

        let stdout_layer = tracing_subscriber::fmt::layer().pretty();

        let subscriber = tracing_subscriber::registry()
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .with(stdout_layer);

        #[cfg(not(debug_assertions))]
        let subscriber = subscriber.with(file_layer);

        subscriber.init();

        #[cfg(debug_assertions)]
        return Ok(Self());
        #[cfg(not(debug_assertions))]
        Ok(Self(guard))
    }
}
