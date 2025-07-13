use crate::{LibLasError, Mnemonic};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WellInformation {
  #[serde(rename = "STRT")]
  pub strt: Mnemonic,
  #[serde(rename = "STOP")]
  pub stop: Mnemonic,
  #[serde(rename = "STEP")]
  pub step: Mnemonic,
  #[serde(rename = "NULL")]
  pub null: Mnemonic,
  #[serde(rename = "COMP")]
  pub comp: Mnemonic,
  #[serde(rename = "WELL")]
  pub well: Mnemonic,
  #[serde(rename = "FLD")]
  pub fld: Mnemonic,
  #[serde(rename = "LOC")]
  pub loc: Mnemonic,
  #[serde(rename = "PROV")]
  pub prov: Mnemonic,
  #[serde(rename = "CNTY")]
  pub cnty: Mnemonic,
  #[serde(rename = "STAT")]
  pub stat: Mnemonic,
  #[serde(rename = "CTRY")]
  pub ctry: Mnemonic,
  #[serde(rename = "SRVC")]
  pub srvc: Mnemonic,
  #[serde(rename = "DATE")]
  pub date: Mnemonic,
  #[serde(rename = "UWI")]
  pub uwi: Mnemonic,
  #[serde(rename = "API")]
  pub api: Mnemonic,
  #[serde(flatten)]
  pub extra: HashMap<String, Mnemonic>,
}

impl WellInformation {
  pub fn from_lines(lines: Vec<String>) -> Result<WellInformation, LibLasError> {
    let mut wi = WellInformation::default();

    for line in lines {
      if line.starts_with("STRT") {
        wi.strt = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STOP") {
        wi.stop = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STEP") {
        wi.step = Mnemonic::from_line(&line)?;
      } else if line.starts_with("NULL") {
        wi.null = Mnemonic::from_line(&line)?;
      } else if line.starts_with("COMP") {
        wi.comp = Mnemonic::from_line(&line)?;
      } else if line.starts_with("WELL") {
        wi.well = Mnemonic::from_line(&line)?;
      } else if line.starts_with("FLD") {
        wi.fld = Mnemonic::from_line(&line)?;
      } else if line.starts_with("LOC") {
        wi.loc = Mnemonic::from_line(&line)?;
      } else if line.starts_with("PROV") {
        wi.prov = Mnemonic::from_line(&line)?;
      } else if line.starts_with("CNTY") {
        wi.cnty = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STAT") {
        wi.stat = Mnemonic::from_line(&line)?;
      } else if line.starts_with("CTRY") {
        wi.ctry = Mnemonic::from_line(&line)?;
      } else if line.starts_with("SRVC") {
        wi.srvc = Mnemonic::from_line(&line)?;
      } else if line.starts_with("DATE") {
        wi.date = Mnemonic::from_line(&line)?;
      } else if line.starts_with("UWI") {
        wi.uwi = Mnemonic::from_line(&line)?;
      } else if line.starts_with("API") {
        wi.api = Mnemonic::from_line(&line)?;
      } else {
        let x = Mnemonic::from_line(&line)?;
        wi.extra.insert(x.name.clone(), x);
      }
    }

    return Ok(wi);
  }
}
