use std::{
  hash::Hash,
  ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

/// SoursARCのインスタンスを表すトレイト
///
/// ## Associated Types
/// - `Id`   : インスタンスのID型
/// - `Name` : インスタンスの名前型
pub trait SoursARCInstance
where
  Self: Sized + Serialize + for<'de> Deserialize<'de>,
{
  type Id: SoursARCId;
  type Name: SoursARCName;

  /// インスタンスのIDを取得する
  fn id(&self) -> &Self::Id;

  /// インスタンスの名前を取得する
  fn name(&self) -> &Self::Name;
}

/// SoursARCのID型に必要なトレイト
///
/// ## Associated Types
/// - `Bound` : このIDが紐づくインスタンス型
pub trait SoursARCId
where
  Self: Copy
    + Ord
    + Hash
    + Serialize
    + for<'de> Deserialize<'de>,
{
  type Bound: SoursARCInstance<Id = Self>;

  /// ストレージから名前を取得する
  ///
  /// ## Argument
  /// - `storage` : 検索対象のストレージ
  ///
  /// ## Return value
  /// - 名前(`Name`)への参照
  fn get_name<'a>(
    &self,
    storage: &'a impl SoursARCStorage<Self::Bound>,
  ) -> Option<
    &'a <<Self as SoursARCId>::Bound as SoursARCInstance>::Name,
  >{
    storage.get_name_by_id(self)
  }
}

/// SoursARCの名前型に必要なトレイト
///
/// ## Associated Types
/// - `Bound` : この名前が紐づくインスタンス型
pub trait SoursARCName
where
  Self: Clone
    + Ord
    + Hash
    + Serialize
    + for<'de> Deserialize<'de>,
{
  type Bound: SoursARCInstance<Name = Self>;

  /// ストレージからIDを取得する
  ///
  /// ## Argument
  /// - `storage` : 検索対象のストレージ
  ///
  /// ## Return value
  /// - ID
  fn get_id(
    &self,
    storage: &impl SoursARCStorageReverseRef<Self::Bound>,
  ) -> Option<
    <<Self as SoursARCName>::Bound as SoursARCInstance>::Id,
  > {
    storage.get_id_by_name(self)
  }
}

/// SoursARCのストレージ操作用トレイト(参照のみ)
///
/// ## Associated Types
/// - `Data<'a>` : データへの参照型
pub trait SoursARCStorage<I: SoursARCInstance> {
  type Data<'a>: Deref<Target = I> + 'a
  where
    Self: 'a;

  /// IDからデータを取得する
  fn get<'a>(&'a self, id: I::Id) -> Option<Self::Data<'a>>;

  /// 名前からデータを取得する
  fn get_by_name<'a>(
    &'a self,
    name: &I::Name,
  ) -> Option<Self::Data<'a>>;

  /// IDから名前を取得する
  fn get_name_by_id<'a>(
    &'a self,
    id: &I::Id,
  ) -> Option<&'a I::Name>;
}

/// 名前からIDを逆引きするためのトレイト
pub trait SoursARCStorageReverseRef<I: SoursARCInstance>:
  SoursARCStorage<I>
{
  /// 名前からIDを取得する
  fn get_id_by_name<'a>(
    &'a self,
    name: &I::Name,
  ) -> Option<I::Id>;
}

/// ミュータブルなストレージ操作用トレイト
///
/// ## Associated Types
/// - `DataMut<'a>` : データへの可変参照型
pub trait SoursARCStorageMut<I: SoursARCInstance>:
  SoursARCStorage<I>
{
  type DataMut<'a>: DerefMut<Target = I> + 'a
  where
    Self: 'a;

  /// IDからデータの可変参照を取得する
  fn get_mut<'a>(
    &'a mut self,
    id: I::Id,
  ) -> Option<Self::DataMut<'a>>;

  /// 名前からデータの可変参照を取得する
  fn get_mut_by_name<'a>(
    &'a mut self,
    name: &I::Name,
  ) -> Option<Self::DataMut<'a>>;
}

/// 親要素を持つインスタンスのマーカートレイト
///
pub trait SoursARCInstanceHasParent<Ip>:
  SoursARCInstance
where
  Ip: SoursARCInstanceHasChild<Self>,
{
}

/// 子要素を持つインスタンスのマーカートレイト
///
pub trait SoursARCInstanceHasChild<Ic>:
  SoursARCInstance
where
  Ic: SoursARCInstanceHasParent<Self>,
{
  fn child(&self) -> impl Iterator<Item = Ic::Id>;
}

// 親要素を持つインスタンス名のマーカートレイト
pub trait SoursARCNameHasParent<Ip>: SoursARCName
where
  Ip:
    SoursARCInstanceHasChild<<Self as SoursARCName>::Bound>,
  <Self as SoursARCName>::Bound:
    SoursARCInstanceHasParent<Ip>,
{
  fn parent_id(&self) -> impl Iterator<Item = Ip::Id>;
  fn parent_name<'a>(
    &'a self,
    storage: &'a impl SoursARCStorage<Ip>,
  ) -> impl Iterator<Item = &'a Ip::Name> + 'a
  where
    Ip: 'a,
  {
    self
      .parent_id()
      .filter_map(|id| storage.get_name_by_id(&id))
  }
}

/// 子要素を持つインスタンスIDのマーカートレイト
pub trait SoursARCIdHasChild<Ic>: SoursARCId
where
  Ic: SoursARCInstanceHasParent<<Self as SoursARCId>::Bound>,
  <Self as SoursARCId>::Bound: SoursARCInstanceHasChild<Ic>,
{
}
impl<Ic, Ip, Iid> SoursARCIdHasChild<Ic> for Iid
where
  Ic: SoursARCInstanceHasParent<Ip>,
  Ip: SoursARCInstanceHasChild<Ic>,
  Iid: SoursARCId<Bound = Ip>,
{
}
