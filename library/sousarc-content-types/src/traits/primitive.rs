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
use std::{fmt::Debug, hash::Hash};

use super::SousARCStorage;

/// データIDトレイト
///
/// ## Summary
/// 各データ型の一意なIDを表現するためのトレイト
pub trait SousARCId:
  Debug
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

  fn get_key<'a, W: From<Self::Bound>>(
    &self,
    storage: &'a impl SousARCStorage<Self::Bound, W>,
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

  fn get_id<W: From<Self::Bound>>(
    &self,
    storage: &impl SousARCStorage<Self::Bound, W>,
  ) -> Option<<Self::Bound as SousARCData>::Id> {
    storage.id(self)
  }
}

/// データ本体トレイト
///
/// ## Summary
/// ID・キーを持つデータ型のためのトレイト
pub trait SousARCData:
  Debug
  + Send
  + Sync
  + Serialize
  + for<'de> Deserialize<'de>
  + 'static
{
  /// データID型
  type Id: SousARCId;
  /// データキー型
  type Key: SousARCKey;
}
