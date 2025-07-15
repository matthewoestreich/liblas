use serde::{Deserialize, Serialize}; /*
• This section is optional. It defines the input values of various parameters relating to this well.
• These input values can consist of numbers or text.
• Only one "~P" section can occur in an LAS 2.0 file.
• The mnemonics used are not restricted but must be defined on the line on which they appear.
• There is no limit on the number of lines that can be used.
• The following is an example of a Parameter Information Section.
*/
use crate::{
  LibLasError::{self, ReadingNextLine},
  Mnemonic, PeekableFileReader,
};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterInformation(HashMap<String, Mnemonic>);

impl ParameterInformation {
  pub fn parse(reader: &mut PeekableFileReader) -> Result<ParameterInformation, LibLasError> {
    let mut this = ParameterInformation::default();

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
      this.0.insert(mnemonic.name.clone(), mnemonic);
    }

    return Ok(this);
  }
}
