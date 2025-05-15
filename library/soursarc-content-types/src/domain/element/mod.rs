use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// # ElementParent
/// 要素の親を示すenum
///
/// - `Work(WorkID)` : Work直下
/// - `Element(ElementID)` : Element直下
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
pub enum ElementParent {
  Work(WorkID),
  Element(ElementID),
}

/// # Element
/// 要素データ
///
/// - `id`: [`ElementID`]  
///   要素ID
/// - `parent`: [`ElementParent`]  
///   親要素
/// - `depth`: `u32`  
///   深度
/// - `name`: `String`  
///   要素名
/// - `summary`: `String`  
///   要約
/// - `categories`: `Vec<CategoryID>`  
///   カテゴリIDリスト
/// - `elements`: `Vec<ElementID>`  
///   子要素IDリスト
/// - `folder`: [`FolderID`]  
///   配下フォルダID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
  pub id: ElementID,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名
  pub display_name: String,
  pub parent: ElementParent,
  pub depth: u32,
  pub name: String,
  pub summary: String,
  pub categories: Vec<CategoryID>,
  pub elements: Vec<ElementID>,
  pub folder: FolderID,
}

impl ContentTrait for Element {
  type ContentID = ElementID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
