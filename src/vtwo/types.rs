use crate::vtwo::errors::ParseError;

#[derive(Debug)]
pub struct LasFile {
    pub sections: Vec<Section>,
}

#[derive(Debug)]
pub struct Section {
    pub header: SectionHeader,
    pub line: usize,
    pub entries: Vec<SectionEntry>,
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
        }
    }

    pub fn parse_line(&mut self, raw: &str, line_number: usize) -> Result<(), ParseError> {
        if self.header.kind == SectionKind::AsciiLogData {
            // Skip for now
            return Ok(());
        }

        if self.header.kind == SectionKind::Other {
            self.entries.push(SectionEntry::Raw(raw.trim().to_string()));
            return Ok(());
        }

        // Split at the *last* colon to isolate description
        let (before_colon, description) = raw.rsplit_once(':').ok_or_else(|| ParseError::MissingDelimiter {
            delimiter: "last colon (':') on line".to_string(),
            line_number,
            line: raw.to_string(),
        })?;

        let description = Some(description.trim().to_string());

        // Find the position of the '.' in the left-hand part
        let dot_index = before_colon.find('.').ok_or_else(|| ParseError::MissingDelimiter {
            delimiter: "first dot ('.') on line".to_string(),
            line_number,
            line: raw.to_string(),
        })?;

        // Everything before '.' is mnemonic (trim it)
        let mnemonic = before_colon[..dot_index].trim().to_string();
        if mnemonic.is_empty() {
            return Err(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            });
        }

        // After the '.' is unit (no spaces allowed until value starts)
        let after_dot = &before_colon[dot_index + 1..];

        let (unit, data) = if after_dot.is_empty() {
            (None, "") // No unit, no value
        } else if after_dot.starts_with(char::is_whitespace) {
            // Space immediately after the dot → no unit
            (None, after_dot.trim())
        } else {
            // Possibly unit followed by value
            match after_dot.split_once(char::is_whitespace) {
                Some((u, rest)) => (Some(u.trim().to_string()), rest.trim()),
                None => (Some(after_dot.trim().to_string()), ""),
            }
        };

        let value = if data.is_empty() {
            LasValue::Text("".to_string())
        } else if let Ok(i) = data.parse::<i64>() {
            LasValue::Int(i)
        } else if data.contains('.')
            && let Ok(f) = data.parse::<f64>()
        {
            LasValue::Float(f)
        } else {
            LasValue::Text(data.to_string())
        };

        let entry = match self.header.kind {
            SectionKind::Curve => {
                let mut api_codes = vec![];
                if let LasValue::Text(api_code_raw) = value {
                    for part in api_code_raw.split_whitespace() {
                        api_codes.push(part.to_string());
                    }
                }

                SectionEntry::Curve(CurveEntry {
                    mnemonic,
                    unit: unit.unwrap_or_default(),
                    api_codes,
                    description,
                })
            }

            _ => SectionEntry::Delimited(DelimitedEntry {
                mnemonic,
                unit,
                value,
                description,
            }),
        };

        self.entries.push(entry);
        Ok(())
    }
}

#[derive(Debug)]
pub struct SectionHeader {
    pub raw: String, // eg. "Curve Information Section"
    pub kind: SectionKind,
}

impl SectionHeader {
    pub fn new(name: String, kind: SectionKind) -> Self {
        Self { raw: name, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        match value.to_lowercase() {
            v if v.starts_with("version") || v.starts_with("v") => SectionKind::Version,
            v if v.starts_with("well") || v.starts_with("w") => SectionKind::Well,
            v if v.starts_with("curve") || v.starts_with("c") => SectionKind::Curve,
            v if v.starts_with("parameter") || v.starts_with("p") => SectionKind::Parameter,
            v if v.starts_with("other") || v.starts_with("o") => SectionKind::Other,
            v if v.starts_with("ascii") || v.starts_with("a") => SectionKind::AsciiLogData,
            _ => unreachable!("unrecognized section! {value}"),
        }
    }
}

#[derive(Debug)]
pub enum SectionEntry {
    Delimited(DelimitedEntry),
    Curve(CurveEntry),
    AsciiRow(Vec<f64>),
    Raw(String),
}

// The sections "VERSION", "WELL", "CURVE" and "PARAMETER" use line delimiters.
#[derive(Debug)]
pub struct DelimitedEntry {
    pub mnemonic: String,
    pub unit: Option<String>,
    pub value: LasValue,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct CurveEntry {
    pub mnemonic: String,
    pub unit: String,
    pub api_codes: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LasValue {
    Int(i64),
    Float(f64),
    Text(String),
}

impl LasValue {
    pub fn parse(raw: &str) -> LasValue {
        let raw = raw.trim();

        if let Ok(i) = raw.parse::<i64>() {
            LasValue::Int(i)
        } else if let Ok(f) = raw.parse::<f64>() {
            LasValue::Float(f)
        } else {
            LasValue::Text(raw.to_string())
        }
    }
}
