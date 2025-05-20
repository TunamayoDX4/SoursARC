use std::{net::SocketAddr, sync::LazyLock};

use axum::{Router, response::Html, routing::get};
use serde::{Deserialize, Serialize};

mod ws;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
  pub host: String,
  pub socket: Vec<SocketAddr>,
}
impl Default for ServerConfig {
  fn default() -> Self {
    Self {
      host: "https://app.example.com".to_string(),
      socket: vec![SocketAddr::from(([0, 0, 0, 0], 8080))],
    }
  }
}

/// サーバを停止するための通知
/// Notify for stopping the server
static SERVER_STOP_NOTIFY: LazyLock<tokio::sync::Notify> =
  LazyLock::new(|| tokio::sync::Notify::new());

/// サーバ停止の通知を待機する
/// Wait for the server stop notification
async fn server_stop(socket: SocketAddr) {
  tracing::info!(
    "Server stopping signal waiting on {}",
    socket
  );
  SERVER_STOP_NOTIFY.notified().await;
  tracing::info!(
    "Server stopping process initializing on {}",
    socket
  );
}

/// サーバ停止の通知を送信する
/// Send a notification to stop the server
async fn wait_for_ctrlc_and_sigterm() {
  #[cfg(unix)]
  tokio::select! {
    _ = tokio::signal::ctrl_c() => {
      tracing::info!("Ctrl-C received");
    }
    _ = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) => {
      tracing::info!("SIGTERM received");
    }
  }
  #[cfg(windows)]
  tokio::select! {
    _ = tokio::signal::ctrl_c() => {
      tracing::info!("Ctrl-C received");
    }
  }
  SERVER_STOP_NOTIFY.notify_waiters();
}

/// サーバの起動
/// Start the server
pub async fn server_starting() -> Result<(), crate::StdError>
{
  let server_conf = &crate::CONFIG.server;

  // Ctrl-CとSIGTERMを待機する
  tokio::spawn(wait_for_ctrlc_and_sigterm());

  let app = Router::new()
    .route("/", get(|| async { Html("Hello, World!") }));

  tracing::info!(
    "Server starting on {}",
    server_conf
      .socket
      .iter()
      .map(|s| s.to_string())
      .collect::<Vec<_>>()
      .join(", ")
  );
  for listener in
    server_conf.socket.iter().map(async |socket| {
      let bind =
        tokio::net::TcpListener::bind(socket).await?;
      axum::serve(
        bind,
        app
          .clone()
          .into_make_service_with_connect_info::<SocketAddr>(
          ),
      )
      .with_graceful_shutdown(server_stop(socket.clone()))
      .await
    })
  {
    listener.await?;
  }
  tracing::info!("Server stopped");

  Ok(())
}
