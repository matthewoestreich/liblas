use crate::{errors::LibLasError::*, *};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LasFile {
  #[serde(rename = "VersionInformation")]
  pub version_information: VersionInformation,
  #[serde(rename = "WellInformation")]
  pub well_information: WellInformation,
  #[serde(rename = "AsciiLogData")]
  pub ascii_log_data: AsciiLogData,
  #[serde(rename = "CurveInformation")]
  pub curve_information: CurveInformation,
  #[serde(rename = "OtherInformation")]
  pub other_information: Option<OtherInformation>,
  #[serde(rename = "ParameterInformation")]
  pub parameter_information: Option<ParameterInformation>,
}

impl LasFile {
  pub fn parse(file_path: PathBuf) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    let file = File::open(file_path).or(Err(OpeningLasFile))?;
    let reader = BufReader::new(file);

    let mut line_reader = reader.lines().peekable();

    while let Some(read_line) = line_reader.next() {
      let current_line = read_line.or(Err(ReadingNextLine))?;

      if current_line.starts_with("~V") {
        if this.version_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Version Information' section may exist per .las file!".into(),
          ));
        }
        this.version_information = VersionInformation::parse(&mut line_reader)?;
      } else if current_line.starts_with("~W") {
        if this.well_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Well Information' section may exist per .las file!".into(),
          ));
        }
        this.well_information = WellInformation::parse(&mut line_reader)?;
      } else if current_line.starts_with("~O") {
        this.other_information = Some(OtherInformation::parse(&mut line_reader)?);
      } else if current_line.starts_with("~P") {
        this.parameter_information = Some(ParameterInformation::parse(&mut line_reader)?);
      } else if current_line.starts_with("~C") {
        if this.curve_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Curve Information' section may exist per .las file!".into(),
          ));
        }
        this.curve_information = CurveInformation::parse(&mut line_reader)?;
      } else if current_line.starts_with("~A") {
        if this.ascii_log_data.is_parsed {
          return Err(InvalidLasFile("Only one '~A' section may exist per .las file!".into()));
        }
        this.ascii_log_data = AsciiLogData::parse(&mut line_reader, current_line, &this.curve_information)?;
      }
    }

    return Ok(this);
  }

  pub fn to_json_str(&self) -> Result<String, LibLasError> {
    return serde_json::to_string_pretty(self).map_err(|e| return ConvertingToJson(e.to_string()));
  }
}
