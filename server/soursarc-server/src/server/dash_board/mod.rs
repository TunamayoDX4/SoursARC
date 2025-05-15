use std::{net::SocketAddr, sync::LazyLock};

use axum::{
  Form, Router,
  extract::ConnectInfo,
  response::{Html, IntoResponse, Redirect},
};
use hyper::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

pub mod admin_users;
pub mod limiter;

const DASH_BOARD_CSS: &str = include_str!("dash_board.css");
static LIMITER: LazyLock<limiter::RateLimiter> =
  LazyLock::new(|| {
    limiter::RateLimiter::new(
      &crate::CONFIG.dashboard.access_limiter_config,
    )
  });

/// # DashBoardConfig
/// ダッシュボードの設定
#[derive(Debug, Serialize, Deserialize)]
pub struct DashBoardConfig {
  pub default_user: String,
  pub default_password: String,
  pub user_list_dir: String,
  pub csv_keep_count: usize,
  pub argon2_m_cost: u32,
  pub argon2_t_cost: u32,
  pub argon2_p_cost: u32,
  pub access_limiter_config: limiter::RateLimiterConfig,
}
impl Default for DashBoardConfig {
  fn default() -> Self {
    Self {
      default_user: "＊＊絶対に変更してください＊＊"
        .to_string(),
      default_password: "＊＊絶対に変更してください＊＊"
        .to_string(),
      user_list_dir: "user_list".to_string(),
      csv_keep_count: 10,
      argon2_m_cost: 32768,
      argon2_t_cost: 2,
      argon2_p_cost: 4,
      access_limiter_config: limiter::RateLimiterConfig {
        expire_duration:
          limiter::RateLimiterConfig::default()
            .expire_duration,
        limit_per_dur: limiter::RateLimiterConfig::default()
          .limit_per_dur,
        cleanup_interval:
          limiter::RateLimiterConfig::default()
            .cleanup_interval,
      },
    }
  }
}

async fn dash_board_index() -> impl IntoResponse {
  let page = format!(
    r#"
    <!DOCTYPE html>
    <html lang="ja">
      <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>DashBoard ＠ SoursARC ～そうさ～く～</title>
        <style>{DASH_BOARD_CSS}</style>
      </head>
      <body>
        <header>
          <h1>DashBoard ＠ SoursARC</h1>
          <div id="mini-summary">
            ABCDEFGHIJKLMNOPQRSTUVWXYZ
          </div>
        </header>
        <main>
          <form id="login-form" action="dash_board/login" method="post">
            <h2>Welcome to the DashBoard!</h2>
            <label for="username">ユーザ名:</label>
            <input type="text" id="username" name="username" required>
            <label for="password">パスワード:</label>
            <input type="password" id="password" name="password" required>
            <button type="submit">ログイン</button>
          </form>
        </main>
      </body>
    </html>
    "#
  );
  Html(page)
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginForm {
  #[serde(default)]
  username: Option<String>,
  #[serde(default)]
  password: Option<String>,
}

async fn login(
  headers: HeaderMap,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
  Form(form): Form<LoginForm>,
) -> impl IntoResponse {
  match limiter::extract_client_ip(&headers, &addr) {
    Some(ip) => {
      if !LIMITER.check(ip).await {
        tracing::warn!("Rate limit exceeded for IP: {ip}");
        return (
          StatusCode::TOO_MANY_REQUESTS,
          Html(
            r#"
            <!DOCTYPE html>
              <html>
                <head><title>429</title></head>
                <body>
                  <h1>429 Too Many Requests</h1>
                </body>
              </html>
            "#,
          ),
        )
          .into_response();
      }
      tracing::debug!("Client IP: {ip}");
    }
    None => {
      tracing::warn!("Failed to extract client IP");
      return (
        StatusCode::BAD_REQUEST,
        Html(
          r#"
          <!DOCTYPE html>
            <html>
              <head><title>Bad Request</title></head>
              <body>
                <h1>Bad Request</h1>
              </body>
            </html>
          "#,
        ),
      )
        .into_response();
    }
  }
  match form {
    LoginForm {
      username: Some(username),
      password: Some(password),
    } if !username.is_empty() && !password.is_empty() => {
      let mut admin = admin_users::STORAGE.lock().await;
      match admin.verify_by_name(&username, &password).await
      {
        Ok(true) => (
          StatusCode::OK,
          Html(
            r#"
          <!DOCTYPE html>
            <html>
              <head><title>DashBoard</title></head>
              <body>
                <h1>DashBoard</h1>
                <p>ログイン成功！</p>
                <a href="/">戻る</a>
              </body>
            </html>
          "#,
          ),
        )
          .into_response(),
        e @ Err(_) | e @ Ok(false) => {
          tracing::debug!("Login failed: {e:?}");
          (
            StatusCode::FORBIDDEN,
            Html(
              r#"
              <!DOCTYPE html>
                <html>
                  <head><title>403</title></head>
                  <body>
                    <h1>403 Forbidden</h1>
                  </body>
                </html>
          "#,
            ),
          )
            .into_response()
        }
      }
    }
    _ => {
      Redirect::to("/?error=missing_form").into_response()
    }
  }
}

pub fn router() -> Router {
  Router::new()
    .route("/", axum::routing::get(dash_board_index))
    .route("/login", axum::routing::post(login))
    .with_state(())
}
