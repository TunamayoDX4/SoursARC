pub mod arc;

use crate::content_traits::ContentTrait;
use std::collections::{HashMap, VecDeque};

/// # StorageCapMode
///
/// ## Summary
/// ストレージのキャッシュ戦略を指定するenum。
/// - `Free` : キャッシュ制限なし
/// - `Limited` : キャッシュ容量を指定し、キャッシュアルゴリズムを適用
#[derive(Debug, PartialEq, Eq)]
pub enum StorageCapMode<C: Cache<T>, T: PartialEq + Copy> {
  /// キャッシュ制限なし
  Free,
  /// キャッシュ容量制限あり
  Limited { _dum: std::marker::PhantomData<T>, cache: C },
}

impl<C, T> Cache<T> for StorageCapMode<C, T>
where
  C: Cache<T>,
  T: PartialEq + Copy,
{
  /// インデックスからキャッシュ内容を取得
  fn get(&self, id: usize) -> Option<&T> {
    match self {
      StorageCapMode::Free => None,
      StorageCapMode::Limited { cache, .. } => cache.get(id),
    }
  }

  /// キャッシュ容量を取得
  fn capacity(&self) -> usize {
    match self {
      StorageCapMode::Free => 0,
      StorageCapMode::Limited { cache, .. } => {
        cache.capacity()
      }
    }
  }

  /// キャッシュを更新（touch）
  fn touch(&mut self, id: T) -> Option<T> {
    match self {
      StorageCapMode::Free => None,
      StorageCapMode::Limited { cache, .. } => {
        cache.touch(id)
      }
    }
  }

  /// キャッシュから削除
  fn remove(&mut self, id: T) -> Option<T> {
    match self {
      StorageCapMode::Free => None,
      StorageCapMode::Limited { cache, .. } => {
        cache.remove(id)
      }
    }
  }
}

/// # Cache
///
/// ## Summary
/// キャッシュアルゴリズムのためのトレイト。
pub trait Cache<T>
where
  T: PartialEq + Copy,
{
  /// インデックスからキャッシュ内容を取得
  fn get(&self, id: usize) -> Option<&T>;
  /// キャッシュ容量を取得
  fn capacity(&self) -> usize;
  /// キャッシュを更新（touch）
  fn touch(&mut self, id: T) -> Option<T>;
  /// キャッシュから削除
  fn remove(&mut self, id: T) -> Option<T>;
}

/// # ResizableCache
///
/// ## Summary
/// キャッシュ容量を動的に変更可能なキャッシュ用トレイト。
pub trait ResizableCache<T>: Cache<T>
where
  T: PartialEq + Copy,
{
  /// キャッシュ容量を変更
  fn resize(&mut self, new_capacity: usize) -> bool;
}

/// # StorageInsertResult
///
/// ## Summary
/// ストレージにデータを追加した結果を表すenum。
#[derive(Debug, PartialEq, Eq)]
pub enum StorageInsertResult<T> {
  /// 既存データと被ったので上書き
  Replaced(T),
  /// キャッシュから追い出された
  OutOfCache(T),
  /// 空きスロットに追加できた
  Inserted(usize),
}

/// # Storage
///
/// ## Summary
/// ContentTraitを満たすデータを格納・検索・キャッシュするストレージ構造体。
pub struct Storage<T: ContentTrait> {
  /// データ本体（Vec<Option<T>>で管理）
  pub data: Vec<Option<T>>,
  /// IDをキーにしたインデックス
  pub by_id: HashMap<<T as ContentTrait>::ContentID, usize>,
  /// name_idをキーにしたインデックス
  pub by_name_id: HashMap<String, usize>,
  /// 空きスロットのインデックス
  free_list: VecDeque<usize>,
  /// キャッシュ戦略
  pub cap_mode: StorageCapMode<arc::ARC<usize>, usize>,
}

impl<T: ContentTrait> Storage<T> {
  /// 新しいStorageを作成
  ///
  /// ## Argument
  /// - `capacity`: Option<usize> (Noneなら無制限、Someならキャッシュ容量)
  ///
  /// ## Return value
  /// - `Storage<T>` : 新しく生成されたストレージ
  pub fn new(capacity: Option<usize>) -> Self {
    let cap_mode = match capacity {
      Some(cap) => StorageCapMode::Limited {
        _dum: std::marker::PhantomData,
        cache: arc::ARC::new(cap),
      },
      None => StorageCapMode::Free,
    };
    Self {
      data: Vec::new(),
      by_id: HashMap::new(),
      by_name_id: HashMap::new(),
      free_list: VecDeque::new(),
      cap_mode,
    }
  }

