mod json_sink;
mod parsed_las_file;
mod parser;
mod sink;
mod value;
mod yaml_sink;

pub use value::*;

pub(crate) use json_sink::*;
pub(crate) use parsed_las_file::*;
pub(crate) use parser::*;
pub(crate) use sink::*;
pub(crate) use yaml_sink::*;

use crate::write_comments;
use serde::{Deserialize, Serialize};
use std::fmt;

const REQUIRED_SECTIONS: [SectionKind; 4] = [
    SectionKind::Version,
    SectionKind::Well,
    SectionKind::Curve,
    SectionKind::AsciiLogData,
];

fn str_contains(str: &str, chars: &[char]) -> Vec<char> {
    let mut matches = vec![];
    for &c in chars {
        if str.contains(c) {
            matches.push(c);
        }
    }
    matches
}

// ================================================================================================
// ------------------------ ParserState -----------------------------------------------------------
// ================================================================================================

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}

// ================================================================================================
// ------------------------ DataLine --------------------------------------------------------------
// ================================================================================================

// The sections "VERSION", "WELL", "CURVE" and "PARAMETER" use line delimiters.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataLine {
    pub mnemonic: String,
    pub unit: Option<String>,
    pub value: Option<LasValue>,
    pub description: Option<String>,
    pub comments: Option<Vec<String>>,
}

impl fmt::Display for DataLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_comments(f, &self.comments)?;
        write!(f, "{}.", self.mnemonic)?;
        if let Some(unit) = self.unit.as_ref() {
            write!(f, "{unit}")?;
        }
        write!(f, " ")?;
        if let Some(value) = self.value.as_ref() {
            write!(f, "{value} ")?;
        }
        write!(f, ":")?;
        if let Some(description) = self.description.as_ref() {
            write!(f, " {description}")?;
        }
        Ok(())
    }
}

// ================================================================================================
// ------------------------ Section ---------------------------------------------------------------
// ================================================================================================

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct Section {
    pub header: SectionHeader,
    pub line: usize,
    pub entries: Vec<SectionEntry>,
    pub ascii_headers: Option<Vec<String>>,
    pub ascii_rows: Vec<Vec<String>>,
    pub comments: Option<Vec<String>>,
}

impl Section {
    pub fn new(name: String, line: usize) -> Self {
        Self {
            header: SectionHeader {
                kind: SectionKind::from(name.as_str()),
                raw: name,
            },
            line,
            entries: vec![],
            ascii_headers: None,
            ascii_rows: vec![],
            comments: None,
        }
    }
}

// ================================================================================================
// ------------------------ SectionEntry ----------------------------------------------------------
// ================================================================================================

#[derive(Debug, Serialize)]
pub(crate) enum SectionEntry {
    Delimited(DataLine),
    AsciiLogData(Vec<String>),
    Raw {
        text: String,
        comments: Option<Vec<String>>,
    },
}

// ================================================================================================
// ------------------------ SectionHeader ---------------------------------------------------------
// ================================================================================================

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct SectionHeader {
    pub raw: String,
    pub kind: SectionKind,
}

#[allow(dead_code)]
impl SectionHeader {
    pub fn new(name: String, kind: SectionKind) -> Self {
        Self { raw: name, kind }
    }
}

// ================================================================================================
// ------------------------ SectionKind -----------------------------------------------------------
// ================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum SectionKind {
    Version,
    Well,
    Curve,
    Parameter,
    Other,
    AsciiLogData,
}

impl From<&str> for SectionKind {
    fn from(value: &str) -> Self {
        match value {
            v if v.starts_with("V") => SectionKind::Version,
            v if v.starts_with("W") => SectionKind::Well,
            v if v.starts_with("C") => SectionKind::Curve,
            v if v.starts_with("P") => SectionKind::Parameter,
            v if v.starts_with("O") => SectionKind::Other,
            v if v.starts_with("A") => SectionKind::AsciiLogData,
            _ => unreachable!("unrecognized section! {value}"),
        }
    }
}
