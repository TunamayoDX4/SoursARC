use super::{ElementID, UserID, WorkID};
use serde::{Deserialize, Serialize};

/// # GlobalID
/// グローバルID
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
pub struct GlobalID {
  user: UserID,
  work: WorkID,
  element: ElementID,
}
