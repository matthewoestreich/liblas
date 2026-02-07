use crate::{
    Section, SectionEntry, SectionKind,
    errors::ParseError,
    parse::{DataLine, LasFloat, LasValue, ParserState, REQUIRED_SECTIONS, Sink, str_contains},
    tokenizer::LasToken,
};
use std::{
    collections::{HashMap, hash_map::Entry},
    iter::Peekable,
};

pub(crate) struct LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    tokens: Peekable<I>,
    current_section: Option<SectionKind>,
    state: ParserState,
    parsed_sections: HashMap<SectionKind, usize>,
    curve_mnemonics: Vec<String>,
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
            curve_mnemonics: vec![],
            comments: None,
        }
    }

    pub fn parse_into<S>(&mut self, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        while let Some(token) = self.next_token()? {
            match token {
                LasToken::SectionHeader { name, line_number } => {
                    let mut next_section = Section::new(name.to_string(), line_number);
                    next_section.comments = self.comments.take();

                    // Version information section must be first!
                    if self.state == ParserState::Start && next_section.header.kind != SectionKind::Version {
                        return Err(ParseError::VersionInformationNotFirst { line_number });
                    }
                    // ASCII log data section must be last
                    if self.state == ParserState::End && next_section.header.kind != SectionKind::AsciiLogData {
                        return Err(ParseError::AsciiLogDataSectionNotLast { line_number });
                    }

                    self.state = match next_section.header.kind {
                        SectionKind::AsciiLogData => ParserState::End,
                        _ => ParserState::Working,
                    };

                    // Check for duplicate section.
                    match self.parsed_sections.entry(next_section.header.kind) {
                        Entry::Occupied(e) => {
                            return Err(ParseError::DuplicateSection {
                                section: next_section.header.kind,
                                line_number,
                                duplicate_line_number: *e.get(),
                            });
                        }
                        Entry::Vacant(e) => e.insert(line_number),
                    };

                    if next_section.header.kind == SectionKind::AsciiLogData {
                        let headers = self.curve_mnemonics.clone();
                        next_section.ascii_headers = Some(headers);
                    }

                    sink.end_section()?;
                    self.current_section = Some(next_section.header.kind);
                    sink.start_section(next_section)?;
                }
                LasToken::DataLine { raw, line_number } => {
                    let entry = self.parse_data_line(&raw, line_number)?;
                    match &entry {
                        SectionEntry::AsciiLogData(row) => sink.ascii_row(row)?,
                        SectionEntry::Raw { .. } => sink.entry(entry)?,
                        SectionEntry::Delimited(data_line) => {
                            if self.current_section.is_some_and(|s| s == SectionKind::Curve) {
                                self.curve_mnemonics.push(data_line.mnemonic.clone());
                            }
                            sink.entry(entry)?;
                        }
                    }
                }
                LasToken::Comment { text, line_number } => {
                    // Comments not allowed in ASCII data section.
                    if self.current_section.is_some_and(|s| s == SectionKind::AsciiLogData) {
                        return Err(ParseError::AsciiDataContainsInvalidLine {
                            line_number,
                            line_kind: crate::InvalidLineKind::Comment,
                        });
                    }

                    self.comments.get_or_insert_with(Vec::new).push(text);
                }
                LasToken::Blank { line_number } => {
                    // Blank lines not allowed in ASCII data section.
                    if self.current_section.is_some_and(|s| s == SectionKind::AsciiLogData) {
                        return Err(ParseError::AsciiDataContainsInvalidLine {
                            line_number,
                            line_kind: crate::InvalidLineKind::Empty,
                        });
                    }
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

        self.validate_curves()?;

        if let Some(_curr_section) = self.current_section.take() {
            sink.end_section()?;
        }

        Ok(())
    }

    fn next_token(&mut self) -> Result<Option<LasToken>, ParseError> {
        match self.tokens.next() {
            Some(Ok(tok)) => Ok(Some(tok)),
            Some(Err(e)) => Err(ParseError::Io(e)),
            None => Ok(None),
        }
    }

    pub fn parse_data_line(&mut self, raw: &str, line_number: usize) -> Result<SectionEntry, ParseError> {
        if self.current_section.is_some_and(|s| s == SectionKind::AsciiLogData) {
            return self.parse_ascii_data_line(raw, line_number);
        }

        if self.current_section.is_some_and(|s| s == SectionKind::Other) {
            return Ok(SectionEntry::Raw {
                text: raw.trim().to_string(),
                comments: self.comments.take(),
            });
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

        Ok(SectionEntry::Delimited(DataLine {
            comments: self.comments.take(),
            value,
            unit: unit.filter(|u| !u.is_empty()),
            description: description.filter(|d| !d.is_empty()),
            mnemonic: mnemonic.ok_or(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            })?,
        }))
    }

    fn parse_ascii_data_line(&mut self, raw: &str, line_number: usize) -> Result<SectionEntry, ParseError> {
        // If we are missing headers here it means we haven't parsed the Curve section yet.
        // Since ASCII section has to be the last section (per CWLS v2.0) it means we have
        // and invalid LAS file.
        if self.curve_mnemonics.is_empty() {
            return Err(ParseError::AsciiLogDataSectionNotLast { line_number });
        }

        let values: Vec<LasFloat> = raw
            .split_whitespace()
            .map(|s| {
                s.parse::<LasFloat>().map_err(|_| ParseError::InvalidAsciiValue {
                    line_number,
                    raw_value: s.to_string(),
                })
            })
            .collect::<Result<_, _>>()?;

        if values.len() != self.curve_mnemonics.len() {
            return Err(ParseError::AsciiColumnsMismatch {
                line_number,
                num_cols_in_headers: self.curve_mnemonics.len(),
                num_cols_in_row: values.len(),
            });
        }

        Ok(SectionEntry::AsciiLogData(values))
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

    fn validate_curves(&self) -> Result<(), ParseError> {
        let allowed_first_curves = [
            "DEPT".to_string(),
            "DEPTH".to_string(),
            "TIME".to_string(),
            "INDEX".to_string(),
        ];

        if self.curve_mnemonics.is_empty() {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Curve,
                one_of: Vec::from(allowed_first_curves),
            });
        }

        // The first data line in the Curve section must be one of "DEPT", "DEPTH", "TIME" or "INDEX".
        //if let SectionEntry::Delimited(first_curve) = &curves.entries[0]
        //    && !allowed_first_curves.contains(&first_curve.mnemonic.to_uppercase().to_string())
        if !allowed_first_curves.contains(&self.curve_mnemonics[0]) {
            return Err(ParseError::DisallowedFirstCurve {
                got: self.curve_mnemonics[0].clone(),
                expected_one_of: Vec::from(allowed_first_curves),
            });
        }

        Ok(())
    }
}
