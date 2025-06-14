pub struct Logger();

impl Logger {
    pub fn init() -> anyhow::Result<Self> {
        use tracing::level_filters::LevelFilter;
        use tracing_subscriber::EnvFilter;
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;

        let stdout_layer = tracing_subscriber::fmt::layer().pretty();

        let subscriber = tracing_subscriber::registry()
            .with(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .with(stdout_layer);

        subscriber.init();

        return Ok(Self());
    }
}
