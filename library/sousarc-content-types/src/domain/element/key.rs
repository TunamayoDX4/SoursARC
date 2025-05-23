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
pub enum ElementParent {
  Root(WorkId),
  Nest(ElementId),
}

impl Display for ElementParent {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    match self {
      Self::Root(id) => write!(f, "Root({})", id),
      Self::Nest(id) => write!(f, "Nest({})", id),
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
  pub parent: ElementParent,
  pub name: String,
}

impl Display for ElementKey {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{p}::{n}", p = self.parent, n = self.name)
  }
}

impl<T: ToString> From<(WorkId, T)> for ElementKey {
  fn from((parent, name): (WorkId, T)) -> Self {
    Self {
      parent: ElementParent::Root(parent),
      name: name.to_string(),
    }
  }
}

impl<T: ToString> From<(ElementId, T)> for ElementKey {
  fn from((parent, name): (ElementId, T)) -> Self {
    Self {
      parent: ElementParent::Nest(parent),
      name: name.to_string(),
    }
  }
}

impl SousARCKey for ElementKey {
  type Bound = ElementData;
}
