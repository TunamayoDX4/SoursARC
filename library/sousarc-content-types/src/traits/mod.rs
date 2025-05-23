//! SousARCデータ型トレイト群
//!
//! ## Summary
//! - `primitive.rs`: ID・キー・データ本体のトレイト定義
//! - `wrapper.rs`: ID・キー・データのラッパー構造体
//! - `topology.rs`: 親子関係を持つデータのためのマーカートレイト
//!
//! これらは相互に依存しつつ、柔軟なデータ構造を表現するための基盤となるよ！

use std::hash::Hash;

pub mod primitive;
//pub mod topology;

pub mod prelude {
  pub use super::{
    SousARCStorage, SousARCStorageMut,
    primitive::{
      IdKeySet, SousARCData, SousARCId, SousARCKey,
    },
    /*topology::{
      SousARCDataHasChild, SousARCDataHasParent,
      SousARCIdHasChild, SousARCKeyHasParent,
    },*/
  };
}

pub trait SousARCStorage<I: primitive::SousARCData> {
  /// データを取得する
  fn get(&self, id: I::Id) -> Option<&I>;

  fn get_by_key<Q: Eq + Hash>(&self, key: &Q) -> Option<&I>
  where
    I::Key: std::borrow::Borrow<Q>;

  fn id<Q: Eq + Hash>(&self, key: &Q) -> Option<I::Id>
  where
    I::Key: std::borrow::Borrow<Q>;

  fn key(&self, id: I::Id) -> Option<&I::Key>;
}

pub trait SousARCStorageMut<I: primitive::SousARCData>:
  SousARCStorage<I>
{
  fn get_mut(&mut self, id: I::Id) -> Option<&mut I>;
  fn get_by_key_mut<Q: Eq + Hash>(
    &mut self,
    key: &Q,
  ) -> Option<&mut I>
  where
    I::Key: std::borrow::Borrow<Q>;
}
