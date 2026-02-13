#[cfg(test)]
mod tests;

mod errors;
mod las_file;

pub(crate) mod parse;
pub(crate) mod tokenizer;

pub mod sections;
pub use errors::*;
pub use las_file::*;
pub use parse::{DataLine, LasValue};

use crate::{parse::*, tokenizer::LasTokenizer};
use std::{
    fmt,
    fs::File,
    io::{BufReader, Read, Write},
};

/// Parse (stream) from a Read into a Write
/// We wrap your [`reader`] in [`BufReader`]
pub fn parse_from_into<R, W>(reader: R, writer: W, output_format: OutputFormat) -> Result<(), ParseError>
where
    R: Read,
    W: Write,
{
    let tokenizer = LasTokenizer::new(BufReader::new(reader));
    let mut parser = LasParser::new(tokenizer);

    match output_format {
        OutputFormat::JSON => {
            let mut sink = JsonSink::new(writer);
            parser.parse_into(&mut sink)?;
        }
        OutputFormat::YAML | OutputFormat::YML => {
            let mut sink = YamlSink::new(writer);
            parser.parse_into(&mut sink)?;
        }
    }

    Ok(())
}

/// Streams from source LAS file directly into writer.
pub fn parse_into<W>(las_file_path: &str, writer: W, output_format: OutputFormat) -> Result<(), ParseError>
where
    W: Write,
{
    let file = File::open(las_file_path)?;
    let reader = BufReader::new(file);
    let tokenizer = LasTokenizer::new(reader);
    let mut parser = LasParser::new(tokenizer);

    match output_format {
        OutputFormat::JSON => {
            let mut sink = JsonSink::new(writer);
            parser.parse_into(&mut sink)?;
        }
        OutputFormat::YAML | OutputFormat::YML => {
            let mut sink = YamlSink::new(writer);
            parser.parse_into(&mut sink)?;
        }
    }

    Ok(())
}

/// Parse .las file into LasFile
pub fn parse(las_file_path: &str) -> Result<LasFile, ParseError> {
    let file = File::open(las_file_path)?;
    let reader = BufReader::new(file);

    let tokenizer = LasTokenizer::new(reader);
    let mut parser = LasParser::new(tokenizer);
    let mut sink = AstSink::new();

    parser.parse_into(&mut sink)?;
    LasFile::try_from(sink)
}

#[derive(Debug, Clone, clap::ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    JSON,
    YAML,
    YML,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::JSON => write!(f, "JSON"),
            OutputFormat::YAML => write!(f, "YAML"),
            OutputFormat::YML => write!(f, "YML"),
        }
    }
}

pub(crate) fn any_present<T>(items: &[&Option<T>]) -> bool {
    items.iter().any(|o| o.is_some())
}

pub(crate) fn write_kv_opt(f: &mut fmt::Formatter<'_>, kv: &Option<DataLine>) -> fmt::Result {
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
