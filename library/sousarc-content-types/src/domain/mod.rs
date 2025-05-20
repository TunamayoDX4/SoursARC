//! ドメイン層の型・ロジックまとめ
//!
//! ## Summary
//! SoursARCのドメインモデル（ユーザ・作品・要素など）を定義するモジュール群。

pub mod element;
pub mod user;
pub mod work;

pub mod prelude {
  pub use super::{
    element::{ElementData, ElementId, ElementKey},
    user::{UserData, UserId, UserKey},
    work::{WorkData, WorkId, WorkKey},
  };
}
