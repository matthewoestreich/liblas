use std::{fmt, io};

use crate::parse::{LasValue, SectionKind};

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    MissingSection {
        section: SectionKind,
    },
    MissingCurveSectionOrAsciiLogsNotLastSectioon,
    MissingMultipleSections {
        missing_sections: Vec<SectionKind>,
    },
    UnexpectedSection {
        expected: SectionKind,
        got: SectionKind,
    },
    MissingRequiredKey {
        key: String,
        line_number: usize,
        line: String,
    },
    MissingDelimiter {
        delimiter: String,
        line_number: usize,
        line: String,
    },
    SectionMissingRequiredData {
        section: SectionKind,
        one_of: Vec<String>,
    },
    InvalidWellValue {
        mnemonic: String,
        value: Option<LasValue>,
    },
    WellDataMissingRequiredValueForMnemonic {
        mnemonic: String,
    },
    DelimetedValueContainsInvalidChars {
        key: String,
        line_number: usize,
        invalid_chars: Vec<char>,
        line: String,
    },
    DuplicateSection {
        section: SectionKind,
        line_number: usize,
        duplicate_line_number: usize,
    },
    VersionInformationNotFirst {
        line_number: usize,
    },
    AsciiLogDataSectionNotLast {
        line_number: usize,
    },
    InvalidAsciiValue {
        raw_value: String,
        line_number: usize,
    },
    InvalidAsciiFloatValue {
        raw_value: String,
    },
    AsciiColumnsMismatch {
        line_number: usize,
        num_cols_in_headers: usize,
        num_cols_in_row: usize,
    },
    AsciiDataContainsEmptyLine {
        line_number: usize,
    },
    CurvesAndAsciiDataColumnsMismatch {
        num_curves: usize,
        num_data_cols: usize,
        curves_line_number: usize,
        ascii_data_line_number: usize,
    },
    ConvertingTo {
        format: String,
    },
    DisallowedFirstCurve {
        got: String,
        expected_one_of: Vec<String>,
    },
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io(error) => write!(f, "ParseError::Io({:?})", error),
            ParseError::MissingSection { section } => {
                write!(f, "ParseError::MissingSection({:?})", section)
            }
            ParseError::MissingCurveSectionOrAsciiLogsNotLastSectioon => {
                write!(f, "ParseError::MissingCurveSectionOrAsciiLogsNotLastSectioon")
            }
            ParseError::MissingMultipleSections { missing_sections } => {
                write!(f, "ParseError::MissingMultipleSections({:?})", missing_sections)
            }
            ParseError::UnexpectedSection { expected, got } => {
                write!(
                    f,
                    "ParseError::UnexpectedSection(expected={:?}, got={:?})",
                    expected, got
                )
            }
            ParseError::MissingRequiredKey { key, line_number, line } => {
                write!(
                    f,
                    "ParseError::MissingRequiredKey(key={:?}, line_number={}, line={:?})",
                    key, line_number, line
                )
            }
            ParseError::MissingDelimiter {
                delimiter,
                line_number,
                line,
            } => {
                write!(
                    f,
                    "ParseError::MissingDelimiter(delimiter={:?}, line_number={}, line={:?})",
                    delimiter, line_number, line
                )
            }
            ParseError::SectionMissingRequiredData { section, one_of } => {
                write!(
                    f,
                    "ParseError::SectionMissingRequiredData(section={:?}, one_of={:?})",
                    section, one_of
                )
            }
            ParseError::InvalidWellValue { mnemonic, value } => {
                write!(
                    f,
                    "ParseError::InvalidWellValue(mnemonic={:?}, value={:?})",
                    mnemonic, value
                )
            }
            ParseError::WellDataMissingRequiredValueForMnemonic { mnemonic } => {
                write!(f, "ParseError::WellDataMissingRequiredValueForMnemonic({:?})", mnemonic)
            }
            ParseError::DelimetedValueContainsInvalidChars {
                key,
                line_number,
                invalid_chars,
                line,
            } => {
                write!(
                    f,
                    "ParseError::DelimetedValueContainsInvalidChars(key={:?}, line_number={}, invalid_chars={:?}, line={:?})",
                    key, line_number, invalid_chars, line
                )
            }
            ParseError::DuplicateSection {
                section,
                line_number,
                duplicate_line_number,
            } => {
                write!(
                    f,
                    "ParseError::DuplicateSection(section={:?}, line_number={}, duplicate_line_number={})",
                    section, line_number, duplicate_line_number
                )
            }
            ParseError::VersionInformationNotFirst { line_number } => {
                write!(f, "ParseError::VersionInformationNotFirst(line_number={})", line_number)
            }
            ParseError::AsciiLogDataSectionNotLast { line_number } => {
                write!(f, "ParseError::AsciiLogDataSectionNotLast(line_number={})", line_number)
            }
            ParseError::InvalidAsciiValue { raw_value, line_number } => {
                write!(
                    f,
                    "ParseError::InvalidAsciiValue(raw_value={:?}, line_number={})",
                    raw_value, line_number
                )
            }
            ParseError::InvalidAsciiFloatValue { raw_value } => {
                write!(f, "ParseError::InvalidAsciiFloatValue({:?})", raw_value)
            }
            ParseError::AsciiColumnsMismatch {
                line_number,
                num_cols_in_headers,
                num_cols_in_row,
            } => {
                write!(
                    f,
                    "ParseError::AsciiColumnsMismatch(line_number={}, headers={}, row={})",
                    line_number, num_cols_in_headers, num_cols_in_row
                )
            }
            ParseError::AsciiDataContainsEmptyLine { line_number } => {
                write!(f, "ParseError::AsciiDataContainsEmptyLine(line_number={})", line_number)
            }
            ParseError::CurvesAndAsciiDataColumnsMismatch {
                num_curves,
                num_data_cols,
                curves_line_number,
                ascii_data_line_number,
            } => {
                write!(
                    f,
                    "ParseError::CurvesAndAsciiDataColumnsMismatch(num_curves={}, num_data_cols={}, curves_line_number={}, ascii_data_line_number={})",
                    num_curves, num_data_cols, curves_line_number, ascii_data_line_number
                )
            }
            ParseError::ConvertingTo { format } => {
                write!(f, "ParseError::ConvertingTo({:?})", format)
            }
            ParseError::DisallowedFirstCurve { got, expected_one_of } => {
                write!(
                    f,
                    "ParseError::DisallowedFirstCurve(got={:?}, expected_one_of={:?})",
                    got, expected_one_of
                )
            }
        }
    }
}
