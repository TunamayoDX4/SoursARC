pub mod global;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

/// # UserSecureID
/// ユーザのセキュリティデータを一意に識別するID
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
pub struct UserSecureID(Ulid);

/// # UserID
/// ユーザを一意に識別するID
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
pub struct UserID(Ulid);

/// # WorkID
/// 創作物を一意に識別するID
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
pub struct WorkID(Ulid);

/// # ElementID
/// 要素を一意に識別するID
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
pub struct ElementID(Ulid);

/// # GenreID
/// ジャンルを一意に識別するID
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
pub struct GenreID(Ulid);

/// # CategoryID
/// カテゴリを一意に識別するID
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
pub struct CategoryID(Ulid);

/// # FolderID
/// フォルダを一意に識別するID
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
pub struct FolderID(Ulid);

/// # DocumentID
/// ドキュメントを一意に識別するID
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
pub struct DocumentID(Ulid);
