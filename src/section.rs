use crate::errors::ParseError;

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

        // After the '.' is unit (no spaces allowed until value starts) up until first space.
        // From first space until last colon is data (aka value).
        // This string will contain both the unit and data.
        let unit_and_data = &before_colon[dot_index + 1..];

        let (unit, data) = if unit_and_data.is_empty() {
            (None, "") // No unit and no data (aka value) 
        } else if unit_and_data.starts_with(char::is_whitespace) {
            (None, unit_and_data.trim()) // Space immediately after the dot -> no unit
        } else {
            // Possibly unit followed by value
            match unit_and_data.split_once(char::is_whitespace) {
                // Both unit and data.
                Some((u, rest)) => (Some(u.trim().to_string()), rest.trim()),
                // No data but unit.
                None => (Some(unit_and_data.trim().to_string()), ""),
            }
        };

        let entry = SectionEntry::Delimited(DelimitedEntry {
            mnemonic,
            unit,
            description,
            value: LasValue::from(data),
        });

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
        } else if raw.contains('.')
            && let Ok(f) = raw.parse::<f64>()
        {
            LasValue::Float(f)
        } else {
            LasValue::Text(raw.to_string())
        }
    }
}

impl From<&str> for LasValue {
    fn from(value: &str) -> Self {
        LasValue::parse(value)
    }
}
