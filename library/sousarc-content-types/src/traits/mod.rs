//! SousARCデータ型トレイト群
//!
//! ## Summary
//! - `primitive.rs`: ID・キー・データ本体のトレイト定義
//! - `wrapper.rs`: ID・キー・データのラッパー構造体
//! - `topology.rs`: 親子関係を持つデータのためのマーカートレイト
//!
//! これらは相互に依存しつつ、柔軟なデータ構造を表現するための基盤となるよ！

use serde::{
  Deserialize, Serialize, de::Visitor, ser::SerializeStruct,
};
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
impl<I, W> Serialize for SousARCDataWrap<I, W>
where
  I: SousARCData,
  W: From<I> + Serialize,
{
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state =
      serializer.serialize_struct("SousARCDataWrap", 2)?;
    state.serialize_field("ident", &self.ident)?;
    state.serialize_field("data", &self.data)?;
    state.end()
  }
}
impl<'de, I, W> Deserialize<'de> for SousARCDataWrap<I, W>
where
  I: SousARCData,
  W: From<I> + Deserialize<'de>,
{
  fn deserialize<D>(
    deserializer: D,
  ) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct SousARCDataWrapVisitor<I, W>
    where
      I: SousARCData,
      W: From<I>,
    {
      marker: std::marker::PhantomData<(I, W)>,
    }
    impl<'de, I, W> Visitor<'de> for SousARCDataWrapVisitor<I, W>
    where
      I: SousARCData,
      W: From<I> + Deserialize<'de>,
    {
      type Value = SousARCDataWrap<I, W>;

      fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
      ) -> std::fmt::Result {
        formatter.write_str("struct SousARCDataWrap")
      }

      fn visit_map<V>(
        self,
        mut map: V,
      ) -> Result<Self::Value, V::Error>
      where
        V: serde::de::MapAccess<'de>,
      {
        let mut ident = None;
        let mut data = None;
        while let Some(key) = map.next_key()? {
          match key {
            "ident" => {
              if ident.is_some() {
                return Err(
                  serde::de::Error::duplicate_field("ident"),
                );
              }
              ident = Some(map.next_value()?);
            }
            "data" => {
              if data.is_some() {
                return Err(
                  serde::de::Error::duplicate_field("data"),
                );
              }
              data = Some(map.next_value()?);
            }
            _ => {
              return Err(serde::de::Error::unknown_field(
                key,
                &["ident", "data"],
              ));
            }
          }
        }
        let ident = ident.ok_or_else(|| {
          serde::de::Error::missing_field("ident")
        })?;
        let data = data.ok_or_else(|| {
          serde::de::Error::missing_field("data")
        })?;
        Ok(SousARCDataWrap { ident, data })
      }
    }
    deserializer.deserialize_struct(
      "SousARCDataWrap",
      &["ident", "data"],
      SousARCDataWrapVisitor::<I, W> {
        marker: std::marker::PhantomData,
      },
    )
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
