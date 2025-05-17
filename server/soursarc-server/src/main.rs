use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

type StdError =
  Box<dyn std::error::Error + Send + Sync + 'static>;

mod logger;
mod server;

/// コンフィグのファイル名
const CONFIG_FILE: &str = "config.json";

/// 共通の設定
/// Common configuration
#[derive(Debug, Serialize, Deserialize, Default)]
struct CommonConfig {
  pub log: logger::LoggerConfig,
  pub server: server::ServerConfig,
}
impl CommonConfig {
  /// 設定を初期化する
  /// JSONファイルから読み取り、NotFoundであれば新規作成するが、設定を促すために一度プログラムを終了させる。
  ///
  /// Initialize the configuration
  fn new() -> Result<Self, StdError> {
    let config: Self =
      if std::path::Path::new(CONFIG_FILE).exists() {
        let file = std::fs::File::open(CONFIG_FILE)?;
        serde_json::from_reader(file)?
      } else {
        let config = Self::default();
        let file = std::fs::File::create(CONFIG_FILE)?;
        serde_json::to_writer_pretty(file, &config)?;
        eprintln!(
          "Config file not found. Created a new one at {}",
          CONFIG_FILE
        );
        std::process::exit(1);
      };
    Ok(config)
  }
}

/// コンフィグ
static CONFIG: LazyLock<CommonConfig> =
  LazyLock::new(|| CommonConfig::new().unwrap());

/// エントリポイント
/// Entry point
#[tokio::main]
async fn main() -> Result<(), StdError> {
  // ロガーの初期化
  let _guard = logger::initialize_logger(&CONFIG.log)?;

  tracing::info!("サーバを起動します");

  // Print a message to the console
  server::server_starting().await?;

  tracing::info!("サーバを終了します");

  Ok(())
}
