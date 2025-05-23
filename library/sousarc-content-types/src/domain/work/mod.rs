use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::traits::prelude::*;

use super::element::ElementId;
use super::user::UserId;

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct WorkId(Uuid);

impl Display for WorkId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl SousARCId for WorkId {
  type Bound = WorkData;
}

#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct WorkKey {
  user_id: UserId,
  work_name: String,
}

impl<T: ToString> From<(UserId, T)> for WorkKey {
  fn from((user_id, work_name): (UserId, T)) -> Self {
    Self { user_id, work_name: work_name.to_string() }
  }
}

impl Display for WorkKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(
      f,
      "{u}::{n}",
      u = self.user_id,
      n = self.work_name
    )
  }
}

impl SousARCKey for WorkKey {
  type Bound = WorkData;
}

#[derive(Debug)]
pub struct WorkData {
  id_key: IdKeySet<Self>,

  pub body: WorkDataBody,
}

impl SousARCData for WorkData {
  type Id = WorkId;
  type Key = WorkKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkDataBody {
  pub children: Vec<ElementId>,

  pub display_name: String,

  pub description: String,
}
