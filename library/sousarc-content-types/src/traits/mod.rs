//! SousARCデータ型トレイト群
//!
//! ## Summary
//! - `primitive.rs`: ID・キー・データ本体のトレイト定義
//! - `wrapper.rs`: ID・キー・データのラッパー構造体
//! - `topology.rs`: 親子関係を持つデータのためのマーカートレイト
//!
//! これらは相互に依存しつつ、柔軟なデータ構造を表現するための基盤となるよ！

use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub mod primitive;
pub mod topology;

pub use self::{primitive::*, topology::*};

/// IDとキーのペア
///
/// ## Summary
/// データ型Iに対するID・キーのセット
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SousARCIdentPair<I: SousARCData> {
  /// データID
  pub id: I::Id,
  /// データキー
  pub key: I::Key,
}

/// データラッパー
///
/// ## Summary
/// ID・キー・データ本体をまとめて格納するラッパー構造体
#[derive(Debug, Clone)]
pub struct SousARCDataWrap<I, W>
where
  I: SousARCData,
  W: From<I>,
{
  /// ID・キーのペア
  pub ident: SousARCIdentPair<I>,
  /// データ本体
  pub data: W,
}
impl<I, W> SousARCDataWrap<I, W>
where
  I: SousARCData,
  W: From<I>,
{
  pub fn id(&self) -> I::Id {
    self.ident.id
  }

  pub fn key(&self) -> &I::Key {
    &self.ident.key
  }
}

pub trait SousARCStorage<
  I: primitive::SousARCData,
  W: From<I>,
>
{
  /// データを取得する
  fn get(&self, id: I::Id) -> Option<&W>;

  fn get_by_key<Q: Eq + Hash>(&self, key: &Q) -> Option<&W>
  where
    I::Key: std::borrow::Borrow<Q>;

  fn id<Q: Eq + Hash>(&self, key: &Q) -> Option<I::Id>
  where
    I::Key: std::borrow::Borrow<Q>;

  fn key(&self, id: I::Id) -> Option<&I::Key>;
}

pub trait SousARCStorageMut<
  I: primitive::SousARCData,
  W: From<I>,
>: SousARCStorage<I, W>
{
  fn get_mut(&mut self, id: I::Id) -> Option<&mut W>;
  fn get_by_key_mut<Q: Eq + Hash>(
    &mut self,
    key: &Q,
  ) -> Option<&mut W>
  where
    I::Key: std::borrow::Borrow<Q>;
}
