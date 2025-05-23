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
impl WorkId {
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
pub struct WorkKey {
  user_id: user::UserId,
  key: String,
}
impl Display for WorkKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}::{}", self.user_id, self.key)
  }
}
impl SousARCKey for WorkKey {
  type Bound = WorkData;
}
impl WorkKey {
  pub fn new(
    user_id: user::UserId,
    key: impl ToString,
  ) -> Self {
    Self { user_id, key: key.to_string() }
  }
  pub fn fq_name(
    &self,
    wrt: &mut impl std::fmt::Write,
    storage: &impl SousARCStorage<user::UserData>,
  ) -> std::fmt::Result {
    let user_key = self.user_id.get_key(storage);
    write!(
      wrt,
      "{}::{}",
      match user_key {
        Some(key) => key.0.as_str(),
        None => "<unknown>",
      },
      self.key
    )
  }
}

#[derive(Debug)]
pub struct WorkData {
  id_key: IdKeySet<Self>,
  children: Vec<element::ElementId>,
}
impl SousARCData for WorkData {
  type Id = WorkId;
  type Key = WorkKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}
impl WorkData {
  pub fn new(
    id: WorkId,
    user_id: user::UserId,
    work_name: impl ToString,
  ) -> Self {
    let key = WorkKey::new(user_id, work_name);
    Self {
      id_key: IdKeySet::new(id, key),
      children: Vec::new(),
    }
  }
  pub fn spawn(
    &mut self,
    element_name: impl ToString,
  ) -> element::ElementData {
    let element_id = element::ElementId::new();
    self.children.push(element_id);
    element::ElementData::new(
      element_id,
      element::ElementParentId::Root(self.id()),
      element_name,
    )
  }
}
