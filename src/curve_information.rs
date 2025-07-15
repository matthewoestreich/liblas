use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader,
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Debug, Default, Deserialize)]
pub struct CurveInformation {
  pub curves: Vec<Mnemonic>,
  pub comments: Vec<String>,
  #[serde(skip)]
  pub(crate) is_parsed: bool,
}

impl CurveInformation {
  pub fn parse(
    reader: &mut PeekableFileReader,
    current_comments: &mut Vec<String>,
  ) -> Result<CurveInformation, LibLasError> {
    let mut this = CurveInformation::default();

    // Comments were above the "~Curve Information" section
    if !current_comments.is_empty() {
      this.comments = current_comments.to_vec();
      // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
      current_comments.clear();
    }

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.trim().to_string().starts_with("~") {
        break;
      }

      let line = reader.next().ok_or(ReadingNextLine)??.trim().to_string();

      if line.starts_with("#") {
        current_comments.push(line.clone());
        continue;
      }

      // If there were comments above this mnemonic line, we pass them
      // in to get attached to that mnemonic struct.
      let mnemonic = Mnemonic::from_str(&line, current_comments)?;
      this.curves.push(mnemonic);
    }

    this.is_parsed = true;
    return Ok(this);
  }

  pub fn new(curves: Vec<Mnemonic>, comments: Vec<String>, is_parsed: bool) -> Self {
    return Self {
      curves,
      comments,
      is_parsed,
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
