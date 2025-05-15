use argon2::PasswordVerifier;
use chrono::Utc;
use csv::{ReaderBuilder, WriterBuilder};
use password_hash::{
  PasswordHasher, SaltString,
  rand_core::{OsRng, RngCore},
};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap, fs, path::PathBuf, str::FromStr,
  sync::LazyLock,
};
use tokio::sync::Mutex;

use crate::CONFIG;
pub const USER_LIST_FILE_NAME: &str = "admin";
pub const USER_LIST_FILE_EXT: &str = "csv";
pub static STORAGE: LazyLock<Mutex<AdminUserStorage>> =
  LazyLock::new(|| {
    Mutex::new(
      AdminUserStorage::load_or_init(
        &CONFIG.dashboard,
        AdminUser::new(
          &CONFIG.dashboard.default_user,
          &CONFIG.dashboard.default_password,
          AdminUserRoleSet::new()
            | AdminUserRole::FullAccess,
        )
        .unwrap(),
      )
      .unwrap(),
    )
  });

/// # FAKE_USER
/// サイドチャネル攻撃の対策用のダミーユーザ
///
/// どんな入力でもこいつに対してログインを試みる
/// これにより処理時間のばらつきをなくす
///
pub static FAKE_USER: LazyLock<Mutex<AdminUser>> =
  LazyLock::new(|| {
    Mutex::new(
      AdminUser::new(
        "fake_user",
        "fake_password",
        AdminUserRoleSet::new(),
      )
      .unwrap(),
    )
  });

#[derive(Debug)]
pub enum AdminUserError {
  UserNotFound,
  InvalidPassword,
  InvalidUserId,
  InvalidUserIdString,
  InvalidPhcString,
  InvalidHasherConfig,
  IOError(std::io::Error),
}
impl std::fmt::Display for AdminUserError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      AdminUserError::UserNotFound => {
        write!(f, "User not found")
      }
      AdminUserError::InvalidPassword => {
        write!(f, "Invalid password")
      }
      AdminUserError::InvalidUserId => {
        write!(f, "Invalid user id")
      }
      AdminUserError::InvalidUserIdString => {
        write!(f, "Invalid user id string")
      }
      AdminUserError::InvalidPhcString => {
        write!(f, "Invalid phc string")
      }
      AdminUserError::InvalidHasherConfig => {
        write!(f, "Invalid hasher config")
      }
      AdminUserError::IOError(err) => write!(f, "{}", err),
    }
  }
}
impl std::error::Error for AdminUserError {
  fn source(
    &self,
  ) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      AdminUserError::IOError(err) => Some(err),
      _ => None,
    }
  }
}
impl From<std::io::Error> for AdminUserError {
  fn from(err: std::io::Error) -> Self {
    AdminUserError::IOError(err)
  }
}

/// # AdminUserRole
/// 管理者ユーザの権限
#[repr(u8)]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
pub enum AdminUserRole {
  Nothing = 0b00000000,
  ReadSecurityConfig = 0b00000001,
  ModifySecurityConfig = 0b00000010,
  ReadSystemConfig = 0b00000100,
  ModifySystemConfig = 0b00001000,
  FullAccess = 0b11111111,
}
impl From<AdminUserRole> for &'static str {
  fn from(role: AdminUserRole) -> Self {
    match role {
      AdminUserRole::Nothing => "nothing",
      AdminUserRole::ReadSecurityConfig => {
        "read_secure_config"
      }
      AdminUserRole::ModifySecurityConfig => {
        "mod_secure_config"
      }
      AdminUserRole::ReadSystemConfig => {
        "read_system_config"
      }
      AdminUserRole::ModifySystemConfig => {
        "mod_system_config"
      }
      AdminUserRole::FullAccess => "full_access",
    }
  }
}
impl std::ops::BitOr for AdminUserRole {
  type Output = AdminUserRoleSet;

  fn bitor(self, rhs: Self) -> Self::Output {
    AdminUserRoleSet((self as u8) | (rhs as u8))
  }
}

/// # AdminUserRoleSet
/// 管理者ユーザの権限セット
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct AdminUserRoleSet(u8);
impl AdminUserRoleSet {
  pub fn new() -> Self {
    Self(0)
  }

  pub fn add(&mut self, role: AdminUserRole) {
    self.0 |= role as u8;
  }

  pub fn remove(&mut self, role: AdminUserRole) {
    self.0 &= !(role as u8);
  }

  pub fn has(&self, role: AdminUserRole) -> bool {
    (self.0 & (role as u8)) != 0
  }
}
impl std::ops::BitOr<AdminUserRole> for AdminUserRoleSet {
  type Output = Self;

