use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod security;

/// # User
/// ユーザデータ
///
/// - `id`: [`UserID`]  
///   ユーザのID
/// - `name_id`: `String`  
///   ユーザの識別名
/// - `secure_id`: [`UserSecureID`]
///   ユーザのセキュリティID
/// - `display_name`: `String`  
///   表示名
/// - `folder`: [`FolderID`]
///   ドキュメントのフォルダ
/// - `genre`: `Vec<[`GenreID`]>`
///   好きなジャンル
/// - `gen_genre`: `Vec<[`GenreID`]>`  
///   AI選択ジャンル
/// - `works`: `Vec<WorkID>`  
///   創作物IDリスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: UserID,
  pub name_id: String,
  pub secure_id: UserSecureID,
  pub display_name: String,
  pub folder: FolderID,
  pub genre: Vec<GenreID>,
  pub gen_genre: Vec<GenreID>,
  pub works: Vec<WorkID>,
}

impl ContentTrait for User {
  type ContentID = UserID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
