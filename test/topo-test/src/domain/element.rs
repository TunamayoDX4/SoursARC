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
pub struct ElementId(Uuid);
impl Display for ElementId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl SousARCId for ElementId {
  type Bound = ElementData;
}
impl ElementId {
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
pub enum ElementParentId {
  Root(work::WorkId),
  Nest(ElementId),
}
impl Display for ElementParentId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::Root(id) => write!(f, "Root({id})"),
      Self::Nest(id) => write!(f, "Nest({id})"),
    }
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
pub struct ElementKey {
  parent: ElementParentId,
  key: String,
}
impl Display for ElementKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}::{}", self.parent, self.key)
  }
}
impl SousARCKey for ElementKey {
  type Bound = ElementData;
}
impl ElementKey {
  pub fn new_root(
    parent: work::WorkId,
    key: impl ToString,
  ) -> Self {
    Self {
      parent: ElementParentId::Root(parent),
      key: key.to_string(),
    }
  }
  pub fn new_nest(
    parent: ElementId,
    key: impl ToString,
  ) -> Self {
    Self {
      parent: ElementParentId::Nest(parent),
      key: key.to_string(),
    }
  }
  pub fn fq_name(
    &self,
    wrt: &mut impl std::fmt::Write,
    user_storage: &impl SousARCStorage<user::UserData>,
    work_storage: &impl SousARCStorage<work::WorkData>,
    element_storage: &impl SousARCStorage<ElementData>,
  ) -> std::fmt::Result {
    match &self.parent {
      ElementParentId::Root(work_id) => {
        work_storage
          .get(*work_id)
          .map(|w| w.key().fq_name(wrt, user_storage))
          .unwrap_or(Ok(()))?;
      }
      ElementParentId::Nest(element_id) => {
        element_storage
          .get(*element_id)
          .map(|e| {
            e.key().fq_name(
              wrt,
              user_storage,
              work_storage,
              element_storage,
            )
          })
          .unwrap_or(Ok(()))?;
      }
    }
    write!(wrt, "::{}", self.key)
  }
}

#[derive(Debug)]
pub struct ElementData {
  id_key: IdKeySet<Self>,
  children: Vec<ElementId>,
}
impl SousARCData for ElementData {
  type Id = ElementId;
  type Key = ElementKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}
impl ElementData {
  pub fn new(
    id: ElementId,
    parent: ElementParentId,
    key: impl ToString,
  ) -> Self {
    let key = match parent {
      ElementParentId::Root(work_id) => {
        ElementKey::new_root(work_id, key)
      }
      ElementParentId::Nest(element_id) => {
        ElementKey::new_nest(element_id, key)
      }
    };
    let id_key = IdKeySet::new(id, key);
    Self { id_key, children: Vec::new() }
  }
  pub fn spawn(&mut self, key: impl ToString) -> Self {
    let element_id = ElementId::new();
    self.children.push(element_id);
    Self::new(
      element_id,
      ElementParentId::Nest(self.id()),
      key,
    )
  }
}
