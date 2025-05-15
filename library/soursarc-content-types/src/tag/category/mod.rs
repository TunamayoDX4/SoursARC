use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// # Category
/// カテゴリデータ
///
/// - `id`: [`CategoryID`]  
///   カテゴリID
/// - `name`: `String`  
///   カテゴリ名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
  pub id: CategoryID,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名
  pub display_name: String,

  /// カテゴリのドキュメントのID
  pub folder: FolderID,
}

impl ContentTrait for Category {
  type ContentID = CategoryID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
