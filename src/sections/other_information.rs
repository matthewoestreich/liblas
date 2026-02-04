use crate::{ParseError, Section, SectionEntry, SectionKind, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformationData {
    pub text: String,
    pub comments: Option<Vec<String>>,
}

impl fmt::Display for OtherInformationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.text)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformation {
    pub data: Vec<OtherInformationData>,
    pub comments: Option<Vec<String>>,

    #[serde(skip)]
    pub(crate) line_number: usize,
    #[serde(skip)]
    pub(crate) header: String,
}

impl fmt::Display for OtherInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for info in self.data.iter() {
            write!(f, "{info}")?;
        }
        Ok(())
    }
}

impl TryFrom<Section> for OtherInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Other {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Other,
                got: section.header.kind,
            });
        }

        let mut other = OtherInformation::default();

        for entry in section.entries {
            if let SectionEntry::Raw { text, comments } = entry {
                other.data.push(OtherInformationData { text, comments });
            }
        }

        other.header = format!("~{}", section.header.raw);
        other.comments = section.comments;
        other.line_number = section.line;
        Ok(other)
    }
}