  fn bitor(self, rhs: AdminUserRole) -> Self::Output {
    Self(self.0 | rhs as u8)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUser {
  pub user_id: ulid::Ulid,
  pub user_id_string: String,
  pub role: AdminUserRoleSet,
  phc_string: String,
}
impl AdminUser {
  /// # new
  /// 管理者ユーザを新規作成する
  pub fn new(
    user_name: &str,
    raw_password: &str,
    role: AdminUserRoleSet,
  ) -> Result<Self, AdminUserError> {
    if user_name.is_empty() {
      return Err(AdminUserError::InvalidUserId);
    }
    if raw_password.is_empty() {
      return Err(AdminUserError::InvalidPassword);
    }
    let user_id = ulid::Ulid::new();
    let user_id_string = user_name.to_string();
    let hasher = argon2::Argon2::new(
      argon2::Algorithm::Argon2id,
      argon2::Version::V0x13,
      argon2::Params::new(
        CONFIG.dashboard.argon2_m_cost,
        CONFIG.dashboard.argon2_t_cost,
        CONFIG.dashboard.argon2_p_cost,
        None,
      )
      .map_err(|_| AdminUserError::InvalidHasherConfig)?,
    );
    let salt = SaltString::generate(OsRng::default());
    let phc_string = hasher
      .hash_password(raw_password.as_bytes(), &salt)
      .map_err(|_| AdminUserError::InvalidPassword)?;
    let phc_string = phc_string.to_string();
    Ok(Self { user_id, user_id_string, role, phc_string })
  }

  /// # verify
  /// 管理者ユーザのパスワードを検証する
  /// - `raw_password`: 検証するパスワード
  pub fn verify(
    &mut self,
    raw_password: &str,
  ) -> Result<bool, AdminUserError> {
    if raw_password.is_empty() {
      return Err(AdminUserError::InvalidPassword);
    }
    let hasher = argon2::Argon2::new(
      argon2::Algorithm::Argon2id,
      argon2::Version::V0x13,
      argon2::Params::new(
        CONFIG.dashboard.argon2_m_cost,
        CONFIG.dashboard.argon2_t_cost,
        CONFIG.dashboard.argon2_p_cost,
        None,
      )
      .map_err(|_| AdminUserError::InvalidHasherConfig)?,
    );
    let password_hash =
      password_hash::PasswordHash::new(&self.phc_string)
        .map_err(|_| AdminUserError::InvalidPhcString)?;
    let result = hasher
      .verify_password(
        raw_password.as_bytes(),
        &password_hash,
      )
      .map(|_| true)
      .map_err(|_| AdminUserError::InvalidPassword);

    let mut out = [0u8; 64];
    let salt = SaltString::generate(OsRng::default());

    // Resultの結果をとりあえず意味はないが使う。
    // これをしないとコンパイラに早期returnされるかもなので。
    out[0] = result
      .as_ref()
      .map(|i| i.then_some(1).unwrap_or(0))
      .unwrap_or(0);

    let out = hasher
      .hash_password(raw_password.as_bytes(), &salt)
      .map_err(|_| AdminUserError::InvalidPassword)?;

    if !result? {
      return Ok(false);
    } else {
      self.phc_string = out.to_string();

      return Ok(true);
    }
  }
}

/// # AdminUserStorage
/// 管理者ユーザのストレージ
///
/// ## Member
/// - `users`: `Vec<Option<AdminUser>>`
///   - 管理者ユーザの実体
/// - `ulid_map`: `HashMap<ulid::Ulid, usize>`
///   - Ulidからインデックスを引くマップ
/// - `name_map`: `HashMap<String, usize>`
///   - user_id_stringからインデックスを引くマップ
pub struct AdminUserStorage {
  users: Vec<Option<AdminUser>>,
  ulid_map: HashMap<ulid::Ulid, usize>,
  name_map: HashMap<String, usize>,
}

impl AdminUserStorage {
  /// ストレージを初期化する
  ///
  /// ## Summary
  /// - CSVファイルが存在しなければ、デフォルトユーザを挿入して新規作成
  /// - 既存ファイルがあれば新しい順にロードを試みる
  /// - どれもダメならエラーで落ちる
  ///
  /// ## Argument
  /// - `config`: `&DashBoardConfig`
  ///   - 設定
  /// - `default_user`: `AdminUser`
  ///   - デフォルトユーザ
  ///
  /// ## Return value
  /// - `Self`
  pub fn load_or_init(
    config: &crate::server::dash_board::DashBoardConfig,
    default_user: AdminUser,
  ) -> std::io::Result<Self> {
    let dir = PathBuf::from(&config.user_list_dir);
    fs::create_dir_all(&dir)?;

    // CSVファイル一覧を取得（admin_*.csv形式）
    let mut csv_files: Vec<_> = fs::read_dir(&dir)?
      .filter_map(|e| e.ok())
      .filter_map(|e| {
        let path = e.path();
        if path.is_file() {
          let fname = path.file_name()?.to_string_lossy();
          if fname.starts_with(USER_LIST_FILE_NAME)
            && fname.ends_with(USER_LIST_FILE_EXT)
          {
            Some(path)
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();

    // 新しい順にソート
    csv_files
      .sort_by(|a, b| b.file_name().cmp(&a.file_name()));

    // 新しい順にロードを試みる
    for file_path in &csv_files {
      let mut users = Vec::new();
      let mut ulid_map = HashMap::new();
      let mut name_map = HashMap::new();
      let mut rdr = match ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
      {
        Ok(r) => r,
        Err(_) => continue,
      };
      let mut ok = true;
      for (i, result) in rdr.deserialize().enumerate() {
        match result {
          Ok(user) => {
            let user: AdminUser = user;
            ulid_map.insert(user.user_id, i);
            name_map.insert(user.user_id_string.clone(), i);
            users.push(Some(user));
          }
          Err(_) => {
            ok = false;
            break;
          }
        }
      }
      if ok {
        return Ok(Self { users, ulid_map, name_map });
      }
    }

    // ファイルがなければデフォルトユーザで新規作成
    let mut users = Vec::new();
    let mut ulid_map = HashMap::new();
    let mut name_map = HashMap::new();
    ulid_map.insert(default_user.user_id, 0);
    name_map.insert(default_user.user_id_string.clone(), 0);
    users.push(Some(default_user));
    Self::save_csv_internal(
      &users,
      config.csv_keep_count,
      &std::path::PathBuf::from(
        CONFIG.dashboard.user_list_dir.as_str(),
      ),
    )?;
    Ok(Self { users, ulid_map, name_map })
  }

  /// CSVファイルへ保存（タイムスタンプ付きで保存＆世代管理）
  fn save_csv_internal(
    users: &Vec<Option<AdminUser>>,
    keep_count: usize,
    dir: &PathBuf,
  ) -> std::io::Result<()> {
    // 新しいファイル名
    let new_file = dir.join(format!(
      "{}_{}.{}",
      USER_LIST_FILE_NAME,
      Utc::now().format("%Y%m%dT%H%M%SZ"),
      USER_LIST_FILE_EXT
    ));
    let mut wtr = WriterBuilder::new()
      .has_headers(true)
      .from_path(&new_file)?;
    for user in users.iter().filter_map(|u| u.as_ref()) {
      wtr.serialize(user)?;
    }
    wtr.flush()?;

    // 世代管理: keep_countを超えたら古いファイルを削除
    let mut csv_files: Vec<_> = fs::read_dir(dir)?
      .filter_map(|e| e.ok())
      .filter_map(|e| {
        let path = e.path();
        if path.is_file() {
          let fname = path.file_name()?.to_string_lossy();
          if fname.starts_with(USER_LIST_FILE_NAME)
            && fname.ends_with(USER_LIST_FILE_EXT)
          {
            Some(path)
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();
    csv_files
      .sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    if csv_files.len() > keep_count {
      for old in csv_files.iter().skip(keep_count) {
        let _ = fs::remove_file(old);
      }
    }
    Ok(())
  }

  /// CSVファイルへ保存（外部呼び出し用）
  pub fn save_csv(
    &self,
    _file_path: &PathBuf,
    users: &Vec<Option<AdminUser>>,
  ) -> std::io::Result<()> {
    // save_csv_internalを呼ぶ（file_path, keep_count, dirを渡す）
    let keep_count = crate::CONFIG.dashboard.csv_keep_count;
    Self::save_csv_internal(
      users,
      keep_count,
      &std::path::PathBuf::from(
        CONFIG.dashboard.user_list_dir.as_str(),
      ),
    )
  }

  /// ユーザ追加
  pub fn add_user(
    &mut self,
    user: AdminUser,
  ) -> std::io::Result<()> {
    let idx = self.users.len();
    self.ulid_map.insert(user.user_id, idx);
    self.name_map.insert(user.user_id_string.clone(), idx);
    self.users.push(Some(user));
    Self::save_csv_internal(
      &self.users,
      CONFIG.dashboard.csv_keep_count,
      &std::path::PathBuf::from(
        CONFIG.dashboard.user_list_dir.as_str(),
      ),
    )?;
    Ok(())
  }

  /// ユーザ検索（Ulid）
  pub fn get_by_ulid(
    &self,
    ulid: &ulid::Ulid,
  ) -> Option<&AdminUser> {
    self
      .ulid_map
      .get(ulid)
      .and_then(|&i| self.users.get(i)?.as_ref())
  }

  /// ユーザ検索（user_id_string）
  pub fn get_by_name(
    &self,
    name: &str,
  ) -> Option<&AdminUser> {
    self
      .name_map
      .get(name)
      .and_then(|&i| self.users.get(i)?.as_ref())
  }

  /// ユーザ削除
  pub fn remove_by_ulid(
    &mut self,
    ulid: &ulid::Ulid,
  ) -> std::io::Result<Option<AdminUser>> {
    if let Some(&idx) = self.ulid_map.get(ulid) {
      if let Some(user) = self.users.remove(idx) {
        self.name_map.remove(&user.user_id_string);
        self.ulid_map.remove(ulid);
        Self::save_csv_internal(
          &self.users,
          CONFIG.dashboard.csv_keep_count,
          &std::path::PathBuf::from(
            CONFIG.dashboard.user_list_dir.as_str(),
          ),
        )?;
        return Ok(Some(user));
      }
    }
    Ok(None)
  }

  /// 指定したindexのユーザで認証を行う
  ///
  /// ## Argument
  /// - `idx`: `usize`
  ///   - users配列のindex
  /// - `raw_password`: `&str`
  ///   - 入力されたパスワード
  ///
  /// ## Return value
  /// - `Result<bool, AdminUserError>`
  ///   - 認証成功: true, 失敗: false, エラー: AdminUserError
  pub async fn verify_by_index(
    &mut self,
    idx: usize,
    raw_password: &str,
  ) -> Result<bool, AdminUserError> {
    if let Some(Some(user)) = self.users.get_mut(idx) {
      let result = user.verify(raw_password)?;
      if result {
        // 認証成功時はCSVを保存し直す
        Self::save_csv_internal(
          &self.users,
          CONFIG.dashboard.csv_keep_count,
          &std::path::PathBuf::from(
            CONFIG.dashboard.user_list_dir.as_str(),
          ),
        )?;
      }
      Ok(result)
    } else {
      self.verify_fake_user(raw_password).await?;
      Err(AdminUserError::UserNotFound)
    }
  }

  /// Ulidで認証
  ///
  /// ## Argument
  /// - `ulid`: `&ulid::Ulid`
  ///   - ユーザのUlid
  /// - `raw_password`: `&str`
  ///   - 入力されたパスワード
  ///
  /// ## Return value
  /// - `Result<bool, AdminUserError>`
  pub async fn verify_by_ulid(
    &mut self,
    ulid: &ulid::Ulid,
    raw_password: &str,
  ) -> Result<bool, AdminUserError> {
    if let Some(&idx) = self.ulid_map.get(ulid) {
      self.verify_by_index(idx, raw_password).await
    } else {
      self.verify_fake_user(raw_password).await?;
      Err(AdminUserError::UserNotFound)
    }
  }

  /// user_id_stringで認証
  ///
  /// ## Argument
  /// - `name`: `&str`
  ///   - ユーザ名
  /// - `raw_password`: `&str`
  ///   - 入力されたパスワード
  ///
  /// ## Return value
  /// - `Result<bool, AdminUserError>`
  pub async fn verify_by_name(
    &mut self,
    name: &str,
    raw_password: &str,
  ) -> Result<bool, AdminUserError> {
    if let Some(&idx) = self.name_map.get(name) {
      self.verify_by_index(idx, raw_password).await
    } else {
      self.verify_fake_user(raw_password).await?;
      Err(AdminUserError::UserNotFound)
    }
  }

  /// fake_userで認証
  ///
  /// ## Argument
  /// - `raw_password`: `&str`
  ///  - 入力されたパスワード
  ///
  /// ## Return value
  /// - `Result<bool, AdminUserError>`
  ///  - 認証成功: true, 失敗: false, エラー: AdminUserError
  ///
  /// ## Summary
  /// - fake_userの認証成否は無意味
  pub async fn verify_fake_user(
    &mut self,
    raw_password: &str,
  ) -> Result<(), AdminUserError> {
    let mut fake_user = FAKE_USER.lock().await;
    fake_user.verify(raw_password)?;
    Ok(())
  }
}
