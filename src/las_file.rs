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
  #[rustfmt::skip]
  pub fn parse(file_path: PathBuf) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    let file = File::open(file_path).or(Err(OpeningLasFile))?;
    let reader = BufReader::new(file);
    let mut line_reader = reader.lines().peekable();

    // Any comments that live before any given section will be stored here and passed into each sections "parser".
    let mut current_comments: Vec<String> = vec![];

    while let Some(read_line) = line_reader.next() {
      let current_line = read_line.or(Err(ReadingNextLine))?;

      if current_line.starts_with(&Token::Comment()) {
        current_comments.push(current_line.clone());
        // Is this the only comment?
        let parsed_comments = this.parse_comments(&mut line_reader)?;
        current_comments.extend(parsed_comments);
      }

      // The "else if" statements are formatted like this (with lines in between each statement)
      // as an attempt to make things more legible.

      else if current_line.starts_with(&Token::VersionInformationSection()) {
        if this.version_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Version Information' section may exist per .las file!".into(),
          ));
        }
        this.version_information = VersionInformation::parse(&mut line_reader, &mut current_comments)?;
      }

      else if current_line.starts_with(&Token::WellInformationSection()) {
        if this.well_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Well Information' section may exist per .las file!".into(),
          ));
        }
        this.well_information = WellInformation::parse(&mut line_reader, &mut current_comments)?;
      }

      else if current_line.starts_with(&Token::OtherSection()) {
        if let Some(other_info) = this.other_information
          && other_info.is_parsed
        {
          return Err(InvalidLasFile(
            "Only one '~Other Information' section may exist per .las file!".into(),
          ));
        }
        this.other_information = Some(OtherInformation::parse(&mut line_reader, &mut current_comments)?);
      }

      else if current_line.starts_with(&Token::ParameterInformationSection()) {
        if let Some(param_info) = this.parameter_information
          && param_info.is_parsed
        {
          return Err(InvalidLasFile(
            "Only one '~Parameter Information' section may exist per .las file!".into(),
          ));
        }
        this.parameter_information = Some(ParameterInformation::parse(&mut line_reader, &mut current_comments)?);
      }

      else if current_line.starts_with(&Token::CurveInformationSection()) {
        if this.curve_information.is_parsed {
          return Err(InvalidLasFile(
            "Only one '~Curve Information' section may exist per .las file!".into(),
          ));
        }
        this.curve_information = CurveInformation::parse(&mut line_reader, &mut current_comments)?;
      }

      else if current_line.starts_with(&Token::AsciiSection()) {
        if this.ascii_log_data.is_parsed {
          return Err(InvalidLasFile("Only one '~A' section may exist per .las file!".into()));
        }
        this.ascii_log_data = AsciiLogData::parse(
          &mut line_reader,
          current_line,
          &this.curve_information,
          &mut current_comments,
        )?;
      }

      if !this.was_version_info_parsed_first() {
        return Err(InvalidLasFile(
          "'~Version Information' must be the first section in a .las file!".into(),
        ));
      }
    }

    return Ok(this);
  }

  pub fn new(
    version_information: VersionInformation,
    well_information: WellInformation,
    curve_information: CurveInformation,
    ascii_log_data: AsciiLogData,
    other_information: Option<OtherInformation>,
    parameter_information: Option<ParameterInformation>,
  ) -> Self {
    return Self {
      version_information,
      well_information,
      curve_information,
      ascii_log_data,
      other_information,
      parameter_information,
    };
  }

  pub fn to_json_str(&self) -> Result<String, LibLasError> {
    return serde_json::to_string_pretty(self).map_err(|e| return ConvertingToJson(e.to_string()));
  }

  // Convert this structure back into .las format
  pub fn to_las_str(&self) -> String {
    let mut output = format!(
      "{}\n{}\n{}",
      self.version_information.to_str(),
      self.well_information.to_str(),
      self.curve_information.to_str()
    );

    if let Some(param_info) = &self.parameter_information {
      if let Some(param_info_str) = param_info.to_str() {
        output = format!("{output}\n{param_info_str}");
      }
    }
    if let Some(other_info) = &self.other_information {
      if let Some(other_info_str) = other_info.to_str() {
        output = format!("{output}\n{other_info_str}");
      }
    }

    output = format!("{output}\n{}", self.ascii_log_data.to_str());
    return output;
  }

  // According to the spec, the "~Version Information" section must appear first!
  // (from spec) "This section is mandatory and must appear as the first section in the file"
  fn was_version_info_parsed_first(&self) -> bool {
    return !(!self.version_information.is_parsed
      && (self.ascii_log_data.is_parsed
        || self.curve_information.is_parsed
        || self.well_information.is_parsed
        || self.parameter_information.is_some()
        || self.other_information.is_some()));
  }

  // Consumes all comment lines up until the next line (which is viewed by peeking) isn't a comment.
  fn parse_comments(&self, line_reader: &mut PeekableFileReader) -> Result<Vec<String>, LibLasError> {
    let mut comments: Vec<String> = vec![];
    while let Some(Ok(peeked_line)) = line_reader.peek() {
      if !peeked_line.trim().to_string().starts_with(&Token::Comment()) {
        break;
      }
      let next_line = line_reader.next().ok_or(ReadingNextLine)??.trim().to_string();
      comments.push(next_line);
    }
    return Ok(comments);
  }
}
