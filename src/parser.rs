use serde::{Deserialize, Serialize, Serializer};

use crate::{errors::ParseError, tokenizer::LasToken, write_comments};
use core::fmt;
use std::{
    collections::{HashMap, hash_map::Entry},
    iter::Peekable,
    str::FromStr,
};

const REQUIRED_SECTIONS: [SectionKind; 4] = [
    SectionKind::Version,
    SectionKind::Well,
    SectionKind::Curve,
    SectionKind::AsciiLogData,
];

#[derive(Debug)]
pub(crate) struct ParsedFile {
    pub sections: Vec<Section>,
}

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}

pub(crate) struct LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    tokens: Peekable<I>,
    current_section: Option<Section>,
    state: ParserState,
    parsed_sections: HashMap<SectionKind, usize>,
    comments: Option<Vec<String>>,
}

impl<I> LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            tokens: iter.peekable(),
            current_section: None,
            state: ParserState::Start,
            parsed_sections: HashMap::new(),
            comments: None,
        }
    }

    pub fn parse(&mut self) -> Result<ParsedFile, ParseError> {
        let mut file = ParsedFile { sections: vec![] };

        // A token is equivalent to a line within the original las file.
        while let Some(token) = self.next_token()? {
            match token {
                // We found a blank line
                LasToken::Blank { line_number } => {
                    // Blank lines not allowed in ASCII data section.
                    if let Some(section) = self.current_section.as_ref()
                        && section.header.kind == SectionKind::AsciiLogData
                    {
                        return Err(ParseError::AsciiDataContainsEmptyLine { line_number });
                    }
                }

                // We are parsing a data line within a section.
                LasToken::DataLine { raw, line_number } => {
                    if let Some(section) = self.current_section.as_mut() {
                        section.parse_line(&raw, line_number, self.comments.take())?;
                    }
                }

                // We have hit a new section.
                LasToken::SectionHeader { name, line_number } => {
                    let mut next_section = Section::new(name.to_string(), line_number);
                    let kind = next_section.header.kind;

                    next_section.comments = self.comments.take();

                    // Version information section must be first!
                    if self.state == ParserState::Start && kind != SectionKind::Version {
                        return Err(ParseError::VersionInformationNotFirst { line_number });
                    }
                    // ASCII log data section must be last
                    if self.state == ParserState::End && kind != SectionKind::AsciiLogData {
                        return Err(ParseError::AsciiLogDataSectionNotLast { line_number });
                    }
                    // If we have a parsed section already, add it to file.
                    if let Some(curr_sect) = self.current_section.take() {
                        file.sections.push(curr_sect);
                    }

                    self.state = match kind {
                        SectionKind::AsciiLogData => ParserState::End,
                        _ => ParserState::Working,
                    };

                    // Check for duplicate section.
                    match self.parsed_sections.entry(kind) {
                        Entry::Occupied(e) => {
                            return Err(ParseError::DuplicateSection {
                                section: kind,
                                line_number,
                                duplicate_line_number: *e.get(),
                            });
                        }
                        Entry::Vacant(e) => e.insert(line_number),
                    };

                    if kind == SectionKind::AsciiLogData {
                        self.set_ascii_headers_from_curve_section(&file, &mut next_section)?;
                    }

                    self.current_section = Some(next_section);
                }

                LasToken::Comment { text, .. } => {
                    self.comments.get_or_insert_with(Vec::new).push(text);
                }
            }
        }

        for required_section in REQUIRED_SECTIONS.iter() {
            if !self.parsed_sections.contains_key(required_section) {
                return Err(ParseError::MissingSection {
                    section: *required_section,
                });
            }
        }

        if let Some(section) = self.current_section.take() {
            file.sections.push(section);
        }

        Ok(file)
    }

    fn next_token(&mut self) -> Result<Option<LasToken>, ParseError> {
        match self.tokens.next() {
            Some(Ok(tok)) => Ok(Some(tok)),
            Some(Err(e)) => Err(ParseError::Io(e)),
            None => Ok(None),
        }
    }

    fn set_ascii_headers_from_curve_section(
        &mut self,
        file: &ParsedFile,
        section: &mut Section,
    ) -> Result<(), ParseError> {
        let curve_section = file
            .sections
            .iter()
            .find(|s| s.header.kind == SectionKind::Curve)
            .ok_or(ParseError::MissingCurveSectionOrAsciiLogsNotLastSectioon)?;

        let headers: Vec<String> = curve_section
            .entries
            .iter()
            .filter_map(|e| {
                if let SectionEntry::Delimited(d) = e {
                    Some(d.mnemonic.clone())
                } else {
                    None
                }
            })
            .collect();

        section.ascii_headers = Some(headers);
        Ok(())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Section {
    pub header: SectionHeader,
    pub line: usize,
    pub entries: Vec<SectionEntry>,
    pub ascii_headers: Option<Vec<String>>,
    pub ascii_rows: Vec<Vec<LasFloat>>,
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

    pub fn parse_line(
        &mut self,
        raw: &str,
        line_number: usize,
        comments: Option<Vec<String>>,
    ) -> Result<(), ParseError> {
        if self.header.kind == SectionKind::AsciiLogData {
            return self.parse_ascii_log_line(raw, line_number);
        }

        if self.header.kind == SectionKind::Other {
            self.entries.push(SectionEntry::Raw {
                text: raw.trim().to_string(),
                comments,
            });
            return Ok(());
        }

        // ############################
        // -- LAS DATA LINE LAYOUT --
        // ############################
        //
        // MNEM.UNITS    DATA   : DESCRIPTION
        //     |     |          |
        //     |     |          +-- last ':' on line
        //     |     +-- first space ' ' AFTER first '.'
        //     +-- first '.' in line
        //
        // -- MNEM -- (required)
        // "mnemonic. This mnemonic can be of any length but must not contain any internal
        // spaces, dots, or colons. Spaces are permitted in front of the mnemonic and between the
        // end of the mnemonic and the dot."
        //
        // -- UNITS -- (optional)
        // "units of the mnemonic (if applicable). The units, if used, must be located directly
        // after the dot. There must be no spaces between the units and the dot. The units can be of
        // any length but must not contain any colons or internal spaces."
        //
        // -- DATA (aka VALUE) -- (optional)
        // "value of, or data relating to the mnemonic. This value or input can be of any length
        // and can contain spaces, dots or colons as appropriate. It must be preceded by at least one
        // space to demarcate it from the units and must be to the left of the last colon in the line."
        //
        // -- DESCRIPTION -- (optional)
        // "description or definition of the mnemonic. It is always located to the right
        // of the last colon. The length of the line is no longer limited."

        let mut space: Option<usize> = None;
        let mut period: Option<usize> = None;
        let mut colon: Option<usize> = None;

        for (i, bytes) in raw.bytes().enumerate() {
            match bytes as char {
                // We only need to make note of the index for the first space in a line that comes AFTER the first period in a line.
                ' ' if space.is_none() && period.is_some() => {
                    space = Some(i);
                }
                // Only record index of first period in a line.
                '.' if period.is_none() => {
                    period = Some(i);
                }
                // We need to use the last colon on a line as a delimiter. Therefore, update the colon index each time we see one.
                ':' => {
                    colon = Some(i);
                }
                _ => {}
            };
        }

        let mut mnemonic: Option<String> = None;
        let mut unit: Option<String> = None;
        let mut value: Option<LasValue> = None;
        let mut description: Option<String> = None;

        if let Some(period_index) = period {
            let raw_mnemonic = raw[..period_index].trim().to_string();
            Self::validate_mnemonic(&raw_mnemonic, raw, line_number)?;
            mnemonic = Some(raw_mnemonic);

            if let Some(space_index) = space
                && let Some(colon_index) = colon
            {
                let raw_unit = raw[period_index..space_index].trim_start_matches('.').to_string();
                Self::validate_unit(&raw_unit, raw, line_number)?;
                unit = Some(raw_unit);

                if space_index > colon_index {
                    return Err(ParseError::MissingDelimiter {
                        delimiter: "Missing ' ' in line! Line must contain at least one space between first period on line and last colon on line!".to_string(),
                        line_number,
                        line: raw.to_string(),
                    });
                }

                value = LasValue::parse(&raw[space_index..colon_index]);
            }
        }

        if let Some(colon_index) = colon {
            // Everything from the last recorded colon index until end of line is description.
            description = Some(raw[colon_index + 1..raw.len()].trim().to_string());
        }

        self.entries.push(SectionEntry::Delimited(KeyValueData {
            comments,
            value,
            unit: unit.filter(|u| !u.is_empty()),
            description: description.filter(|d| !d.is_empty()),
            mnemonic: mnemonic.ok_or(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            })?,
        }));

        Ok(())
    }

    fn parse_ascii_log_line(&mut self, raw: &str, line_number: usize) -> Result<(), ParseError> {
        // If we are missing headers here it means we haven't parsed the Curve section yet.
        // Since ASCII section has to be the last section (per CWLS v2.0) it means we have
        // and invalid LAS file.
        let headers = self
            .ascii_headers
            .as_ref()
            .ok_or(ParseError::AsciiLogDataSectionNotLast { line_number })?;

        let values: Vec<LasFloat> = raw
            .split_whitespace()
            .map(|s| {
                s.parse::<LasFloat>().map_err(|_| ParseError::InvalidAsciiValue {
                    line_number,
                    raw_value: s.to_string(),
                })
            })
            .collect::<Result<_, _>>()?;

        if values.len() != headers.len() {
            return Err(ParseError::AsciiColumnsMismatch {
                line_number,
                num_cols_in_headers: headers.len(),
                num_cols_in_row: values.len(),
            });
        }

        self.ascii_rows.push(values);
        Ok(())
    }

    fn validate_mnemonic(raw_mnemonic: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
        if raw_mnemonic.is_empty() {
            return Err(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            });
        }
        let invalid_mnemonic_chars = str_contains(raw_mnemonic, &['.', ':', ' ']);
        if !invalid_mnemonic_chars.is_empty() {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "mnemonic".to_string(),
                line_number,
                invalid_chars: invalid_mnemonic_chars,
                line: raw.to_string(),
            });
        }
        Ok(())
    }

    fn validate_unit(raw_unit: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
        if raw_unit.starts_with(" ") {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "units".to_string(),
                line_number,
                invalid_chars: Vec::from([' ']),
                line: raw.to_string(),
            });
        }
        let invalid_unit_chars = str_contains(raw_unit, &[' ', ':']);
        if !invalid_unit_chars.is_empty() {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "units".to_string(),
                line_number,
                invalid_chars: invalid_unit_chars,
                line: raw.to_string(),
            });
        }
        Ok(())
    }
}

fn str_contains(str: &str, chars: &[char]) -> Vec<char> {
    let mut matches = vec![];
    for &c in chars {
        if str.contains(c) {
            matches.push(c);
        }
    }
    matches
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

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum SectionEntry {
    Delimited(KeyValueData),
    AsciiRow {
        data: Vec<f64>,
        comments: Option<Vec<String>>,
    },
    Raw {
        text: String,
        comments: Option<Vec<String>>,
    },
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
