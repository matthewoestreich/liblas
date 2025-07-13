use std::collections::HashMap;

use crate::{LasioError, Mnemonic};

#[derive(Default, Debug)]
pub struct VersionInformation {
  pub version: Mnemonic,
  pub wrap: Mnemonic,
  pub extra: HashMap<String, Mnemonic>,
}

impl VersionInformation {
  pub fn from_lines(lines: Vec<String>) -> Result<VersionInformation, LasioError> {
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
