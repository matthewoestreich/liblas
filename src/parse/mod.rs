mod ast_sink;
mod json_sink;
mod parser;
mod yaml_sink;

pub(crate) use ast_sink::*;
pub(crate) use json_sink::*;
pub(crate) use parser::*;
pub(crate) use yaml_sink::*;

use crate::{ParseError, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

// The max number of sections a .las file can have.
const MAX_NUM_SECTIONS: usize = 6;

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

// ================================================================================================
// ------------------------ LasValue --------------------------------------------------------------
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum LasValue {
    Int(i64),
    Text(String),
}

impl LasValue {
    pub fn new(value: &str) -> Option<Self> {
        Self::parse(value)
    }
}

impl fmt::Display for LasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LasValue::Int(i) => write!(f, "{i}"),
            LasValue::Text(t) => write!(f, "{t}"),
        }
    }
}

impl LasValue {
    pub fn parse(raw: &str) -> Option<LasValue> {
        let raw = raw.trim();
        if raw.is_empty() {
            None
        } else if let Ok(i) = raw.parse::<i64>() {
            Some(LasValue::Int(i))
        } else {
            Some(LasValue::Text(raw.to_string()))
        }
    }
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
// ------------------------ Sink (trait) ----------------------------------------------------------
// ================================================================================================

pub(crate) trait Sink {
    // Fires when we encounter a new section.
    fn section_start(&mut self, section: Section) -> Result<(), ParseError>;
    // Fires when we encounter a section entry.
    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError>;
    // Fires when we encounter an ascii data row.
    fn ascii_row(&mut self, row: &[String]) -> Result<(), ParseError>;
    // Fires when we are done parsing a section.
    fn section_end(&mut self) -> Result<(), ParseError>;
    // Fires when the parser starts parsing.
    fn start(&mut self) -> Result<(), ParseError> {
        Ok(())
    }
    // Fires when the parser is done parsing.
    fn end(&mut self) -> Result<(), ParseError> {
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
// ------------------------ ParserState -----------------------------------------------------------
// ================================================================================================

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}
