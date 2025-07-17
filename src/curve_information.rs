use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader, Token,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Debug, Default, Deserialize)]
pub struct CurveInformation {
  pub curves: Vec<Mnemonic>,
  pub comments: Vec<String>,
}

impl CurveInformation {
  pub fn parse(reader: &mut PeekableFileReader, current_comments: &mut Vec<String>) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    // Comments were above the "~Curve Information" section
    if !current_comments.is_empty() {
      this.comments = current_comments.to_vec();
      // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
      current_comments.clear();
    }

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.trim().to_string().starts_with(&Token::Tilde()) {
        break;
      }

      let line = reader.next().ok_or(ReadingNextLine)??.trim().to_string();

      if line.starts_with(&Token::Comment()) {
        current_comments.push(line.clone());
        continue;
      }

      let mnemonic = Mnemonic::from_str(&line, current_comments)?;
      this.curves.push(mnemonic);
    }

    return Ok(this);
  }

  pub fn to_str(&self) -> String {
    let mut output = "~Curve Information".to_string();
    if !self.comments.is_empty() {
      output = format!("{}\n{output}", self.comments.join(" "));
    }
    if !self.curves.is_empty() {
      self
        .curves
        .iter()
        .for_each(|a| output = format!("{output}\n{}", a.to_str()));
    }
    return output;
  }

  pub fn new(curves: Vec<Mnemonic>, comments: Vec<String>) -> Self {
    return Self {
      curves,
      comments,
    };
  }
}

impl Serialize for CurveInformation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.curves.len() + 1))?;

    for mnemonic in &self.curves {
      map.serialize_entry(&mnemonic.name, mnemonic)?;
    }

    map.serialize_entry("comments", &self.comments)?;
    return map.end();
  }
}
