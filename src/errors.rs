use std::io;

use crate::parser::{LasValue, SectionKind};

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
    AsciiColumnsMismatch {
        line_number: usize,
        num_cols_from_curve_section: usize,
        num_cols_in_ascii_section: usize,
    },
    ConvertingTo {
        format: String,
    },
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}
