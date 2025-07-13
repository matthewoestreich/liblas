use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;
use std::path::PathBuf;

use crate::{errors::LasioError::*, *};

#[derive(Debug)]
pub struct LasFile {
  pub file_path: PathBuf,
  pub version_info: Option<VersionInformation>,
  pub well_info: Option<WellInformation>,
  pub ascii_log_data: Option<AsciiLogData>,
  pub curve_info: Option<CurveInformation>,
  pub other_info: Option<OtherInformation>,
  pub parameter_info: Option<ParameterInformation>,
}

impl LasFile {
  pub fn new(file_path: PathBuf) -> Self {
    Self {
      file_path,
      version_info: None,
      well_info: None,
      ascii_log_data: None,
      curve_info: None,
      other_info: None,
      parameter_info: None,
    }
  }

  pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
    let file = File::open(&self.file_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().peekable();

    while let Some(read_line) = lines.next() {
      let line = read_line?;

      if line.starts_with("~V") {
        self.parse_version_information(&mut lines)?;
        continue;
      }
      if line.starts_with("~O") {
        self.parse_other_information(&mut lines)?;
        continue;
      }
      if line.starts_with("~P") {
        self.parse_parameter_information(&mut lines)?;
        continue;
      }
      if line.starts_with("~C") {
        continue;
      }
      if line.starts_with("~A") {
        continue;
      }
    }

    return Ok(());
  }

  fn chop_section(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<Vec<String>, LasioError> {
    let mut section: Vec<String> = vec![];

    while let Some(Ok(peeked_line)) = lines.peek() {
      if peeked_line.starts_with('~') {
        break;
      }
      let next_line = lines.next().ok_or(ReadingNextLine)?.map_err(|_| ReadingNextLine)?;
      section.push(next_line);
    }

    return Ok(section);
  }

  fn parse_version_information(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<(), LasioError> {
    let v_lines = self.chop_section(lines)?;
    self.version_info = Some(VersionInformation::from_lines(v_lines)?);
    return Ok(());
  }

  fn parse_other_information(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<(), LasioError> {
    let o_lines = self.chop_section(lines)?;
    self.other_info = Some(o_lines.join(" "));
    return Ok(());
  }

  fn parse_parameter_information(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<(), LasioError> {
    let p_lines = self.chop_section(lines)?;
    self.parameter_info = Some(ParameterInformation::from_lines(p_lines)?);
    return Ok(());
  }

  pub fn to_json(&self) {}
}
