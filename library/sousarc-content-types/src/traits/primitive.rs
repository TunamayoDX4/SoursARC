//! SousARCデータ型のプリミティブトレイト
//!
//! ## Summary
//! - データID（`SousARCId`）
//! - データキー（`SousARCKey`）
//! - データ本体（`SousARCData`）
//! を定義するモジュール。
//!
//! これらは`topology.rs`や`wrapper.rs`からも参照される基盤だよ！

use serde::{Deserialize, Serialize};
use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

use super::SousARCStorage;

/// データIDとキーのセット
///
/// ## Summary
/// データIDとキーを組み合わせた構造体
/// これを使うことで、IDとキーの両方を持つデータを一意に識別できる
#[derive(
  Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub struct IdKeySet<D: SousARCData> {
  id: D::Id,
  key: D::Key,
}
impl<D: SousARCData> IdKeySet<D> {
  /// 新しい`IdKeySet`を作成する
  pub fn new(id: D::Id, key: D::Key) -> Self {
    Self { id, key }
  }

  /// IDを取得する
  pub fn id(&self) -> D::Id {
    self.id
  }

  /// キーを取得する
  pub fn key(&self) -> &D::Key {
    &self.key
  }
}

/// データIDトレイト
///
/// ## Summary
/// 各データ型の一意なIDを表現するためのトレイト
pub trait SousARCId:
  Debug
  + Display
  + Send
  + Sync
  + Copy
  + Ord
  + Hash
  + Serialize
  + for<'de> Deserialize<'de>
  + 'static
{
  /// このIDが紐づくデータ型
  type Bound: SousARCData<Id = Self>;

  fn get_key<'a>(
    &self,
    storage: &'a impl SousARCStorage<Self::Bound>,
  ) -> Option<&'a <Self::Bound as SousARCData>::Key> {
    storage.key(*self)
  }
}

/// データキー（主キーや複合キーなど）
///
/// ## Summary
/// データの検索や親子関係に使うキーを表現するトレイト
pub trait SousARCKey:
  Debug
  + Display
  + Send
  + Sync
  + Clone
  + Ord
  + Hash
  + Serialize
  + for<'de> Deserialize<'de>
  + 'static
{
  /// このキーが紐づくデータ型
  type Bound: SousARCData<Key = Self>;

  fn get_id(
    &self,
    storage: &impl SousARCStorage<Self::Bound>,
  ) -> Option<<Self::Bound as SousARCData>::Id> {
    storage.id(self)
  }
}

/// データ本体トレイト
///
/// ## Summary
/// ID・キーを持つデータ型のためのトレイト
pub trait SousARCData:
  Sized + Send + Sync + 'static
{
  /// データID型
  type Id: SousARCId;
  /// データキー型
  type Key: SousARCKey;

  /// IDとキーのセットを取得する
  fn id_key_set(&self) -> &IdKeySet<Self>;

  /// IDを取得する
  fn id(&self) -> Self::Id {
    self.id_key_set().id()
  }

  /// キーを取得する
  fn key(&self) -> &Self::Key {
    self.id_key_set().key()
  }
}
