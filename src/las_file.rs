use crate::{
    ParseError,
    parse::{JsonSink, LasParser, ParsedLasFile, SectionKind, YamlSink},
    sections::*,
    tokenizer::LasTokenizer,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

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

    /// Streams from parser into out file.
    pub fn parse_into_json_file(las_file_path: &str, json_file_path: &str) -> Result<(), ParseError> {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let out_file = File::create(json_file_path)?;
        let writer = BufWriter::new(out_file);
        let mut sink = JsonSink::new(writer);

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;
        Ok(())
    }

    pub fn parse_into_json_writer<W>(las_file_path: &str, writer: W) -> Result<(), ParseError>
    where
        W: Write,
    {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let mut sink = JsonSink::new(writer);

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;
        Ok(())
    }

    pub fn parse_into_yaml_writer<W>(las_file_path: &str, writer: W) -> Result<(), ParseError>
    where
        W: Write,
    {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let mut sink = YamlSink::new(writer);

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;
        Ok(())
    }

    pub fn parse_into_yaml_file(las_file_path: &str, yaml_file_path: &str) -> Result<(), ParseError> {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let out_file = File::create(yaml_file_path)?;
        let writer = BufWriter::new(out_file);
        let mut sink = YamlSink::new(writer);

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn parse_to_stdout(las_file_path: &str) -> Result<(), ParseError> {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let stdout = std::io::stdout();
        let handle = stdout.lock();
        let mut sink = JsonSink::new(handle);

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;
        Ok(())
    }

    pub fn parse(las_file_path: &str) -> Result<Self, ParseError> {
        let file = File::open(las_file_path)?;
        let reader = BufReader::new(file);

        let mut sink = ParsedLasFile::new();

        let tokenizer = LasTokenizer::new(reader);
        let mut parser = LasParser::new(tokenizer);
        parser.parse_into(&mut sink)?;

        LasFile::try_from(sink)
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
