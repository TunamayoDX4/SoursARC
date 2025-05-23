use super::*;

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
impl UserId {
  pub fn new() -> Self {
    Self(Uuid::now_v7())
  }
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
pub struct UserKey(pub(super) String);
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
impl UserKey {
  pub fn new(key: impl ToString) -> Self {
    Self(key.to_string())
  }
}

#[derive(Debug)]
pub struct UserData {
  id_key: IdKeySet<Self>,
  children: Vec<work::WorkId>,
}
impl SousARCData for UserData {
  type Id = UserId;
  type Key = UserKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}
impl UserData {
  pub fn new(user_name: impl ToString) -> Self {
    let id = UserId::new();
    let key = UserKey::new(user_name);
    let id_key = IdKeySet::new(id, key);
    Self { id_key, children: Vec::new() }
  }
  pub fn children(
    &self,
  ) -> impl Iterator<Item = work::WorkId> {
    self.children.iter().copied()
  }
  pub fn spawn(
    &mut self,
    work_name: impl ToString,
  ) -> work::WorkData {
    let work_id = work::WorkId::new();
    self.children.push(work_id);
    work::WorkData::new(work_id, self.id(), work_name)
  }
}
