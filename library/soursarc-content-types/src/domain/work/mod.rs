use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// # Work
/// 創作物データ
///
/// - `id`: [`WorkID`]  
///   創作物ID
/// - `author`: [`UserID`]  
///   作者ID
/// - `name_id`: `String`  
///   創作物の文字列ID（ユニークな識別子、Storageのキーに使う）
/// - `display_name`: `String`  
///   創作物の表示名
/// - `summary`: `String`  
///   要約
/// - `genres`: `Vec<GenreID>`  
///   ジャンルIDリスト
/// - `elements`: `Vec<ElementID>`  
///   要素IDリスト
/// - `folder`: [`FolderID`]  
///   配下フォルダID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
  pub id: WorkID,
  pub author: UserID,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名
  pub display_name: String,
  pub summary: String,
  pub genres: Vec<GenreID>,
  pub elements: Vec<ElementID>,
  pub folder: FolderID,
}

impl ContentTrait for Work {
  type ContentID = WorkID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
