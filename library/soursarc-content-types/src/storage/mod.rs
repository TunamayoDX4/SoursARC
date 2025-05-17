use super::traits::*;

pub struct SimpleStorage<I: SoursARCInstance> {
  pub data: std::collections::HashMap<I::Id, I>,
  pub table: indexmap::IndexMap<I::Name, I::Id>,
}
impl<I: SoursARCInstance> SimpleStorage<I> {
  pub fn new() -> Self {
    Self {
      data: std::collections::HashMap::new(),
      table: indexmap::IndexMap::new(),
    }
  }

  /// データを挿入する
  ///
  /// ## Argument
  /// - `item`: 挿入するインスタンス
  ///
  /// ## Return value
  /// - 既存の値があればそれを返す
  pub fn insert(&mut self, item: I) -> Option<I> {
    let id = item.id().clone();
    let name = item.name().clone();
    self.table.insert(name.clone(), id);
    self.data.insert(id, item)
  }

  /// データを削除する
  ///
  /// ## Argument
  /// - `id`: 削除するインスタンスのID
  ///
  /// ## Return value
  /// - 削除された値
  pub fn remove(&mut self, id: &I::Id) -> Option<I> {
    if let Some(item) = self.data.remove(id) {
      self.table.shift_remove(item.name());
      Some(item)
    } else {
      None
    }
  }

  /// データのイテレータを返す
  pub fn iter(&self) -> impl Iterator<Item = (&I::Id, &I)> {
    self.data.iter()
  }

  /// 名前テーブルのイテレータを返す
  pub fn iter_name(
    &self,
  ) -> impl Iterator<Item = (&I::Name, &I)> {
    self
      .table
      .iter()
      .map(|(k, id)| (k, self.data.get(id).unwrap()))
  }

  /// data(HashMap<I::Id, I>)のみをMessagePackでファイル出力する
  ///
  /// ## Argument
  /// - `path`: 出力先パス
  ///
  /// ## Return value
  /// - `Result<(), std::io::Error>`
  pub fn export_data_msgpack(
    &self,
    path: impl AsRef<std::path::Path>,
  ) -> Result<(), std::io::Error> {
    let file = std::fs::File::create(path)?;
    rmp_serde::encode::write(
      &mut std::io::BufWriter::new(file),
      &self.data,
    )
    .map_err(|e| {
      std::io::Error::new(std::io::ErrorKind::Other, e)
    })
  }

  /// MessagePackファイルからdata(HashMap<I::Id, I>)をインポートし、IndexMapも再構築する
  ///
  /// ## Argument
  /// - `path`: 読み込み元パス
  ///
  /// ## Return value
  /// - `Result<(), std::io::Error>`
  pub fn import_data_msgpack(
    &mut self,
    path: impl AsRef<std::path::Path>,
  ) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let data: std::collections::HashMap<I::Id, I> =
      rmp_serde::from_read(reader).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, e)
      })?;
    // IndexMap(table)を再構築
    let mut table = indexmap::IndexMap::new();
    for (id, item) in &data {
      table.insert(item.name().clone(), id.clone());
    }
    self.data = data;
    self.table = table;
    Ok(())
  }
}
impl<I> SoursARCStorage<I> for SimpleStorage<I>
where
  I: SoursARCInstance,
{
  type Data<'a>
    = &'a I
  where
    Self: 'a;
  fn get<'a>(&'a self, id: I::Id) -> Option<Self::Data<'a>> {
    self.data.get(&id)
  }
  fn get_by_name<'a>(
    &'a self,
    name: &I::Name,
  ) -> Option<Self::Data<'a>> {
    self.data.get(self.table.get(name)?)
  }
  fn get_name_by_id<'a>(
    &'a self,
    id: &I::Id,
  ) -> Option<&'a I::Name> {
    self.data.get(id).map(|i| i.name())
  }
}

impl<I> SoursARCStorageReverseRef<I> for SimpleStorage<I>
where
  I: SoursARCInstance,
{
  fn get_id_by_name<'a>(
    &'a self,
    name: &I::Name,
  ) -> Option<I::Id> {
    self.table.get(name).copied()
  }
}
impl<I> SoursARCStorageMut<I> for SimpleStorage<I>
where
  I: SoursARCInstance,
{
  type DataMut<'a>
    = &'a mut I
  where
    Self: 'a;
  fn get_mut<'a>(
    &'a mut self,
    id: <I as SoursARCInstance>::Id,
  ) -> Option<Self::DataMut<'a>> {
    self.data.get_mut(&id)
  }
  fn get_mut_by_name<'a>(
    &'a mut self,
    name: &I::Name,
  ) -> Option<Self::DataMut<'a>> {
    self.data.get_mut(self.table.get(name)?)
  }
}
