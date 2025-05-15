use crate::content_traits::ContentTrait;
use crate::id::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// # FolderParent
/// フォルダの親を示すenum
///
/// - `User(UserID)`
/// - `Work(WorkID)`
/// - `Element(ElementID)`
/// - `Genre(GenreID)`
/// - `Category(CategoryID)`
/// - `Folder(FolderID)`
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
pub enum FolderParent {
  User(UserID),
  Work(WorkID),
  Element(ElementID),
  Genre(GenreID),
  Category(CategoryID),
  Folder(FolderID),
}

/// # FolderType
/// フォルダの種類を示すenum
///
/// - `Simple(Vec<FolderID>)`
///   単純なフォルダ
/// - `List(Vec<(String, FolderID)>)`
///   リストフォルダ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FolderType {
  Simple(Vec<FolderChild>),
  List(Vec<(String, FolderChild)>),
}

/// # FolderChild
/// フォルダの子を示すenum
///   
/// - `Document(DocumentID)`
/// - `Folder(FolderID)`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FolderChild {
  Document(DocumentID),
  Folder(FolderID),
}

/// # Folder
/// フォルダデータ
///
/// - `id`: [`FolderID`]  
///   フォルダID
/// - `parent`: [`FolderParent`]  
///   親フォルダ
/// - `name`: `String`  
///   フォルダ名
/// - `index`: [`DocumentID`]  
///   インデックスとなるドキュメントID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
  pub id: FolderID,
  pub parent: FolderParent,
  /// 文字列ID（ユニークな識別子、Storageのキーに使う）
  pub name_id: String,
  /// 表示名
  pub display_name: String,
  /// インデックスとなるドキュメントのID
  /// ID of the document that serves as the index
  pub index: DocumentID,

  /// フォルダの種類・子要素
  /// Type of folder and child elements
  pub children: FolderType,
}

impl ContentTrait for Folder {
  type ContentID = FolderID;

  fn id(&self) -> Self::ContentID {
    self.id
  }

  /// 文字列ID（Storageのキーに使う）
  fn name_id(&self) -> Cow<str> {
    Cow::Borrowed(&self.name_id)
  }
}
