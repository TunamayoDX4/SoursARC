use serde::{Deserialize, Serialize};

use super::traits::*;

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
pub struct UserId(ulid::Ulid);
impl SoursARCId for UserId {
  type Bound = User;
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
pub struct UserName {
  name: String,
}
impl SoursARCName for UserName {
  type Bound = User;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  id: UserId,
  name: UserName,
  works: Vec<WorkId>,
}
impl SoursARCInstance for User {
  type Id = UserId;
  type Name = UserName;
  fn id(&self) -> &Self::Id {
    &self.id
  }
  fn name(&self) -> &Self::Name {
    &self.name
  }
}
impl SoursARCInstanceHasChild<Work> for User {
  fn child(
    &self,
  ) -> impl Iterator<Item = <Work as SoursARCInstance>::Id>
  {
    self.works.iter().copied()
  }
}

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
pub struct WorkId(ulid::Ulid);
impl SoursARCId for WorkId {
  type Bound = Work;
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
pub struct WorkName {
  user_id: UserId,
  name: String,
}
impl SoursARCName for WorkName {
  type Bound = Work;
}
impl SoursARCNameHasParent<User> for WorkName {
  fn parent_id(&self) -> impl Iterator<Item = UserId> {
    [self.user_id].into_iter()
  }
}
impl WorkName {
  pub fn fqname<'a>(
    &'a self,
    users: &'a impl SoursARCStorage<User>,
  ) -> impl Iterator<Item = &'a str> {
    [
      self.parent_name(users).map(|n| &n.name).next(),
      Some(&self.name),
    ]
    .into_iter()
    .filter_map(|n| n.map(|n| n.as_str()))
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Work {
  id: WorkId,
  name: WorkName,
  elements: Vec<ElementId>,
}
impl SoursARCInstance for Work {
  type Id = WorkId;
  type Name = WorkName;
  fn id(&self) -> &Self::Id {
    &self.id
  }
  fn name(&self) -> &Self::Name {
    &self.name
  }
}
impl SoursARCInstanceHasParent<User> for Work {}
impl SoursARCInstanceHasChild<Element> for Work {
  fn child(
    &self,
  ) -> impl Iterator<Item = <Element as SoursARCInstance>::Id>
  {
    self.elements.iter().copied()
  }
}

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
pub struct ElementId(ulid::Ulid);
impl SoursARCId for ElementId {
  type Bound = Element;
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
pub struct ElementName {
  work_id: WorkId,
  element_id: Vec<ElementId>,
  name: String,
}
impl SoursARCName for ElementName {
  type Bound = Element;
}
impl SoursARCNameHasParent<Work> for ElementName {
  fn parent_id(&self) -> impl Iterator<Item = WorkId> {
    [self.work_id].into_iter()
  }
}
impl SoursARCNameHasParent<Element> for ElementName {
  fn parent_id(&self) -> impl Iterator<Item = ElementId> {
    self.element_id.iter().copied()
  }
}
impl ElementName {
  pub fn fqname<'a, W: std::fmt::Write>(
    &'a self,
    w: &mut W,
    users: &'a impl SoursARCStorage<User>,
    works: &'a impl SoursARCStorage<Work>,
    elements: &'a impl SoursARCStorage<Element>,
  ) -> Result<(), std::fmt::Error> {
    let work: &WorkName =
      self.parent_name(works).next().unwrap();
    work
      .fqname(users)
      .chain(
        self.parent_name(elements).map(|n| n.name.as_str()),
      )
      .try_for_each(|n| {
        write!(w, "{}::", n)?;
        Ok(())
      })?;
    write!(w, "{}", self.name)?;
    Ok(())
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Element {
  id: ElementId,
  name: ElementName,
  elements: Vec<ElementId>,
}
impl SoursARCInstance for Element {
  type Id = ElementId;
  type Name = ElementName;
  fn id(&self) -> &Self::Id {
    &self.id
  }
  fn name(&self) -> &Self::Name {
    &self.name
  }
}
impl SoursARCInstanceHasParent<Work> for Element {}
impl SoursARCInstanceHasChild<Self> for Element {
  fn child(
    &self,
  ) -> impl Iterator<Item = <Self as SoursARCInstance>::Id>
  {
    self.elements.iter().copied()
  }
}
impl SoursARCInstanceHasParent<Self> for Element {}
