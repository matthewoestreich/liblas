use crate::{
    Section, SectionEntry, SectionKind,
    errors::ParseError,
    parse::{ParsedLasFile, ParserState, REQUIRED_SECTIONS},
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

    pub fn parse(&mut self) -> Result<ParsedLasFile, ParseError> {
        let mut file = ParsedLasFile { sections: vec![] };

        // A token is equivalent to a line within the original las file.
        while let Some(token) = self.next_token()? {
            match token {
                LasToken::DataLine { raw, line_number } => {
                    if let Some(section) = self.current_section.as_mut() {
                        section.parse_line(&raw, line_number, self.comments.take())?;
                    }
                }
                LasToken::SectionHeader { name, line_number } => {
                    self.start_section(&mut file, &name, line_number)?;
                }
                LasToken::Comment { text, .. } => {
                    self.comments.get_or_insert_with(Vec::new).push(text);
                }
                // We found a blank line
                LasToken::Blank { line_number } => {
                    // Blank lines not allowed in ASCII data section.
                    if let Some(section) = self.current_section.as_ref()
                        && section.header.kind == SectionKind::AsciiLogData
                    {
                        return Err(ParseError::AsciiDataContainsEmptyLine { line_number });
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

        if let Some(section) = self.current_section.take() {
            file.sections.push(section);
        }

        self.validate_curves(&file)?;

        Ok(file)
    }

    fn next_token(&mut self) -> Result<Option<LasToken>, ParseError> {
        match self.tokens.next() {
            Some(Ok(tok)) => Ok(Some(tok)),
            Some(Err(e)) => Err(ParseError::Io(e)),
            None => Ok(None),
        }
    }

    fn start_section(&mut self, file: &mut ParsedLasFile, name: &str, line_number: usize) -> Result<(), ParseError> {
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
            self.set_ascii_headers_from_curve_section(file, &mut next_section)?;
        }

        self.current_section = Some(next_section);
        Ok(())
    }

    fn set_ascii_headers_from_curve_section(
        &mut self,
        file: &ParsedLasFile,
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

    fn validate_curves(&self, file: &ParsedLasFile) -> Result<(), ParseError> {
        use SectionKind as SK;
        // Validate curves/ascii logs
        let curves = file.sections.iter().find(|s| s.header.kind == SK::Curve);
        // We know these should exist here (from code outside of this func) but it is better to play it safe.
        if curves.is_none() {
            return Err(ParseError::MissingSection { section: SK::Curve });
        }

        let ascii_logs = file.sections.iter().find(|s| s.header.kind == SK::AsciiLogData);
        // We know these should exist here (from code outside of this func) but it is better to play it safe.
        if ascii_logs.is_none() {
            return Err(ParseError::MissingSection {
                section: SK::AsciiLogData,
            });
        }

        // We know these exist here, safe to call expect
        let curves = curves.expect("some");
        let ascii_logs = ascii_logs.expect("some");

        // The channels (data-lines) in the curve section must be present in the data set (ascii_log_data).
        // There are some "official" las file examples that don't have the curve mnemonic match
        // exactly to the column header, so we play it loose and just ensure the lengths match.
        //
        // Example of las file where curve mnemonic doesn't match data header.. (from https://www.minnelusa.com/sampledata.php)
        // The curve mnemonic is "RESD", but the data header is "Resist." (which is the DEEP RESISTIVITY curve/data)
        if ascii_logs.ascii_rows[0].len() != curves.entries.len() {
            return Err(ParseError::CurvesAndAsciiDataColumnsMismatch {
                num_curves: curves.entries.len(),
                num_data_cols: ascii_logs.ascii_rows[0].len(),
                curves_line_number: curves.line,
                ascii_data_line_number: ascii_logs.line,
            });
        }

        let allowed_first_curves = [
            "DEPT".to_string(),
            "DEPTH".to_string(),
            "TIME".to_string(),
            "INDEX".to_string(),
        ];

        if curves.entries.is_empty() {
            return Err(ParseError::SectionMissingRequiredData {
                section: SK::Curve,
                one_of: Vec::from(allowed_first_curves),
            });
        }

        // The first data line in the Curve section must be one of "DEPT", "DEPTH", "TIME" or "INDEX".
        if let SectionEntry::Delimited(first_curve) = &curves.entries[0]
            && !allowed_first_curves.contains(&first_curve.mnemonic.to_uppercase().to_string())
        {
            return Err(ParseError::DisallowedFirstCurve {
                got: first_curve.mnemonic.clone(),
                expected_one_of: Vec::from(allowed_first_curves),
            });
        }

        Ok(())
    }
}
