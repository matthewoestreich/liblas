use crate::{
    ParseError,
    parse::{AstSink, SectionKind},
    sections::*,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
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
    pub fn new(
        version_info: VersionInformation,
        well_info: WellInformation,
        curve_info: CurveInformation,
        ascii_log_data: AsciiLogData,
        other_info: Option<OtherInformation>,
        param_info: Option<ParameterInformation>,
    ) -> Self {
        Self {
            version_information: version_info,
            well_information: well_info,
            curve_information: curve_info,
            ascii_log_data,
            other_information: other_info,
            parameter_information: param_info,
        }
    }

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
}

impl TryFrom<AstSink> for LasFile {
    type Error = ParseError;

    fn try_from(ast_sink: AstSink) -> Result<Self, Self::Error> {
        let mut las_file = LasFile::default();

        for section in ast_sink.sections {
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
