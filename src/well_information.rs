/*
  ~W (Well Information)
  • This section is mandatory.
  • Only one "~W" section can occur in an LAS 2.0 file.
  • It identifies the well, its unique location identifier and indicates the start and stop depths (or
  time, or index number) of the file.
  • This section must contain the following lines with the mnemonics as indicated:
  STRT.M nnn.nn : START DEPTH
  Refers to the first depth (or time, or index number) in the file. The "nnn.nn" refers to the
  depth (or time or index) value. The value must be identical in value to the first depth (time,
  index) in the ~ASCII section although its format may vary (123.45 is equivalent to
  123.45000).
  The number of decimals used is not restricted. If the index is depth, the units must be M
  (metres), F (feet) or FT (feet). Units must match on the lines relating to STRT, STOP, STEP
  and the index (first) channel in the ~C section. If time or index the units can be any unit that
  results in a floating point number representation of time or the index number. (dd/mm/yy
  or hh:mm:ss formats are not supported). The logical depth, time or index order can be
  6
  2017-01
  increasing or decreasing. The start depth (or time, or index) value when divided by the step
  depth (or time or index) value must be a whole number.
  STOP.M nnn.n : STOP DEPTH
  Same comments as for STRT except this value represents the LAST data line in the ~ASCII
  log data section. The stop depth (or time or index) value when divided by the step depth (or
  time or index) value must be a whole number.
  STEP.M nnn.nn : STEP
  Same comments as for STRT, except this value represents the actual difference between
  every successive depth, time or index values in the ~ASCII log data section. The sign (+ or -)
  represents the logical difference between each successive index value. (+ for increasing
  index values). The step must be identical in value between every index value throughout
  the file. If the step increment is not exactly consistent between every depth, time or index
  sample, then the step must have a value of 0.
  NULL. nnnn.nn : NULL VALUE
  Refers to null values. Commonly used null values are -9999, -999.25 and -9999.25.
  COMP. aaaaaaaaaaaaaaaaaaaaa : COMPANY
  Refers to company name.
  WELL. aaaaaaaaaaaaaaaaaaaaa : WELL
  Refers to the well name.
  FLD. aaaaaaaaaaaaaaaaaaaaa : FIELD
  Refers to the field name.
  LOC. aaaaaaaaaaaaaaaaaaaaa : LOCATION
  Refers to the well location.
  PROV. aaaaaaaaaaaaaaaaaaaaa : PROVINCE
  Refers to the province. For areas outside Canada this line may be
  replaced by:
  CNTY. aaaaaaaaaaaaaaaaaaaaa : COUNTY
  STAT. aaaaaaaaaaaaaaaaaaaaa : STATE
  CTRY. aaaaaaaaaaaaaaaaaaaaa : COUNTRY
  SRVC. aaaaaaaaaaaaaaaaaaaaa : SERVICE COMPANY
  Refers to logging/service company.
  DATE. aaaaaaaaaaaaaaaaaaaaa : DATE
  Refers to date logged. The preferred date is of the form yyyy-mm-dd
  UWI . aaaaaaaaaaaaaaaaaaaaa : UNIQUE WELL ID
  Refers to unique well identifier. Within Canada, the most common UWI consists of a 16
  character string. Excluding all dashes, slashes and spaces from such UWIs makes it easier
  for software to parse them.
  7
  2017-01
  For areas in the United States this may be replaced by:
  API . aaaaaaaaaaaaaaaaaaaaa : API NUMBER
  • Additional lines in the well information section are optional. There is no limit on the number of
  additional lines.
  LIC. nnnnnn : LICENCE NUMBER
  Refers to a regulatory licence number. Required by ERCB in Alberta
  • The following is an example of a Well Information Section in LAS version 2.0:
  _____________________________________________________________________
  ~Well Information Section
  #MNEM.UNIT VALUE/NAME DESCRIPTION
  #-------- -------------- ---------------------
  STRT.M 635.0000 :START DEPTH
  STOP.M 400.0000 :STOP DEPTH
  STEP.M -0.125 :STEP
  NULL. -999.25 :NULL VALUE
  COMP. ANY OIL COMPANY INC. :COMPANY
  WELL. ANY ET AL 12-34-12-34 :WELL
  FLD . WILDCAT :FIELD
  LOC . 12-34-12-34W5M :LOCATION
  PROV. ALBERTA :PROVINCE
  SRVC. ANY LOGGING COMPANY INC. :SERVICE COMPANY
  LIC . 12345 :ERCB LICENCE NUMBER
  DATE. 13-DEC-86 :LOG DATE
  UWI . 100123401234W500 :UNIQUE WELL ID
*/
use crate::{LibLasError, Mnemonic, PeekableFileReader, errors::LibLasError::*};
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
  #[serde(skip)]
  pub is_parsed: bool,
}

