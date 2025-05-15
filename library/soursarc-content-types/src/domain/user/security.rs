use argon2::{
  PasswordHash, PasswordVerifier,
  password_hash::{PasswordHasher, SaltString, rand_core},
};
use serde::{Deserialize, Serialize};

use crate::id::UserSecureID;

/// SoursArcのユーザのセキュリティ情報を表す構造体
/// SoursArc user security information struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSecurityData {
  pub id: UserSecureID,
  pub user_name_id: String,
  password_hash: String,
  pub email: String,
  pub last_login: chrono::DateTime<chrono::Utc>,
  pub last_pswd_change: chrono::DateTime<chrono::Utc>,
}
impl UserSecurityData {
  /// パスワードをハッシュ化する
  /// Hash the password
  pub fn verify_password(
    &mut self,
    password: &str,
    argon2_config: &argon2::Params,
  ) -> Result<bool, PasswordHashError> {
    tracing::info!("Verifying password");
    tracing::debug!("Password verifying process started");
    // パスワードを検証する
    // Verify password
    let hasher = argon2::Argon2::new(
      argon2::Algorithm::Argon2id,
      argon2::Version::V0x13,
      argon2_config.clone(),
    );
    // パスワードのハッシュをPHC形式から変換する
    let password_hash =
      PasswordHash::new(&self.password_hash)
        .map_err(|e| PasswordHashError::InvalidFormat(e))?;

    tracing::debug!(
      "Password hash generation process started"
    );
    // パスワードを検証する
    let result = hasher
      .verify_password(password.as_bytes(), &password_hash);

    // 新しいハッシュを生成する
    let salt_string =
      SaltString::generate(&mut rand_core::OsRng);
    let new_hash = hasher
      .hash_password(password.as_bytes(), &salt_string);

    tracing::debug!(
      "Password verification process finished"
    );

    let new_hash = new_hash.map_err(|e| {
      PasswordHashError::Argon2HashingError(e)
    })?;

    if result.is_ok() {
      self.password_hash = new_hash.to_string();
      self.last_login = chrono::Utc::now();

      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// データを新規作成する
  /// Create new data
  pub fn new(
    id: UserSecureID,
    user_name_id: String,
    raw_password: &str,
    email: String,
    argon2_config: argon2::Params,
  ) -> Result<Self, PasswordHashError> {
    let hasher = argon2::Argon2::new(
      argon2::Algorithm::Argon2id,
      argon2::Version::V0x13,
      argon2_config,
    );
    let salt = SaltString::generate(&mut rand_core::OsRng);
    let password_hash = hasher
      .hash_password(raw_password.as_bytes(), &salt)
      .map_err(|e| {
        PasswordHashError::Argon2HashingError(e)
      })?;

    Ok(Self {
      id,
      user_name_id,
      password_hash: password_hash.to_string(),
      email,
      last_login: chrono::Utc::now(),
      last_pswd_change: chrono::Utc::now(),
    })
  }
}

/// SoursArcのユーザのパスワード検証にかかわるエラーを表す列挙型
/// SoursArc user password verification error enum
#[derive(Debug, Clone)]
pub enum PasswordHashError {
  /// パスワードのハッシュが無効な場合
  /// Invalid password hash
  Invalid,

  /// パスワードのハッシュが不正な形式の場合
  /// Invalid password hash format
  InvalidFormat(password_hash::Error),

  /// パスワードのハッシュの検証に失敗した場合
  /// Failed to verify password hash
  Argon2HashingError(argon2::password_hash::Error),
}
impl std::fmt::Display for PasswordHashError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      PasswordHashError::Invalid => {
        write!(f, "Invalid password hash")
      }
      PasswordHashError::InvalidFormat(e) => {
        write!(f, "Invalid password hash format: {e}")
      }
      PasswordHashError::Argon2HashingError(e) => {
        write!(f, "Argon2 error: {}", e)
      }
    }
  }
}
impl std::error::Error for PasswordHashError {}

/// SoursArcのJWTトークンのペイロードを表す構造体
/// SoursArc JWT token payload struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
  /// ユーザのID
  /// ID of the user
  pub user_id: super::UserID,

  /// ユーザの文字列ID
  /// String ID of the user
  pub sub: String,

  /// トークンの有効期限
  /// Expiration date of the token
  pub exp: chrono::DateTime<chrono::Utc>,
}
