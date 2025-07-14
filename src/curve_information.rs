use crate::{LibLasError, Mnemonic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CurveInformation(pub HashMap<String, Mnemonic>);

impl CurveInformation {
  pub fn from_lines(lines: Vec<String>) -> Result<CurveInformation, LibLasError> {
    let mut ci = CurveInformation::default();

    for line in lines {
      let mnemonic = Mnemonic::from_line(&line)?;
      ci.0.insert(mnemonic.name.clone(), mnemonic);
    }

    return Ok(ci);
  }
}