impl WellInformation {
  pub fn parse(reader: &mut PeekableFileReader) -> Result<WellInformation, LibLasError> {
    let mut this = WellInformation::default();

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.starts_with("~") {
        break;
      }

      let line = reader.next().ok_or(ReadingNextLine)??;

      // TODO : SKIPPING COMMENTS FOR NOW
      if line.starts_with("#") {
        continue;
      }

      if line.starts_with("STRT") {
        this.strt = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STOP") {
        this.stop = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STEP") {
        this.step = Mnemonic::from_line(&line)?;
      } else if line.starts_with("NULL") {
        this.null = Mnemonic::from_line(&line)?;
      } else if line.starts_with("COMP") {
        this.comp = Mnemonic::from_line(&line)?;
      } else if line.starts_with("WELL") {
        this.well = Mnemonic::from_line(&line)?;
      } else if line.starts_with("FLD") {
        this.fld = Mnemonic::from_line(&line)?;
      } else if line.starts_with("LOC") {
        this.loc = Mnemonic::from_line(&line)?;
      } else if line.starts_with("PROV") {
        this.prov = Mnemonic::from_line(&line)?;
      } else if line.starts_with("CNTY") {
        this.cnty = Mnemonic::from_line(&line)?;
      } else if line.starts_with("STAT") {
        this.stat = Mnemonic::from_line(&line)?;
      } else if line.starts_with("CTRY") {
        this.ctry = Mnemonic::from_line(&line)?;
      } else if line.starts_with("SRVC") {
        this.srvc = Mnemonic::from_line(&line)?;
      } else if line.starts_with("DATE") {
        this.date = Mnemonic::from_line(&line)?;
      } else if line.starts_with("UWI") {
        this.uwi = Mnemonic::from_line(&line)?;
      } else if line.starts_with("API") {
        this.api = Mnemonic::from_line(&line)?;
      } else {
        let x = Mnemonic::from_line(&line)?;
        this.extra.insert(x.name.clone(), x);
      }
    }

    // Validate required fields
    let required = [
      ("STRT", &this.strt),
      ("STOP", &this.stop),
      ("STEP", &this.step),
      ("NULL", &this.null),
      ("COMP", &this.comp),
      ("WELL", &this.well),
      ("FLD", &this.fld),
      ("LOC", &this.loc),
      ("SRVC", &this.srvc),
      ("DATE", &this.date),
      //////////////////////
      //("PROV", &wi.prov), | -------------------------------------------
      //("CNTY", &wi.cnty), | One of PROV, CNTY, STAT, CTRY must exist!
      //("STAT", &wi.stat), | -------------------------------------------
      //("CTRY", &wi.ctry), | -------------------------------------------
      //////////////////////
      //////////////////////
      //("UWI", &wi.uwi), | either UWI (Canada) or,
      //("API", &wi.api), | API (USA) is required!
      //////////////////////
    ];

    for (field_name, mnemonic) in required.iter() {
      if mnemonic.name.trim().is_empty() {
        let mut e = "[~Well Information] -> ".to_owned();
        e.push_str(field_name);
        return Err(MissingRequiredMnemonicField(e));
      }
    }

    let one_of_prov_cnty_ctry_state_must_exist = [(
      ("PROV", &this.prov),
      ("CTRY", &this.ctry),
      ("CNTY", &this.cnty),
      ("STAT", &this.stat),
    )];

    for (pair_a, pair_b, pair_c, pair_d) in one_of_prov_cnty_ctry_state_must_exist.iter() {
      if pair_a.1.name.trim().is_empty()
        && pair_b.1.name.trim().is_empty()
        && pair_c.1.name.trim().is_empty()
        && pair_d.1.name.trim().is_empty()
      {
        let e = "[~Well Information] Must have one of PROV, CNTY, CTRY, STAT! ->".to_owned();
        return Err(MissingRequiredMnemonicField(e));
      }
    }

    if this.uwi.name.trim().is_empty() && this.api.name.trim().is_empty() {
      let e = "[~Well Information] Must have one of API or UWI! ->".to_owned();
      return Err(MissingRequiredMnemonicField(e));
    }

    this.is_parsed = true;
    return Ok(this);
  }
}
