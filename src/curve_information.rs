use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct CurveInformation {
  pub curves: Vec<Mnemonic>,
  pub(crate) is_parsed: bool,
}

impl CurveInformation {
  pub fn parse(reader: &mut PeekableFileReader) -> Result<CurveInformation, LibLasError> {
    let mut this = CurveInformation::default();

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.starts_with("~") {
        break;
      }
      let line = reader.next().ok_or(ReadingNextLine)??;
      // TODO : SKIPPING COMMENTS FOR NOW
      if line.starts_with("#") {
        continue;
      }
      let mnemonic = Mnemonic::from_line(&line)?;
      this.curves.push(mnemonic);
    }

    this.is_parsed = true;
    return Ok(this);
  }
}

impl Serialize for CurveInformation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    return self.curves.serialize(serializer);
  }
}

impl<'de> Deserialize<'de> for CurveInformation {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let curves = Vec::<Mnemonic>::deserialize(deserializer)?;
    let is_parsed = !curves.is_empty();
    return Ok(CurveInformation { curves, is_parsed });
  }
}
