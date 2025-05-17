use std::cell::{LazyCell, RefCell};

use hashbrown::HashMap;
use icu_properties::{CodePointSetData, props::Emoji};
use wasm_bindgen::prelude::*;

struct EmojiConverter {
  /// `💛` to `:yellow_heart:`
  emoji_decoder: HashMap<char, String>,

  /// `:yellow_heart:` to `<img href=".../yellow_heart.svg">`
  emoji_encoder: HashMap<String, String>,

  buffer: String,

  replacer: regex::Regex,
}
impl EmojiConverter {
  pub fn new(
    emoji_decoder: HashMap<char, String>,
    emoji_encoder: HashMap<String, String>,
  ) -> Self {
    Self {
      emoji_decoder,
      emoji_encoder,
      buffer: String::new(),
      replacer: regex::Regex::new(r":([a-zA-Z_]+):")
        .unwrap(),
    }
  }

  pub fn dec_enc(
    &mut self,
    wrt: &mut impl std::fmt::Write,
    text: &str,
  ) -> std::fmt::Result {
    let cpsd = CodePointSetData::new::<Emoji>();
    self.buffer.clear();
    for (emoji, c) in
      text.chars().map(|c| (cpsd.contains(c), c))
    {
      if !emoji {
        self.buffer.push(c);
      } else {
        if let Some(emoji) = self.emoji_decoder.get(&c) {
          self.buffer.push_str(emoji);
        } else {
          self.buffer.push(c);
        }
      }
    }
    let output = self.replacer.replace_all(
      &self.buffer,
      |caps: &regex::Captures| {
        let emoji = &caps[1];
        if let Some(emoji) = self.emoji_encoder.get(emoji) {
          format!("<img src=\"{emoji}\" alt=\"{emoji}\" />",)
        } else {
          format!(":{emoji}:")
        }
      },
    );
    write!(wrt, "{output}")?;

    Ok(())
  }
}

thread_local! {
  static EMOJI_CONVERTER: Option<RefCell<EmojiConverter>> = None;
}

