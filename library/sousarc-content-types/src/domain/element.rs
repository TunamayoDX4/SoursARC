//! 要素（キャラクター・国・文化など）関連の型・ロジック
//!
//! ## Summary
//! SoursARCの世界観要素・IDなどを定義するモジュール。

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
pub struct ElementId(Ulid);
impl ElementId {
  /// 新しい要素IDを生成する
  pub fn new() -> Self {
    Self(Ulid::new())
  }
}
impl SousARCId for ElementId {
  type Bound = ElementData;
}
impl SousARCIdHasChild<ElementData, ElementData, ElementKey>
  for ElementId
{
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
pub struct ElementKey {
  pub work_id: WorkId,
  pub parents: Option<Vec<ElementId>>,
  pub element_name: String,
}
impl SousARCKey for ElementKey {
  type Bound = ElementData;
}

impl SousARCKeyHasParent<WorkData, ElementData, WorkId>
  for ElementKey
{
  fn parent_id(&self) -> impl Iterator<Item = WorkId> {
    [self.work_id].into_iter()
  }
}
impl SousARCKeyHasParent<ElementData, ElementData, ElementId>
  for ElementKey
{
  fn parent_id(&self) -> impl Iterator<Item = ElementId> {
    self
      .parents
      .as_ref()
      .map(|ids| ids.iter().copied())
      .unwrap_or_default()
  }
}

/// 要素
///
/// ## Summary
/// SoursARCのキャラクター・国・文化などの設定データ
#[derive(Debug, Serialize, Deserialize)]
pub struct ElementData {
  /// 子要素のID
  pub children: Vec<ElementId>,

  /// 表示される名前
  pub display_name: String,

  /// ユーザにより設定された内容
  pub content: String,

  /// 変更履歴
  pub history: Vec<String>,

  /// 要素に関するAI生成メタデータ
  pub gen_meta: GeneratedElementMeta,
}
impl SousARCData for ElementData {
  type Id = ElementId;
  type Key = ElementKey;
}

impl SousARCDataHasChild<ElementData, ElementId, ElementKey>
  for ElementData
{
  fn children(&self) -> impl Iterator<Item = ElementId> {
    self.children.iter().copied()
  }
}

/// 要素に関するAI生成メタデータ
///
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedElementMeta {
  /// 要素の要約
  pub summary: String,

  /// 要素の要点リスト(カンマ区切り)
  pub topics: String,

  /// 要素のキーワード(カンマ区切り)
  pub keywords: String,
}
