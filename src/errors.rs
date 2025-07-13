use std::{
  error::Error,
  fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum LibLasError {
  UnknownSection(String),
  MissingRequiredMnemonicField(String),
  MissingRequiredDelimeter(String),
  UnableToParseDataValue(String),
  DuplicateSectionFound(String),
  ReadingNextLine,
  MissingData(String),
  MalformedAsciiData(String),
}

impl Error for LibLasError {}

impl Display for LibLasError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    use LibLasError::*;
    match self {
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
