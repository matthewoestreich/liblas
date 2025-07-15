use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader,
};
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInformation {
  #[serde(rename = "VERS")]
  pub version: Mnemonic,
  #[serde(rename = "WRAP")]
  pub wrap: Mnemonic,
  #[serde(flatten)]
  pub extra: HashMap<String, Mnemonic>,
  pub comments: String,
  #[serde(skip)]
  pub(crate) is_parsed: bool,
}

impl VersionInformation {
  pub fn parse(reader: &mut PeekableFileReader) -> Result<VersionInformation, LibLasError> {
    let mut this = VersionInformation::default();

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.starts_with("~") {
        break;
      }

      let next_line = reader.next().ok_or(ReadingNextLine)??;

      // TODO : SKIPPING COMMENTS FOR NOW
      if next_line.starts_with("#") {
        continue;
      }

      if next_line.starts_with("VERS") {
        this.version = Mnemonic::from_line(&next_line)?;
      } else if next_line.starts_with("WRAP") {
        this.wrap = Mnemonic::from_line(&next_line)?;
      } else {
        let x = Mnemonic::from_line(&next_line)?;
        this.extra.insert(x.name.clone(), x);
      }
    }

    this.is_parsed = true;
    return Ok(this);
  }
}
