use crate::{DataLine, ParseError, Section, SectionEntry, SectionKind, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionInformation {
    #[serde(rename = "VERS")]
    pub version: DataLine,
    #[serde(rename = "WRAP")]
    pub wrap: DataLine,
    pub additional: Vec<DataLine>,
    pub comments: Option<Vec<String>>,
    pub header: String,
    #[serde(skip)]
    pub(crate) line_number: usize,
}

impl PartialEq for VersionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
            && self.wrap == other.wrap
            && self.additional == other.additional
            && self.comments == other.comments
            && self.header == other.header
    }
}

impl Eq for VersionInformation {}

impl fmt::Display for VersionInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.wrap)?;
        for addition in self.additional.iter() {
            writeln!(f, "{addition}")?;
        }
        Ok(())
    }
}

impl TryFrom<Section> for VersionInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Version {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Version,
                got: section.header.kind,
            });
        }

        let mut version = VersionInformation::default();
        let mut has_vers = false;
        let mut has_wrap = false;

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                match kv.mnemonic.to_lowercase().as_str() {
                    "vers" => {
                        version.version = kv;
                        has_vers = true;
                    }
                    "wrap" => {
                        version.wrap = kv;
                        has_wrap = true;
                    }
                    _ => version.additional.push(kv),
                };
            }
        }

        if !has_vers || !has_wrap {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Version,
                one_of: vec!["VERS".to_string(), "WRAP".to_string()],
            });
        }

        version.header = format!("~{}", section.header.raw);
        version.comments = section.comments;
        version.line_number = section.line;
        Ok(version)
    }
}
