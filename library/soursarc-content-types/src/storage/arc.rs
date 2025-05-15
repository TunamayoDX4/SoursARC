use super::{Cache, ResizableCache};
use std::collections::VecDeque;

/// # ARC
///
/// ## Summary
/// Adaptive Replacement Cache (ARC) の簡易実装。
///
/// - 「最近使った」(recent) と「よく使う」(frequent) の2つのリストでキャッシュ管理を行う。
/// - キャッシュ容量を超えた場合は自動で古いエントリを削除。
/// - `touch`でアクセス時にキャッシュを更新し、
///   recent→frequentへの昇格や、先頭への移動を行う。
/// - `remove`でキャッシュから明示的に削除可能。
///
/// ## Member
/// - `recent` : 最近使ったエントリのリスト
/// - `frequent` : よく使うエントリのリスト
/// - `cache_capacity` : キャッシュ全体の最大サイズ
#[derive(Debug)]
pub struct ARC<T> {
  recent: VecDeque<T>,
  frequent: VecDeque<T>,
  pub cache_capacity: usize,
}

impl<T: PartialEq + Copy> ARC<T> {
  /// 新規作成
  ///
  /// ## Argument
  /// - `cache_capacity` : キャッシュ全体の最大サイズ
  ///
  /// ## Return value
  /// - `ARC<T>` : 新しく生成されたARCキャッシュ
  pub fn new(cache_capacity: usize) -> Self {
    Self {
      recent: VecDeque::with_capacity(
        cache_capacity / 2 + 1,
      ),
      frequent: VecDeque::with_capacity(
        cache_capacity / 2 + 1,
      ),
      cache_capacity,
    }
  }

  /// キャッシュを更新
  ///
  /// ## Summary
  /// アクセスされたエントリをキャッシュ内で昇格・移動する。
  /// - frequentにあれば先頭へ
  /// - recentにあればfrequentへ昇格
  /// - どちらにもなければrecentへ追加
  ///
  /// ## Argument
  /// - `idx` : アクセスされたエントリ
  ///
  /// ## Return value
  /// - Option<T> : キャッシュから溢れて追い出されたエントリ（なければNone）
  pub fn touch(&mut self, idx: T) -> Option<T> {
    // frequentを先に見て、ない場合はrecentを調べる
    match self.frequent.iter().position(|&i| i == idx) {
      Some(up) => {
        // frequentにあれば先頭に移動
        self.frequent.remove(up);
        self.frequent.push_front(idx);
        // あふれたのを捨てる
        if self.frequent.len() > self.cache_capacity / 2 {
          self.frequent.pop_back()
        } else {
          None
        }
      }
      None => {
        // frequentにない場合はrecentを調べる
        match self.recent.iter().position(|&i| i == idx) {
          // recentにあればfrequentへ昇格
          Some(up) => {
            self.recent.remove(up);
            self.frequent.push_front(idx);
            if self.recent.len() > self.cache_capacity / 2 {
              self.recent.pop_back()
            } else {
              None
            }
          }
          None => {
            // どちらにもなければrecentに追加
            self.recent.push_front(idx);
            // recentのサイズがキャッシュ容量の半分を超えたら古いエントリを削除
            if self.recent.len() > self.cache_capacity / 2 {
              self.recent.pop_back()
            } else {
              None
            }
          }
        }
      }
    }
  }

  /// キャッシュから削除
  ///
  /// ## Summary
  /// 指定したエントリをrecent/frequent両方から削除する。
  ///
  /// ## Argument
  /// - `idx` : 削除するエントリ
  pub fn remove(&mut self, idx: T) -> Option<T> {
    match self.frequent.iter().position(|&i| i == idx) {
      Some(pos) => self.frequent.remove(pos),
      None => {
        let pos =
          self.recent.iter().position(|&i| i == idx)?;
        self.recent.remove(pos)
      }
    }
  }

  /// デバッグ用: キャッシュ内容取得
  ///
  /// ## Return value
  /// - (Vec<T>, Vec<T>) : (recent, frequent)の内容
  pub fn indices(&self) -> (Vec<T>, Vec<T>) {
    (
      self.recent.iter().copied().collect(),
      self.frequent.iter().copied().collect(),
    )
  }

  /// デバッグ用: キャッシュ内容取得
  ///
  /// ## Return value
  /// - (impl Iterator<Item = T>, impl Iterator<Item = T>) : (recent, frequent)の内容
  pub fn indices_iter(
    &self,
  ) -> (impl Iterator<Item = T>, impl Iterator<Item = T>) {
    (
      self.recent.iter().copied(),
      self.frequent.iter().copied(),
    )
  }

  /// デバッグ用: キャッシュサイズ取得
  ///
  /// ## Return value
  /// - usize : キャッシュのサイズ
  pub fn len(&self) -> usize {
    self.recent.len() + self.frequent.len()
  }
}

impl<T: PartialEq + Copy> Cache<T> for ARC<T> {
  /// 指定したインデックスの値を取得
  fn get(&self, id: usize) -> Option<&T> {
    self.recent.iter().chain(self.frequent.iter()).nth(id)
  }

  /// キャッシュ容量を取得
  fn capacity(&self) -> usize {
    self.cache_capacity
  }

  /// キャッシュを更新（touch）
  fn touch(&mut self, id: T) -> Option<T> {
    ARC::touch(self, id)
  }

  /// キャッシュから削除
  fn remove(&mut self, id: T) -> Option<T> {
    ARC::remove(self, id)
  }
}

impl<T: PartialEq + Copy> ResizableCache<T> for ARC<T> {
  /// キャッシュ容量を変更
  fn resize(&mut self, new_capacity: usize) -> bool {
    if new_capacity == 0 {
      return false;
    }
    self.cache_capacity = new_capacity;
    // recent/frequentのサイズを調整
    let half = new_capacity / 2 + 1;
    while self.recent.len() > half {
      self.recent.pop_back();
    }
    while self.frequent.len() > half {
      self.frequent.pop_back();
    }
    true
  }
}
