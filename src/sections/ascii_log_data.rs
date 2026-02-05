use crate::{LasFloat, ParseError, Section, SectionKind, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AsciiLogData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<LasFloat>>,
    pub comments: Option<Vec<String>>,
    pub header: String,
    #[serde(skip)]
    pub(crate) line_number: usize,
}

impl PartialEq for AsciiLogData {
    fn eq(&self, other: &Self) -> bool {
        self.headers == other.headers
            && self.rows == other.rows
            && self.comments == other.comments
            && self.header == other.header
    }
}

impl Eq for AsciiLogData {}

impl fmt::Display for AsciiLogData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for row in self.rows.iter() {
            for cell in row.iter() {
                write!(f, "{} ", cell.raw.clone())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<Section> for AsciiLogData {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::AsciiLogData {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::AsciiLogData,
                got: section.header.kind,
            });
        }
        if section.ascii_headers.is_none() {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::AsciiLogData,
                one_of: vec!["headers".to_string()],
            });
        }

        let mut ascii_logs = AsciiLogData::default();

        if let Some(headers) = section.ascii_headers {
            ascii_logs.headers = headers;
            ascii_logs.rows = section.ascii_rows;
        }

        ascii_logs.header = format!("~{}", section.header.raw);
        ascii_logs.comments = section.comments;
        ascii_logs.line_number = section.line;
        Ok(ascii_logs)
    }
}
