use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::traits::prelude::*;

use super::work::WorkId;

pub mod id;
pub use id::*;
pub mod key;
pub use key::*;

#[derive(Debug)]
pub struct ElementData {
  id_key: IdKeySet<Self>,

  pub body: ElementDataBody,
}

impl SousARCData for ElementData {
  type Id = ElementId;
  type Key = ElementKey;

  fn id_key_set(&self) -> &IdKeySet<Self> {
    &self.id_key
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementDataBody {
  pub children: Option<Vec<ElementId>>,

  pub display_name: String,

  pub content: String,
}
