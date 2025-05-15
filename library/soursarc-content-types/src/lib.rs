//! SoursARCのコンテンツを定義するライブラリ
//! Library for defining SoursARC content
//!
//! ## Contentの概要
//! このライブラリは、SoursARCのコンテンツ構造を定義するためのものです。
//! 各コンテンツの詳細については、ドキュメントを参照してください。
//!
//! ### 主なコンテンツ
//! - ユーザ (User)
//! - 創作物 (Work)
//! - 要素 (Element)
//! - ジャンル (Genre)
//! - カテゴリ (Category)
//! - フォルダ (Folder)
//! - ドキュメント (Document)

pub mod content;
pub mod domain;
pub mod tag;

pub mod content_traits;
pub mod id;
pub mod storage;

pub mod prelude;
