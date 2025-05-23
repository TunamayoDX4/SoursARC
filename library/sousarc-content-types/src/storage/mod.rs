use std::collections::{HashMap, VecDeque};

use indexmap::IndexMap;

use crate::traits::prelude::*;

#[derive(Debug)]
pub struct StandardStorage<D: SousARCData> {
  pub data: Vec<Option<D>>,
  pub empty_slot: VecDeque<usize>,
  pub keymap: IndexMap<D::Key, usize>,
  pub idmap: HashMap<D::Id, usize>,
}
impl<D: SousARCData> StandardStorage<D> {
  pub fn get_data(&self) -> &Vec<Option<D>> {
    &self.data
  }

  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      empty_slot: VecDeque::new(),
      keymap: IndexMap::new(),
      idmap: HashMap::new(),
    }
  }

  pub fn remove(&mut self, id: D::Id) -> Option<D> {
    if let Some(idx) = self.idmap.remove(&id) {
      if let Some(data) = self.data[idx].take() {
        self.keymap.shift_remove(data.key());
        self.empty_slot.push_back(idx);
        return Some(data);
      }
    }
    None
  }

  pub fn insert(&mut self, data: D) -> Option<D> {
    let idx = self
      .empty_slot
      .pop_front()
      .ok_or_else(|| self.data.len());
    if let Some(_) = self
      .keymap
      .insert(data.key().clone(), idx.unwrap_or_else(|e| e))
    {
      Some(data)
    } else {
      match self
        .idmap
        .insert(data.id(), idx.unwrap_or_else(|e| e))
      {
        None => {}
        Some(_) => unreachable!("IDが重複しています"),
      }
      match idx {
        Ok(idx) => {
          self.data[idx] = Some(data);
          None
        }
        Err(_) => {
          self.data.push(Some(data));
          None
        }
      }
    }
  }
}
impl<D> SousARCStorage<D> for StandardStorage<D>
where
  D: SousARCData,
{
  fn get(&self, id: <D as SousARCData>::Id) -> Option<&D> {
    self
      .idmap
      .get(&id)
      .map(|i| self.data[*i].as_ref())
      .flatten()
  }

  fn get_by_key<Q: Eq + std::hash::Hash>(
    &self,
    key: &Q,
  ) -> Option<&D>
  where
    <D as SousARCData>::Key: std::borrow::Borrow<Q>,
  {
    self
      .keymap
      .get(key)
      .map(|i| self.data[*i].as_ref())
      .flatten()
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
impl<D> SousARCStorageMut<D> for StandardStorage<D>
where
  D: SousARCData,
{
  fn get_mut(
    &mut self,
    id: <D as SousARCData>::Id,
  ) -> Option<&mut D> {
    self
      .idmap
      .get(&id)
      .map(|i| self.data[*i].as_mut())
      .flatten()
  }

  fn get_by_key_mut<Q: Eq + std::hash::Hash>(
    &mut self,
    key: &Q,
  ) -> Option<&mut D>
  where
    <D as SousARCData>::Key: std::borrow::Borrow<Q>,
  {
    self.keymap.get(key).and_then(|i| self.data[*i].as_mut())
  }
}
