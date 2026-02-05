use crate::{KeyValueData, ParseError, Section, SectionEntry, SectionKind, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CurveInformation {
    pub curves: Vec<KeyValueData>,
    pub comments: Option<Vec<String>>,
    pub header: String,
    #[serde(skip)]
    pub(crate) line_number: usize,
}

impl PartialEq for CurveInformation {
    fn eq(&self, other: &Self) -> bool {
        self.curves == other.curves && self.comments == other.comments && self.header == other.header
    }
}

impl Eq for CurveInformation {}

impl fmt::Display for CurveInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_comments(f, &self.comments)?;
        writeln!(f, "{}", self.header)?;
        for curve in self.curves.iter() {
            writeln!(f, "{curve}")?;
        }
        Ok(())
    }
}

impl TryFrom<Section> for CurveInformation {
    type Error = ParseError;

    fn try_from(section: Section) -> Result<Self, Self::Error> {
        if section.header.kind != SectionKind::Curve {
            return Err(ParseError::UnexpectedSection {
                expected: SectionKind::Curve,
                got: section.header.kind,
            });
        }

        let mut curve = CurveInformation::default();

        for entry in section.entries {
            if let SectionEntry::Delimited(kv) = entry {
                curve.curves.push(kv);
            }
        }

        curve.header = format!("~{}", section.header.raw);
        curve.comments = section.comments;
        curve.line_number = section.line;
        Ok(curve)
    }
}
