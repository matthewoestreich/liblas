use crate::{
  LibLasError::{self, ReadingNextLine},
  PeekableFileReader,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformation(pub String);

impl OtherInformation {
  pub fn parse(reader: &mut PeekableFileReader) -> Result<OtherInformation, LibLasError> {
    let mut this = OtherInformation::default();

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.starts_with("~") {
        break;
      }
      let line = &reader.next().ok_or(ReadingNextLine)??;
      // TODO : SKIPPING COMMENTS FOR NOW
      if line.starts_with("#") {
        continue;
      }
      this.0.push_str(line);
    }

    return Ok(this);
  }
}
