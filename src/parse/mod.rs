mod parser;
mod section;

pub(crate) use parser::*;
pub(crate) use section::*;

use crate::{ParseError, write_comments};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt, str::FromStr};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// The sections "VERSION", "WELL", "CURVE" and "PARAMETER" use line delimiters.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct KeyValueData {
    pub mnemonic: String,
    pub unit: Option<String>,
    pub value: Option<LasValue>,
    pub description: Option<String>,
    pub comments: Option<Vec<String>>,
}

impl fmt::Display for KeyValueData {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LasValue {
    Int(i64),
    Float(LasFloat),
    Text(String),
}

impl fmt::Display for LasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LasValue::Int(i) => write!(f, "{i}"),
            LasValue::Float(lf) => write!(f, "{}", lf.raw),
            LasValue::Text(t) => write!(f, "{t}"),
        }
    }
}

impl LasValue {
    pub fn parse(raw: &str) -> Option<LasValue> {
        let raw = raw.trim();
        if let Ok(i) = raw.parse::<i64>() {
            Some(LasValue::Int(i))
        } else if raw.contains('.')
            && let Ok(f) = raw.parse::<f64>()
        {
            Some(LasValue::Float(LasFloat {
                raw: raw.to_string(),
                value: f,
            }))
        } else if raw.is_empty() {
            None
        } else {
            Some(LasValue::Text(raw.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub struct LasFloat {
    pub value: f64,
    pub raw: String,
}

impl FromStr for LasFloat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f64>().map_err(|_| ParseError::InvalidAsciiFloatValue {
            raw_value: s.to_string(),
        })?;
        Ok(LasFloat {
            raw: s.to_string(),
            value,
        })
    }
}

impl fmt::Display for LasFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Serialize for LasFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.value)
    }
}

impl<'de> Deserialize<'de> for LasFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = f64::deserialize(deserializer)?;
        Ok(Self {
            raw: v.to_string(),
            value: v,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ParsedLasFile {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub(crate) enum SectionEntry {
    Delimited(KeyValueData),
    Raw {
        text: String,
        comments: Option<Vec<String>>,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct SectionHeader {
    pub raw: String, // eg. "Curve Information Section"
    pub kind: SectionKind,
}

#[allow(dead_code)]
impl SectionHeader {
    pub fn new(name: String, kind: SectionKind) -> Self {
        Self { raw: name, kind }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}
