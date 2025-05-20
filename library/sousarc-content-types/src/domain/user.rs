//! ユーザ関連の型・ロジック
//!
//! ## Summary
//! SoursARCのユーザ情報・ユーザIDなどを定義するモジュール。

use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::{prelude::*, work::WorkKey};
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
/// ユーザID
///
/// ## Summary
/// SoursARC内で一意となるユーザ識別子
pub struct UserId(Ulid);
impl UserId {
  /// 新しいユーザIDを生成する
  pub fn new() -> Self {
    Self(Ulid::new())
  }
}
impl SousARCId for UserId {
  type Bound = UserData;
}

impl SousARCIdHasChild<UserData, WorkData, WorkKey>
  for UserId
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
pub struct UserKey(String);
impl UserKey {
  /// 新しいユーザキーを生成する
  pub fn new(user_name: impl ToString) -> Self {
    Self(user_name.to_string())
  }
}
impl SousARCKey for UserKey {
  type Bound = UserData;
}
#[derive(Debug, Serialize, Deserialize)]
/// ユーザ
///
/// ## Summary
/// SoursARCのユーザ情報
pub struct UserData {
  /// 子要素となるワークのID
  pub works: Vec<WorkId>,

  /// 表示される名前
  pub display_name: String,

  /// ユーザによる自己紹介
  pub introduction: String,

  /// ユーザのEメールアドレス
  pub email: String,
}
impl SousARCData for UserData {
  type Id = UserId;
  type Key = UserKey;
}

impl SousARCDataHasChild<WorkData, UserId, WorkKey>
  for UserData
{
  fn children(&self) -> impl Iterator<Item = WorkId> {
    self.works.iter().copied()
  }
}
