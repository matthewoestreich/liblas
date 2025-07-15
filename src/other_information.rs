use crate::{
  LibLasError::{self, ReadingNextLine},
  PeekableFileReader, Token,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformation {
  pub text: String,
  pub comments: Vec<String>,
  #[serde(skip)]
  pub(crate) is_parsed: bool,
}

impl OtherInformation {
  pub fn parse(
    reader: &mut PeekableFileReader,
    current_comments: &mut Vec<String>,
  ) -> Result<OtherInformation, LibLasError> {
    let mut this = OtherInformation::default();

    // Comments were abovve the "~Other Info" section
    if !current_comments.is_empty() {
      this.comments = current_comments.to_vec();
      // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
      current_comments.clear();
    }

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.trim().to_string().starts_with(&Token::Tilde()) {
        break;
      }

      let line = &reader.next().ok_or(ReadingNextLine)??.trim().to_string();

      if line.starts_with(&Token::Comment()) {
        current_comments.push(line.clone());
        continue;
      }

      let final_line = if line.ends_with(" ") { line } else { &format!("{line} ") };

      this.text.push_str(final_line);
    }

    this.text = this.text.trim().to_string();
    this.is_parsed = true;
    return Ok(this);
  }

  pub fn new(text: String, comments: Vec<String>, is_parsed: bool) -> Self {
    return Self {
      text,
      comments,
      is_parsed,
    };
  }
}
