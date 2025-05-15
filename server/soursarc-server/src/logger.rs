use serde::{Deserialize, Serialize};

use crate::StdError;

/// ロガーの設定
/// Logger configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerConfig {
  pub dir: String,
  pub filename_prefix: String,
  pub max_rolling_count: usize,
}
impl Default for LoggerConfig {
  fn default() -> Self {
    Self {
      dir: "log".to_string(),
      filename_prefix: "log-soursarc_".to_string(),
      max_rolling_count: 14,
    }
  }
}

/// ロガーの初期化
/// Initialize the logger
pub fn initialize_logger(
  config: &LoggerConfig,
) -> Result<
  tracing_appender::non_blocking::WorkerGuard,
  StdError,
> {
  let tracing_appender =
    tracing_appender::rolling::Builder::new()
      .rotation(tracing_appender::rolling::Rotation::DAILY)
      .filename_prefix(&config.filename_prefix)
      .filename_suffix(".log")
      .max_log_files(config.max_rolling_count)
      .build(&config.dir)?;
  let (non_blocking, guard) =
    tracing_appender::non_blocking(tracing_appender);
  tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .with_max_level(if cfg!(debug_assertions) {
      tracing::Level::TRACE
    } else {
      tracing::Level::INFO
    })
    .with_ansi(false)
    .with_line_number(true)
    .init();
  Ok(guard)
}
