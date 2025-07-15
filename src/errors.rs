use std::{
  error,
  fmt::{Display, Formatter, Result},
  io, num,
};

#[derive(Debug)]
pub enum LibLasError {
  IoError(io::Error),
  ParseFloatError(num::ParseFloatError),
  InvalidLasFile(String),
  UnknownSection(String),
  MissingRequiredMnemonicField(String),
  MissingRequiredDelimeter(String),
  UnableToParseDataValue(String),
  DuplicateSectionFound(String),
  ReadingNextLine,
  MissingData(String),
  MalformedAsciiData(String),
  OpeningLasFile,
  AsciiLogDataNotLastSection,
  ConvertingToJson(String),
  CurveInfoRequiredToParseAsciiLogData,
  GeneralError(String),
}

impl error::Error for LibLasError {}

impl From<io::Error> for LibLasError {
  fn from(e: io::Error) -> Self {
    return LibLasError::IoError(e);
  }
}

impl From<num::ParseFloatError> for LibLasError {
  fn from(e: num::ParseFloatError) -> Self {
    return LibLasError::ParseFloatError(e);
  }
}

impl Display for LibLasError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    use LibLasError::*;
    #[allow(clippy::implicit_return)]
    match self {
      GeneralError(message) => write!(f, "{message}"),
      IoError(err) => write!(f, "{err}"),
      ParseFloatError(err) => write!(f, "{err}"),
      InvalidLasFile(reason) => write!(f, "Invalid .las file! {reason}"),
      CurveInfoRequiredToParseAsciiLogData => write!(f, "Curve Information is needed to parse ASCII Log Data!"),
      ConvertingToJson(message) => write!(f, "Error converting to JSON! {message}"),
      AsciiLogDataNotLastSection => write!(f, "According to CWLS 2.0, ASCII Log Data must be the last section!"),
      OpeningLasFile => write!(f, "Unable to open .las file!"),
      UnknownSection(section) => write!(f, "Unknown section encountered! '{section}'"),
      MissingRequiredDelimeter(delimeter) => write!(f, "Missing required delimeter! '{delimeter}'"),
      MissingRequiredMnemonicField(field_name) => {
        write!(f, "Missing required mnemonic field! '{field_name}'")
      }
      UnableToParseDataValue(data_value) => write!(f, "Unable to parse data value! '{data_value}'"),
      ReadingNextLine => write!(f, "Error while reading next line!"),
      DuplicateSectionFound(duplicated_section) => {
        write!(f, "Duplicate section found! '{duplicated_section}'")
      }
      MissingData(line) => write!(f, "Missing data! Line = '{line}'"),
      MalformedAsciiData(message) => write!(f, "Malformed ASCII data! Error={message}"),
    }
  }
}
