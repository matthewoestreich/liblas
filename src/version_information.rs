use crate::{LibLasError, Mnemonic};
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
}

impl VersionInformation {
  pub fn from_lines(lines: Vec<String>) -> Result<VersionInformation, LibLasError> {
    let mut vi = VersionInformation::default();

    for line in lines {
      if line.starts_with("VERS") {
        vi.version = Mnemonic::from_line(&line)?;
      } else if line.starts_with("WRAP") {
        vi.wrap = Mnemonic::from_line(&line)?;
      } else {
        let x = Mnemonic::from_line(&line)?;
        vi.extra.insert(x.name.clone(), x);
      }
    }

    return Ok(vi);
  }
}
