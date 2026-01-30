use std::io;

use crate::vtwo::section::SectionKind;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    MissingSection {
        section: SectionKind,
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
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}
