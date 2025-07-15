use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader, Token,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

/*
• This section is optional. It defines the input values of various parameters relating to this well.
• These input values can consist of numbers or text.
• Only one "~P" section can occur in an LAS 2.0 file.
• The mnemonics used are not restricted but must be defined on the line on which they appear.
• There is no limit on the number of lines that can be used.
• The following is an example of a Parameter Information Section.
*/

#[derive(Debug, Default, Deserialize)]
pub struct ParameterInformation {
  pub parameters: Vec<Mnemonic>,
  pub comments: Vec<String>,
  #[serde(skip)]
  pub(crate) is_parsed: bool,
}

impl ParameterInformation {
  pub fn parse(
    reader: &mut PeekableFileReader,
    current_comments: &mut Vec<String>,
  ) -> Result<Self, LibLasError> {
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

    this.is_parsed = true;
    return Ok(this);
  }

  pub fn new(parameters: Vec<Mnemonic>, comments: Vec<String>, is_parsed: bool) -> Self {
    return Self {
      parameters,
      comments,
      is_parsed,
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
