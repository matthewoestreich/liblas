use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader, Token,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Debug, Default, Deserialize)]
pub struct ParameterInformation {
  pub parameters: Vec<Mnemonic>,
  pub comments: Vec<String>,
}

impl ParameterInformation {
  pub fn parse(reader: &mut PeekableFileReader, current_comments: &mut Vec<String>) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    // Comments were above the "~Parameter Info" line
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
      this.parameters.push(mnemonic);
    }

    return Ok(this);
  }

  pub fn to_str(&self) -> Option<String> {
    if self.comments.is_empty() && self.parameters.is_empty() {
      return None;
    }
    let mut output = "~Parameter Information".to_string();
    if !self.comments.is_empty() {
      output = format!("{}\n{output}", self.comments.join(" "));
    }
    if !self.parameters.is_empty() {
      self
        .parameters
        .iter()
        .for_each(|a| output = format!("{output}\n{}", a.to_str()));
    }
    return Some(output);
  }

  pub fn new(parameters: Vec<Mnemonic>, comments: Vec<String>) -> Self {
    return Self {
      parameters,
      comments,
    };
  }
}

impl Serialize for ParameterInformation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.parameters.len() + 1))?;

    for mnemonic in &self.parameters {
      map.serialize_entry(&mnemonic.name, mnemonic)?;
    }

    map.serialize_entry("comments", &self.comments)?;
    return map.end();
  }
}
