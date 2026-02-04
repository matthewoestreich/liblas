use crate::{KeyValueData, ParseError, Section, SectionEntry, SectionKind, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParameterInformation {
    pub parameters: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,

    #[serde(skip)]
    pub(crate) line_number: usize,
    #[serde(skip)]
    pub(crate) header: String,
}

impl fmt::Display for ParameterInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for param in self.parameters.iter() {
            writeln!(f, "{param}")?;
        }
        Ok(())
    }
}

impl TryFrom<Section> for ParameterInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Parameter {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Parameter,
                got: section.header.kind,
            });
        }

        let mut parameter = ParameterInformation::default();

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                parameter.parameters.push(kv);
            }
        }

        parameter.header = format!("~{}", section.header.raw);
        parameter.comments = section.comments;
        parameter.line_number = section.line;
        Ok(parameter)
    }
}
