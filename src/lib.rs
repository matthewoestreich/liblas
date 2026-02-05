pub mod sections;
pub use errors::*;

#[cfg(test)]
mod tests;

mod errors;

pub(crate) mod parse;
pub(crate) mod tokenizer;

use crate::{parse::*, sections::*, tokenizer::*};
use serde::{self, Deserialize, Serialize};
use std::{fmt, fs::File, io::BufReader};

pub(crate) fn any_present<T>(items: &[&Option<T>]) -> bool {
    items.iter().any(|o| o.is_some())
}

pub(crate) fn write_kv_opt(f: &mut fmt::Formatter<'_>, kv: &Option<KeyValueData>) -> fmt::Result {
    if let Some(v) = kv {
        writeln!(f, "{v}")?;
    }
    Ok(())
}

pub(crate) fn write_comments(f: &mut fmt::Formatter<'_>, comments: &Option<Vec<String>>) -> fmt::Result {
    if let Some(cs) = comments {
        for c in cs {
            let fc = format!("# {c}").trim().to_string();
            writeln!(f, "{fc}")?;
        }
    }
    Ok(())
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LasFile {
    #[serde(rename = "VersionInformation")]
    pub version_information: VersionInformation,
    #[serde(rename = "WellInformation")]
    pub well_information: WellInformation,
    #[serde(rename = "AsciiLogData")]
    pub ascii_log_data: AsciiLogData,
    #[serde(rename = "CurveInformation")]
    pub curve_information: CurveInformation,
    #[serde(rename = "OtherInformation")]
    pub other_information: Option<OtherInformation>,
    #[serde(rename = "ParameterInformation")]
    pub parameter_information: Option<ParameterInformation>,
}

impl fmt::Display for LasFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version_information)?;
        write!(f, "{}", self.well_information)?;
        write!(f, "{}", self.curve_information)?;
        if let Some(parameter) = self.parameter_information.as_ref() {
            write!(f, "{parameter}")?;
        }
        if let Some(other) = self.other_information.as_ref() {
            write!(f, "{other}")?;
        }
        write!(f, "{}", self.ascii_log_data)
    }
}

impl LasFile {
    pub fn try_from_json_str(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn try_from_yaml_str(yaml_str: &str) -> Result<Self, serde_yaml_ng::Error> {
        serde_yaml_ng::from_str(yaml_str)
    }

    pub fn to_json_str(&mut self) -> Result<String, ParseError> {
        serde_json::to_string_pretty(self).map_err(|_| ParseError::ConvertingTo {
            format: "json".to_string(),
        })
    }

    pub fn to_yaml_str(&mut self) -> Result<String, ParseError> {
        serde_yaml_ng::to_string(self).map_err(|_| ParseError::ConvertingTo {
            format: "yaml".to_string(),
        })
    }

    pub fn to_las_str(&mut self) -> String {
        self.to_string()
    }

    pub fn parse(las_file_path: &str) -> Result<Self, ParseError> {
        let reader = BufReader::new(File::open(las_file_path)?);
        let mut parser = LasParser::new(LasTokenizer::new(reader));
        LasFile::try_from(parser.parse()?)
    }
}

impl TryFrom<ParsedLasFile> for LasFile {
    type Error = ParseError;

    fn try_from(file: ParsedLasFile) -> Result<Self, Self::Error> {
        let mut las_file = LasFile::default();

        for section in file.sections {
            match section.header.kind {
                SectionKind::Version => {
                    las_file.version_information = VersionInformation::try_from(section)?;
                }
                SectionKind::Well => {
                    las_file.well_information = WellInformation::try_from(section)?;
                }
                SectionKind::Curve => {
                    las_file.curve_information = CurveInformation::try_from(section)?;
                }
                SectionKind::Parameter => {
                    las_file.parameter_information = Some(ParameterInformation::try_from(section)?);
                }
                SectionKind::Other => {
                    las_file.other_information = Some(OtherInformation::try_from(section)?);
                }
                SectionKind::AsciiLogData => {
                    las_file.ascii_log_data = AsciiLogData::try_from(section)?;
                }
            }
        }

        Ok(las_file)
    }
}
