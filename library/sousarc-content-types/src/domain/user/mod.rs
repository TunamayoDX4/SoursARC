use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::traits::prelude::*;

use super::work::WorkId;

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
pub struct UserId(Uuid);

impl Display for UserId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl SousARCId for UserId {
  type Bound = UserData;
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
pub struct UserKey(String);

impl Display for UserKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl SousARCKey for UserKey {
  type Bound = UserData;
}

#[derive(Debug)]
pub struct UserData {
  id_key: IdKeySet<Self>,

  pub body: RwLock<UserDataBody>,
}
impl SousARCData for UserData {
  type Id = UserId;
  type Key = UserKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}

impl UserData {
  pub fn new(
    id: UserId,
    key: UserKey,
    body: UserDataBody,
  ) -> Self {
    Self {
      id_key: IdKeySet::new(id, key),
      body: RwLock::new(body),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDataBody {
  pub children: Vec<WorkId>,

  pub display_name: String,

  pub introduction: String,
}

impl UserDataBody {
  pub fn children(&self) -> impl Iterator<Item = WorkId> {
    self.children.iter().copied()
  }
}
