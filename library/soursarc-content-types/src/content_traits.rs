use std::{borrow::Cow, hash::Hash};

use serde::{Deserialize, Serialize};

/// # ContentTrait
/// コンテンツのIDと名前を持つトレイト
pub trait ContentTrait:
  Serialize + for<'de> Deserialize<'de>
{
  type ContentID: Eq
    + Hash
    + Clone
    + Serialize
    + for<'de> Deserialize<'de>;

  fn id(&self) -> Self::ContentID;
  fn name_id(&self) -> Cow<str>;
}
