#[cfg(test)]
mod tests;

mod errors;
mod parser;
mod sections;
mod tokenizer;

pub use errors::*;
pub use parser::*;
pub use sections::*;
pub use tokenizer::*;

use serde::{self, Deserialize, Serialize};
use std::{fs::File, io::BufReader};

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

impl LasFile {
    pub fn to_json_str(&mut self) -> Result<String, ParseError> {
        serde_json::to_string_pretty(self).map_err(|_| ParseError::ConvertingTo {
            format: "json".to_string(),
        })
    }

    pub fn parse(las_file_path: &str) -> Result<Self, ParseError> {
        let reader = BufReader::new(File::open(las_file_path)?);
        let mut parser = LasParser::new(LasTokenizer::new(reader));
        LasFile::try_from(parser.parse()?)
    }
}

impl TryFrom<ParsedFile> for LasFile {
    type Error = ParseError;

    fn try_from(file: ParsedFile) -> Result<Self, Self::Error> {
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
