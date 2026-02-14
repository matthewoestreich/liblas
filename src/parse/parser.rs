use crate::{
    InvalidLineKind, ParseError, Section, SectionEntry, SectionKind,
    parse::{
        DataLine, LasValue, LineDelimiters, REQUIRED_SECTIONS, Sink, context::ParserContext, state::ParserState,
        str_contains,
    },
    tokenizer::LasToken,
};
use std::io;

pub(crate) struct LasParser<I>
where
    I: Iterator<Item = Result<LasToken, io::Error>>,
{
    tokens: I,
    ctx: ParserContext,
}

impl<I> LasParser<I>
where
    I: Iterator<Item = Result<LasToken, io::Error>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            tokens: iter,
            ctx: ParserContext::default(),
        }
    }

    pub fn parse_into<S>(&mut self, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        sink.start()?;
        while let Some(token) = self.next_token()? {
            self.handle_token(token, sink)?;
        }
        self.finish(sink)
    }

    fn handle_token<S>(&mut self, token: LasToken, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        match token {
            LasToken::SectionHeader { name, line_number } => self.handle_section_header(name, line_number, sink),
            LasToken::DataLine { raw, line_number } => self.handle_data_line(&raw, line_number, sink),
            LasToken::Comment { text, line_number } => self.handle_comment(text, line_number),
            LasToken::Blank { line_number } => self.handle_blank(line_number),
        }
    }

    fn handle_section_header<S>(&mut self, name: String, line_number: usize, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        let mut next_section = Section::new(name, line_number);
        next_section.comments = self.ctx.comments.take();

        self.enter_section(&next_section.header.kind, line_number)?;

        if next_section.header.kind == SectionKind::AsciiLogData {
            next_section.ascii_headers = Some(self.ctx.curve_mnemonics.clone());
        }

        sink.section_end()?;
        sink.section_start(next_section)?;
        Ok(())
    }

    fn enter_section(&mut self, section: &SectionKind, line_number: usize) -> Result<(), ParseError> {
        self.validate_transition(section, line_number)?;
        self.validate_duplicates(section, line_number)?;
        self.ctx.sections.insert(*section, line_number);
        self.ctx.state = ParserState::In(*section);
        Ok(())
    }

    fn validate_transition(&mut self, section: &SectionKind, line_number: usize) -> Result<(), ParseError> {
        match (&self.ctx.state, section) {
            // Version must be first
            (ParserState::Start, SectionKind::Version) => Ok(()),
            // From Start to anything other than Version is illegal.
            (ParserState::Start, _) => Err(ParseError::VersionInformationNotFirst { line_number }),
            // Going from In anything to Version section means Version wasn't first, or is duplicate..
            (ParserState::In(_), SectionKind::Version) => {
                if let Some(v) = self.ctx.sections.get(&SectionKind::Version) {
                    return Err(ParseError::DuplicateSection {
                        section: *section,
                        line_number,
                        duplicate_line_number: *v,
                    });
                }
                Err(ParseError::VersionInformationNotFirst { line_number })
            }
            // Transitioning from Ascii section to anything means Ascii wasn't last.
            (ParserState::In(SectionKind::AsciiLogData), _) => {
                Err(ParseError::AsciiLogDataSectionNotLast { line_number })
            }
            (ParserState::In(_), _) => Ok(()),
        }
    }

    fn next_token(&mut self) -> Result<Option<LasToken>, ParseError> {
        match self.tokens.next() {
            Some(Ok(tok)) => Ok(Some(tok)),
            Some(Err(e)) => Err(ParseError::Io(e)),
            None => Ok(None),
        }
    }

    fn handle_data_line<S>(&mut self, raw: &str, line_number: usize, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        let entry = match self.ctx.state {
            ParserState::In(SectionKind::Other) => SectionEntry::Raw {
                text: raw.trim().to_string(),
                comments: self.ctx.comments.take(),
            },
            ParserState::In(SectionKind::AsciiLogData) => self.parse_ascii_data_line(raw, line_number)?,
            _ => self.parse_data_line(raw, line_number)?,
        };

        match entry {
            SectionEntry::AsciiLogData(ref row) => sink.ascii_row(row)?,
            SectionEntry::Raw { .. } => sink.entry(entry)?,
            SectionEntry::Delimited(ref data_line) => {
                if self.ctx.state == ParserState::In(SectionKind::Curve) {
                    self.ctx.curve_mnemonics.push(data_line.mnemonic.clone());
                }
                sink.entry(entry)?;
            }
        }

        Ok(())
    }

    fn handle_comment(&mut self, text: String, line_number: usize) -> Result<(), ParseError> {
        if self.ctx.state == ParserState::In(SectionKind::AsciiLogData) {
            return Err(ParseError::AsciiDataContainsInvalidLine {
                line_number,
                line_kind: InvalidLineKind::Comment,
            });
        }
        self.ctx.comments.push(text);
        Ok(())
    }

    fn handle_blank(&mut self, line_number: usize) -> Result<(), ParseError> {
        if self.ctx.state == ParserState::In(SectionKind::AsciiLogData) {
            return Err(ParseError::AsciiDataContainsInvalidLine {
                line_number,
                line_kind: InvalidLineKind::Empty,
            });
        }
        Ok(())
    }

    fn validate_duplicates(&mut self, kind: &SectionKind, line_number: usize) -> Result<(), ParseError> {
        if let Some(&duplicate_line_number) = self.ctx.sections.get(kind) {
            return Err(ParseError::DuplicateSection {
                section: *kind,
                line_number,
                duplicate_line_number,
            });
        }
        Ok(())
    }

    fn check_for_required_sections(&self) -> Result<(), ParseError> {
        for required_section in REQUIRED_SECTIONS.iter() {
            if !self.ctx.sections.contains_key(required_section) {
                return Err(ParseError::MissingSection {
                    section: *required_section,
                });
            }
        }
        Ok(())
    }

    fn finish<S>(&mut self, sink: &mut S) -> Result<(), ParseError>
    where
        S: Sink,
    {
        self.check_for_required_sections()?;
        self.validate_curves()?;

        if let ParserState::In(_) = self.ctx.state {
            sink.section_end()?;
        }

        sink.end()?;
        Ok(())
    }

    fn parse_data_line(&mut self, raw: &str, line_number: usize) -> Result<SectionEntry, ParseError> {
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

        let ld = LineDelimiters::find_in(raw);

        let mut mnemonic: Option<String> = None;
        let mut unit: Option<String> = None;
        let mut value: Option<LasValue> = None;
        let mut description: Option<String> = None;

        if let Some(period_index) = ld.period {
            let raw_mnemonic = raw[..period_index].trim().to_string();
            Self::validate_mnemonic(&raw_mnemonic, raw, line_number)?;
            mnemonic = Some(raw_mnemonic);

            if let Some(space_index) = ld.space
                && let Some(colon_index) = ld.colon
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

        if let Some(colon_index) = ld.colon {
            // Everything from the last recorded colon index until end of line is description.
            description = Some(raw[colon_index + 1..raw.len()].trim().to_string());
        }

        Ok(SectionEntry::Delimited(DataLine {
            comments: self.ctx.comments.take(),
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
        if self.ctx.curve_mnemonics.is_empty() {
            return Err(ParseError::AsciiLogDataSectionNotLast { line_number });
        }

        let mut values = Vec::with_capacity(self.ctx.curve_mnemonics.len());
        for token in raw.split_ascii_whitespace() {
            values.push(token.to_string());
        }

        if values.len() != self.ctx.curve_mnemonics.len() {
            return Err(ParseError::AsciiColumnsMismatch {
                line_number,
                num_cols_in_headers: self.ctx.curve_mnemonics.len(),
                num_cols_in_row: values.len(),
            });
        }

        Ok(SectionEntry::AsciiLogData(values))
    }

    pub(crate) fn validate_mnemonic(raw_mnemonic: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
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

    pub(crate) fn validate_unit(raw_unit: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
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

        if self.ctx.curve_mnemonics.is_empty() {
            return Err(ParseError::SectionMissingRequiredData {
                section: SectionKind::Curve,
                one_of: Vec::from(allowed_first_curves),
            });
        }

        // The first data line in the Curve section must be one of "DEPT", "DEPTH", "TIME" or "INDEX".
        if !allowed_first_curves.contains(&self.ctx.curve_mnemonics[0]) {
            return Err(ParseError::DisallowedFirstCurve {
                got: self.ctx.curve_mnemonics[0].clone(),
                expected_one_of: Vec::from(allowed_first_curves),
            });
        }

        Ok(())
    }
}
