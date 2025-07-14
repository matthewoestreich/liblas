use crate::{errors::LibLasError::*, *};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::PathBuf;

type PeakableLine = Peekable<Lines<BufReader<File>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LasFile {
  #[serde(skip)]
  pub file_path: PathBuf,
  #[serde(rename = "VersionInformation")]
  pub version_information: Option<VersionInformation>,
  #[serde(rename = "WellInformation")]
  pub well_information: Option<WellInformation>,
  #[serde(rename = "AsciiLogData")]
  pub ascii_log_data: Option<AsciiLogData>,
  #[serde(rename = "CurveInformation")]
  pub curve_information: Option<CurveInformation>,
  #[serde(rename = "OtherInformation")]
  pub other_information: Option<OtherInformation>,
  #[serde(rename = "ParameterInformation")]
  pub parameter_information: Option<ParameterInformation>,
}

impl LasFile {
  pub fn new(file_path: PathBuf) -> Self {
    return Self {
      file_path,
      version_information: None,
      well_information: None,
      ascii_log_data: None,
      curve_information: None,
      other_information: None,
      parameter_information: None,
    };
  }

  pub fn to_json_str(&self) -> Result<String, LibLasError> {
    return serde_json::to_string_pretty(self).map_err(|e| return ConvertingToJson(e.to_string()));
  }

  pub fn parse(&mut self) -> Result<(), LibLasError> {
    let file = File::open(&self.file_path).or(Err(OpeningLasFile))?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().peekable();

    while let Some(read_line) = lines.next() {
      let line = read_line.or(Err(LibLasError::ReadingNextLine))?;

      if line.starts_with("~V") {
        self.parse_version_information(&mut lines)?;
      } else if line.starts_with("~W") {
        self.parse_well_information(&mut lines)?;
      } else if line.starts_with("~O") {
        self.parse_other_information(&mut lines)?;
      } else if line.starts_with("~P") {
        self.parse_parameter_information(&mut lines)?;
        // "~C" MUST **ALWAYS** COME BEFORE "~A"
      } else if line.starts_with("~C") {
        self.parse_curve_information(&mut lines)?;
      } else if line.starts_with("~A") {
        self.parse_ascii_data(line.clone(), &mut lines)?;
        // ^^^^^^ "~C" MUST **ALWAYS** COME BEFORE "~A"
      }
    }

    return Ok(());
  }

  fn chop_section(&mut self, lines: &mut PeakableLine) -> Result<Vec<String>, LibLasError> {
    let mut section: Vec<String> = vec![];

    while let Some(Ok(peeked_line)) = lines.peek() {
      if peeked_line.starts_with('~') {
        break;
      }
      let next_line = lines
        .next()
        .ok_or(ReadingNextLine)?
        .map_err(|_| return ReadingNextLine)?;
      // TODO : SKIPPING COMMENTS FOR NOW
      if !next_line.starts_with("#") {
        section.push(next_line);
      }
    }

    return Ok(section);
  }

  fn parse_version_information(&mut self, lines: &mut PeakableLine) -> Result<(), LibLasError> {
    if self.version_information.is_some() {
      return Err(DuplicateSectionFound("~Version Information".to_string()));
    }
    let v_lines = self.chop_section(lines)?;
    self.version_information = Some(VersionInformation::from_lines(v_lines)?);
    return Ok(());
  }

  fn parse_well_information(&mut self, lines: &mut PeakableLine) -> Result<(), LibLasError> {
    if self.well_information.is_some() {
      return Err(DuplicateSectionFound("~Well Information".to_string()));
    }
    let w_lines = self.chop_section(lines)?;
    self.well_information = Some(WellInformation::from_lines(w_lines)?);
    return Ok(());
  }

  fn parse_other_information(&mut self, lines: &mut PeakableLine) -> Result<(), LibLasError> {
    let o_lines = self.chop_section(lines)?;
    self.other_information = Some(OtherInformation(o_lines.join(" ")));
    return Ok(());
  }

  fn parse_parameter_information(&mut self, lines: &mut PeakableLine) -> Result<(), LibLasError> {
    let p_lines = self.chop_section(lines)?;
    self.parameter_information = Some(ParameterInformation::from_lines(p_lines)?);
    return Ok(());
  }

  fn parse_curve_information(&mut self, lines: &mut PeakableLine) -> Result<(), LibLasError> {
    if self.curve_information.is_some() {
      return Err(DuplicateSectionFound("~Curve Information".to_string()));
    }
    let c_lines = self.chop_section(lines)?;
    self.curve_information = Some(CurveInformation::from_lines(c_lines)?);
    return Ok(());
  }

  fn parse_ascii_data(&mut self, current_line: String, rest_of_lines: &mut PeakableLine) -> Result<(), LibLasError> {
    let mut a_lines: Vec<String> = vec![current_line];
    a_lines.extend(self.chop_section(rest_of_lines)?);

    let ci = self
      .curve_information
      .as_ref()
      .ok_or(CurveInfoRequiredToParseAsciiLogData)?;

    self.ascii_log_data = Some(AsciiLogData::from_lines(a_lines, ci)?);

    if rest_of_lines.next().is_some() {
      return Err(AsciiLogDataNotLastSection);
    }
    return Ok(());
  }
}
