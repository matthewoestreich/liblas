use crate::{
    Section, SectionEntry, SectionKind,
    errors::ParseError,
    parse::{ParsedFile, ParserState, REQUIRED_SECTIONS},
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
