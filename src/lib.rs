#[cfg(test)]
mod tests;

mod errors;
mod parser;
mod sections;
mod tokenizer;

pub use errors::*;
pub use parser::*;
pub use sections::*;

pub(crate) use tokenizer::*;

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

    pub fn to_yaml_str(&mut self) -> Result<String, ParseError> {
        serde_yaml_ng::to_string(self).map_err(|_| ParseError::ConvertingTo {
            format: "yaml".to_string(),
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

        // The first data line in the Curve section must be one of "DEPT", "DEPTH", "TIME" or "INDEX".
        let first_curve = &las_file.curve_information.curves[0];
        let allowed_first_curves = [
            "DEPT".to_string(),
            "DEPTH".to_string(),
            "TIME".to_string(),
            "INDEX".to_string(),
        ];
        if !allowed_first_curves.contains(&first_curve.mnemonic.to_uppercase().to_string()) {
            return Err(ParseError::DisallowedFirstCurve {
                got: first_curve.mnemonic.clone(),
                expected_one_of: Vec::from(allowed_first_curves),
            });
        }

        // The channels (data-lines) in the curve section must be present in the data set (ascii_log_data).
        // There are some "official" las file examples that don't have the curve mnemonic match
        // exactly to the column header, so we play it loose and just ensure the lengths match.
        //
        // Example of las file where curve mnemonic doesn't match data header.. (from https://www.minnelusa.com/sampledata.php)
        // The curve mnemonic is "RESD", but the data header is "Resist." (which is the DEEP RESISTIVITY curve/data)
        if las_file.ascii_log_data.rows[0].len() != las_file.curve_information.curves.len() {
            return Err(ParseError::CurvesAndAsciiDataColumnsMismatch {
                num_curves: las_file.curve_information.curves.len(),
                num_data_cols: las_file.ascii_log_data.rows[0].len(),
                curves_line_number: las_file.curve_information.line_number,
                ascii_data_line_number: las_file.ascii_log_data.line_number,
            });
        }

        Ok(las_file)
    }
}
