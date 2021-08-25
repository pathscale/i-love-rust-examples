use anyhow::Context;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;
use tracing_subscriber::fmt;
pub fn setup_logs(log_level: LevelFilter) -> anyhow::Result<()> {
    LogTracer::init().context("Cannot setup_logs")?;
    let default_level = format!("[]={}", log_level);
    let subscriber = fmt()
        .with_thread_names(true)
        .with_env_filter(
            default_level,
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("Cannot setup_logs")?;
    log_panics::init();
    Ok(())
}
