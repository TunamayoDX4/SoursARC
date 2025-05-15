use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// # Genre
/// ジャンルデータ
///
/// - `id`: [`GenreID`]  
///   ジャンルID
/// - `name`: `String`  
///   ジャンル名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
  pub id: GenreID,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名
  pub display_name: String,
  /// ジャンルの要約
  pub summary: String,
  /// ジャンルの要約(生成用)
  pub generated_summary: String,
}

impl ContentTrait for Genre {
  type ContentID = GenreID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
