use std::collections::{HashMap, VecDeque};

use indexmap::IndexMap;

use crate::traits::{
  SousARCData, SousARCDataWrap, SousARCStorage,
  SousARCStorageMut,
};

pub struct StandardStorage<D: SousARCData, W: From<D>> {
  pub data: Vec<Option<SousARCDataWrap<D, W>>>,
  pub empty_slot: VecDeque<usize>,
  pub keymap: IndexMap<D::Key, usize>,
  pub idmap: HashMap<D::Id, usize>,
}
impl<D: SousARCData, W: From<D>> StandardStorage<D, W> {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      empty_slot: VecDeque::new(),
      keymap: IndexMap::new(),
      idmap: HashMap::new(),
    }
  }

  pub fn insert(
    &mut self,
    data: SousARCDataWrap<D, W>,
  ) -> Option<SousARCDataWrap<D, W>> {
    let idx = if let Some(idx) = self.empty_slot.pop_front()
    {
      Ok(idx)
    } else {
      Err(self.data.len())
    };
    if let Some(_) = self.keymap.insert(
      data.key().clone(),
      idx.map_or_else(|e| e, |o| o),
    ) {
      Some(data)
    } else {
      match self
        .idmap
        .insert(data.id(), idx.map_or_else(|e| e, |o| o))
      {
        None => {}
        Some(_) => unreachable!("IDが重複しています"),
      }
      match idx {
        Ok(idx) => {
          self.data[idx] = Some(data);
          None
        }
        Err(idx) => {
          self.data.push(Some(data));
          self.empty_slot.push_back(idx);
          None
        }
      }
    }
  }
}
impl<D, W> SousARCStorage<D, W> for StandardStorage<D, W>
where
  D: SousARCData,
  W: From<D>,
{
  fn get(&self, id: <D as SousARCData>::Id) -> Option<&W> {
    if let Some(idx) = self.idmap.get(&id) {
      if let Some(data) = self.data.get(*idx) {
        return data.as_ref().map(|d| &d.data);
      }
    }
    None
  }

  fn get_by_key<Q: Eq + std::hash::Hash>(
    &self,
    key: &Q,
  ) -> Option<&W>
  where
    <D as SousARCData>::Key: std::borrow::Borrow<Q>,
  {
    if let Some(idx) = self.keymap.get(key) {
      if let Some(data) = self.data.get(*idx) {
        return data.as_ref().map(|d| &d.data);
      }
    }
    None
  }

  fn id<Q: Eq + std::hash::Hash>(
    &self,
    key: &Q,
  ) -> Option<<D as SousARCData>::Id>
  where
    <D as SousARCData>::Key: std::borrow::Borrow<Q>,
  {
    if let Some(idx) = self.keymap.get(key) {
      if let Some(data) = self.data.get(*idx) {
        return data.as_ref().map(|d| d.id());
      }
    }
    None
  }

  fn key(
    &self,
    id: <D as SousARCData>::Id,
  ) -> Option<&<D as SousARCData>::Key> {
    if let Some(idx) = self.idmap.get(&id) {
      if let Some(data) = self.data.get(*idx) {
        return data.as_ref().map(|d| d.key());
      }
    }
    None
  }
}
impl<D, W> SousARCStorageMut<D, W> for StandardStorage<D, W>
where
  D: SousARCData,
  W: From<D>,
{
  fn get_mut(
    &mut self,
    id: <D as SousARCData>::Id,
  ) -> Option<&mut W> {
    if let Some(idx) = self.idmap.get(&id) {
      if let Some(data) = self.data.get_mut(*idx) {
        return data.as_mut().map(|d| &mut d.data);
      }
    }
    None
  }

  fn get_by_key_mut<Q: Eq + std::hash::Hash>(
    &mut self,
    key: &Q,
  ) -> Option<&mut W>
  where
    <D as SousARCData>::Key: std::borrow::Borrow<Q>,
  {
    if let Some(idx) = self.keymap.get(key) {
      if let Some(data) = self.data.get_mut(*idx) {
        return data.as_mut().map(|d| &mut d.data);
      }
    }
    None
  }
}
