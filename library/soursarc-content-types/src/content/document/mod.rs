use crate::{content_traits::ContentTrait, id::*};
use serde::{Deserialize, Serialize};

/// # Document
/// ドキュメントデータ
///
/// - `id`: [`DocumentID`]  
///   ドキュメントID
/// - `parent`: [`FolderID`]  
///   親フォルダID
/// - `title`: `String`  
///   タイトル
/// - `body`: `String`  
///   本文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
  pub id: DocumentID,
  pub parent: FolderID,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名（タイトル）
  pub display_name: String,
  /// 要約
  pub summary: String,
  /// 要点(カンマ区切り)
  pub topics: String,
  /// データ本文
  pub body: String,
}

impl ContentTrait for Document {
  type ContentID = DocumentID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> std::borrow::Cow<str> {
    std::borrow::Cow::Borrowed(&self.name_id)
  }
}