  /// データを追加
  ///
  /// ## Argument
  /// - `item`: 追加するデータ
  ///
  /// ## Return value
  /// - `StorageInsertResult<T>` : 追加結果
  pub fn insert(
    &mut self,
    item: T,
  ) -> StorageInsertResult<T> {
    let id = item.id();
    let name_id = item.name_id().to_string();

    // 既存なら上書き
    if let Some(&idx) = self.by_id.get(&id) {
      let old = self.data[idx].replace(item);
      self.by_name_id.insert(name_id, idx);
      self.cap_mode.touch(idx);
      return StorageInsertResult::Replaced(old.unwrap());
    }

    // 空きスロットがあれば再利用
    let idx =
      if let Some(free_idx) = self.free_list.pop_front() {
        self.data[free_idx] = Some(item);
        free_idx
      } else {
        self.data.push(Some(item));
        self.data.len() - 1
      };

    self.by_id.insert(id, idx);
    self.by_name_id.insert(name_id, idx);
    // キャッシュに追加し、溢れたらOutOfCacheで返す
    if let Some(evicted) = self.cap_mode.touch(idx) {
      StorageInsertResult::OutOfCache(
        // キャッシュから追い出されたものを返す
        // 追い出されたものはキャッシュから削除する
        self.remove_by_index(evicted).expect("logic error"),
      )
    } else {
      StorageInsertResult::Inserted(idx)
    }
  }

  /// IDからデータを取得
  ///
  /// ## Argument
  /// - `id`: データのID
  ///
  /// ## Return value
  /// - `Option<&T>` : 取得できたデータ
  pub fn get_by_id(
    &mut self,
    id: <T as ContentTrait>::ContentID,
  ) -> Option<&T> {
    if let Some(&idx) = self.by_id.get(&id) {
      self.cap_mode.touch(idx);
      self.data.get(idx).and_then(|opt| opt.as_ref())
    } else {
      None
    }
  }

  /// name_idからデータを取得
  ///
  /// ## Argument
  /// - `name_id`: データの文字列ID
  ///
  /// ## Return value
  /// - `Option<&T>` : 取得できたデータ
  pub fn get_by_name_id(
    &mut self,
    name_id: &str,
  ) -> Option<&T> {
    if let Some(&idx) = self.by_name_id.get(name_id) {
      self.cap_mode.touch(idx);
      self.data.get(idx).and_then(|opt| opt.as_ref())
    } else {
      None
    }
  }

  /// IDで削除
  ///
  /// ## Argument
  /// - `id`: データのID
  ///
  /// ## Return value
  /// - `Option<T>` : 削除されたデータ
  pub fn remove_by_id(
    &mut self,
    id: <T as ContentTrait>::ContentID,
  ) -> Option<T> {
    if let Some(idx) = self.by_id.remove(&id) {
      if let Some(item) = self.data[idx].as_ref() {
        self.by_name_id.remove(&item.name_id().to_string());
      }
      let old = self.data[idx].take();
      self.free_list.push_back(idx);
      self.cap_mode.remove(idx);
      old
    } else {
      None
    }
  }

  /// インデックスで削除
  ///
  /// ## Argument
  /// - `idx`: データのインデックス
  ///
  /// ## Return value
  /// - `Option<T>` : 削除されたデータ
  pub fn remove_by_index(
    &mut self,
    idx: usize,
  ) -> Option<T> {
    if idx >= self.data.len() {
      return None;
    }
    if let Some(item) = self.data[idx].take() {
      // IDとname_idのインデックスも消す
      self.by_id.remove(&item.id());
      self.by_name_id.remove(&item.name_id().to_string());
      self.free_list.push_back(idx);
      self.cap_mode.remove(idx);
      Some(item)
    } else {
      None
    }
  }

  /// イテレーション
  ///
  /// ## Return value
  /// - `impl Iterator<Item = &T>` : データのイテレータ
  pub fn iter(&self) -> impl Iterator<Item = &T> {
    self.data.iter().filter_map(|opt| opt.as_ref())
  }
}
