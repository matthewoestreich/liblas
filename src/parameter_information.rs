use crate::{LasioError, Mnemonic};
use std::collections::HashMap;

/*
• This section is optional. It defines the input values of various parameters relating to this well.
• These input values can consist of numbers or text.
• Only one "~P" section can occur in an LAS 2.0 file.
• The mnemonics used are not restricted but must be defined on the line on which they appear.
• There is no limit on the number of lines that can be used.
• The following is an example of a Parameter Information Section.
*/

#[derive(Debug)]
pub struct ParameterInformation(HashMap<String, Mnemonic>);

impl Default for ParameterInformation {
  fn default() -> Self {
    return Self(HashMap::<String, Mnemonic>::new());
  }
}

impl ParameterInformation {
  pub fn from_lines(lines: Vec<String>) -> Result<ParameterInformation, LasioError> {
    let mut pi = ParameterInformation::default();

    for line in lines {
      let mnemonic = Mnemonic::from_line(&line)?;
      pi.0.insert(mnemonic.name.clone(), mnemonic);
    }

    return Ok(pi);
  }
}
