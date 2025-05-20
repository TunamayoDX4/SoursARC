//! 作品（ワーク）関連の型・ロジック
//!
//! ## Summary
//! SoursARCの作品情報・作品IDなどを定義するモジュール。

use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::prelude::*;
use crate::traits::*;

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct WorkId(Ulid);
impl WorkId {
  /// 新しい作品IDを生成する
  pub fn new() -> Self {
    Self(Ulid::new())
  }
}
impl SousARCId for WorkId {
  type Bound = WorkData;
}

impl SousARCIdHasChild<WorkData, ElementData, ElementKey>
  for WorkId
{
}

impl ToString for WorkId {
  fn to_string(&self) -> String {
    self.0.to_string()
  }
}

#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct WorkKey {
  pub user_id: UserId,
  pub work_name: String,
}
impl SousARCKey for WorkKey {
  type Bound = WorkData;
}

impl ToString for WorkKey {
  fn to_string(&self) -> String {
    self.user_id.to_string() + self.work_name.as_str()
  }
}

impl SousARCKeyHasParent<UserData, WorkData, UserId>
  for WorkKey
{
  fn parent_id(&self) -> impl Iterator<Item = UserId> {
    [self.user_id].into_iter()
  }
}

/// 作品
///
/// ## Summary
/// SoursARCの作品情報
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkData {
  /// 子要素となる要素のID
  pub elements: Vec<ElementId>,

  /// 表示される名前
  pub display_name: String,

  /// ユーザにより設定された内容
  pub description: String,

  /// 変更履歴
  pub history: Vec<String>,

  /// 作品のAI生成メタデータ
  pub gen_meta: GeneratedWorkMeta,
}
impl SousARCData for WorkData {
  type Id = WorkId;
  type Key = WorkKey;
}

impl SousARCDataHasChild<ElementData, WorkId, ElementKey>
  for WorkData
{
  fn children(&self) -> impl Iterator<Item = ElementId> {
    self.elements.iter().copied()
  }
}

/// 作品に関するAI生成メタデータ
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedWorkMeta {
  /// 作品の要約
  pub summary: String,

  /// 作品のキーワード
  pub keywords: Vec<String>,

  /// 作品のテーマ
  pub theme: String,

  /// 作品のジャンル
  pub genre: String,
}
