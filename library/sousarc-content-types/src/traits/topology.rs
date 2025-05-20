//! SousARCデータの親子関係マーカートレイト
//!
//! ## Summary
//! - データ型間の親子・階層関係を型レベルで表現するためのトレイト群
//! - `primitive.rs`のID/Key/Dataトレイトと相互依存
//!
//! これでツリー・グラフ構造も型安全に表現できるよ！

use super::{SousARCStorage, primitive::*};

/// 子要素を持つデータ型のトレイト
///
/// ## Summary
/// 子要素（ID）を返すデータ型のためのトレイト
pub trait SousARCDataHasChild<C, Pi, Ck>
where
  Self: SousARCData,
  C: SousARCDataHasParent<Self, Pi, Ck>,
  Pi: SousARCIdHasChild<Self, C, Ck>,
  Ck: SousARCKeyHasParent<Self, C, Pi>,
{
  /// 子要素IDのイテレータを返す
  fn children(&self) -> impl Iterator<Item = C::Id>;

  /// 子要素キーのイテレータを返す
  fn children_key<'a, W: From<C>>(
    &self,
    storage: &'a impl SousARCStorage<C, W>,
  ) -> impl Iterator<Item = &'a C::Key> {
    self.children().filter_map(|id| storage.key(id))
  }
}

/// 子要素を持つデータ型のIDのマーカートレイト
///
/// ## Summary
/// 親→子のID関係を型レベルで表現
pub trait SousARCIdHasChild<P, C, Ck>
where
  P: SousARCDataHasChild<C, Self, Ck>,
  C: SousARCDataHasParent<P, Self, Ck>,
  Self: SousARCId<Bound = P>,
  Ck: SousARCKeyHasParent<P, C, Self>,
{
}
/*
/// 自動実装
impl<P, C, Pi, Ck> SousARCIdHasChild<P, C, Ck> for Pi
where
  P: SousARCDataHasChild<C, Self, Ck>,
  C: SousARCDataHasParent<P, Self, Ck>,
  Pi: SousARCId<Bound = P>,
  Ck: SousARCKeyHasParent<P, C, Self>,
{
}
*/

/// 親要素を持つデータ型のマーカートレイト
///
/// ## Summary
/// 子→親のデータ型関係を型レベルで表現
pub trait SousARCDataHasParent<P, Pi, Ck>
where
  P: SousARCDataHasChild<Self, Pi, Ck>,
  Self: SousARCData,
  Pi: SousARCIdHasChild<P, Self, Ck>,
  Ck: SousARCKeyHasParent<P, Self, Pi>,
{
}
/// 自動実装
impl<P, C, Pi, Ck> SousARCDataHasParent<P, Pi, Ck> for C
where
  P: SousARCDataHasChild<C, Pi, Ck>,
  C: SousARCData,
  Pi: SousARCIdHasChild<P, C, Ck>,
  Ck: SousARCKeyHasParent<P, C, Pi>,
{
}

/// 親要素を持つデータ型のキーのトレイト
///
/// ## Summary
/// 子→親のキー関係を型レベルで表現
pub trait SousARCKeyHasParent<P, C, Pi>
where
  P: SousARCDataHasChild<C, Pi, Self>,
  C: SousARCDataHasParent<P, Pi, Self>,
  Pi: SousARCIdHasChild<P, C, Self>,
  Self: SousARCKey<Bound = C>,
{
  /// 親IDのイテレータを返す
  fn parent_id(&self) -> impl Iterator<Item = P::Id>;

  /// 親キーのイテレータを返す
  fn parent_key<'a, W: From<P>>(
    &self,
    storage: &'a impl SousARCStorage<P, W>,
  ) -> impl Iterator<Item = &'a P::Key> {
    self.parent_id().filter_map(|id| storage.key(id))
  }
}
