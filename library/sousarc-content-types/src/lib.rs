pub mod traits;

pub mod domain;
pub mod storage;

pub mod prelude {
  pub use crate::{
    domain::{
      element::{ElementData, ElementId, ElementKey},
      user::{UserData, UserId, UserKey},
      work::{WorkData, WorkId, WorkKey},
    },
    storage::*,
    traits::*,
  };
}
