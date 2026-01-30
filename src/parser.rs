use crate::{
    LasFile,
    errors::ParseError,
    section::{Section, SectionKind},
    tokenizer::LasToken,
};
use std::{
    collections::{HashMap, hash_map::Entry},
    iter::Peekable,
};

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    Finished,
}

pub struct LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    tokens: Peekable<I>,
    current_section: Option<Section>,
    state: ParserState,
    parsed_sections: HashMap<SectionKind, usize>,
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
        }
    }

    pub fn parse(&mut self) -> Result<LasFile, ParseError> {
        let mut file = LasFile { sections: vec![] };

        // A token is equivalent to a line within the original las file.
        while let Some(token) = self.next_token()? {
            match token {
                LasToken::SectionHeader { name, line_number } => {
                    self.validate_and_set_current_section(&mut file, name.as_str(), line_number)?;
                }
                LasToken::DataLine { raw, line_number } => {
                    if let Some(section) = self.current_section.as_mut() {
                        section.parse_line(&raw, line_number)?;
                    }
                }
                // TODO : parse comments
                _ => {}
            }
        }

        // If we made it out of the while loop while still in Start state,
        // it means we never saw the Version information section, which is required.
        if self.state == ParserState::Start {
            return Err(ParseError::MissingSection {
                section: SectionKind::Version,
            });
        }
        // If we are not in Finished state, it means we never saw the ASCII Log Data
        // section, which is required.
        if self.state != ParserState::Finished {
            return Err(ParseError::MissingSection {
                section: SectionKind::AsciiLogData,
            });
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

    fn validate_and_set_current_section(
        &mut self,
        file: &mut LasFile,
        name: &str,
        line: usize,
    ) -> Result<(), ParseError> {
        let kind = SectionKind::from(name);

        // Version information section must be first!
        if self.state == ParserState::Start && kind != SectionKind::Version {
            return Err(ParseError::VersionInformationNotFirst { line_number: line });
        }

        // ASCII log data section must be last
        if self.state == ParserState::Finished && kind != SectionKind::AsciiLogData {
            return Err(ParseError::AsciiLogDataSectionNotLast { line_number: line });
        }

        // If we have a parsed section already, add it to file.
        if let Some(curr_sect) = self.current_section.take() {
            file.sections.push(curr_sect);
        }

        self.state = match kind {
            SectionKind::AsciiLogData => ParserState::Finished,
            _ => ParserState::Working,
        };

        let next_section = Section::new(name.to_string(), line);
        self.check_for_duplicate_section(next_section.header.kind, line)?;
        self.current_section = Some(next_section);

        Ok(())
    }

    fn check_for_duplicate_section(&mut self, section: SectionKind, line_number: usize) -> Result<(), ParseError> {
        match self.parsed_sections.entry(section) {
            Entry::Occupied(e) => Err(ParseError::DuplicateSection {
                section,
                line_number,
                duplicate_line_number: *e.get(),
            }),
            Entry::Vacant(vacant_entry) => {
                _ = vacant_entry.insert(line_number);
                Ok(())
            }
        }
    }
}
