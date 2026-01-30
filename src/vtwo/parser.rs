use crate::{
    tokenizer::LasToken,
    vtwo::{
        errors::ParseError,
        types::{LasFile, Section},
    },
};
use std::iter::Peekable;

pub struct LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    tokens: Peekable<I>,
    current_section: Option<Section>,
}

impl<I> LasParser<I>
where
    I: Iterator<Item = Result<LasToken, std::io::Error>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            tokens: iter.peekable(),
            current_section: None,
        }
    }

    pub fn parse_n_lines(&mut self, n: usize) -> Result<LasFile, ParseError> {
        let mut file = LasFile { sections: vec![] };
        let mut i = n;

        while i > 0
            && let Ok(token) = self.next_token()
            && token.is_some()
        {
            match token.expect("already checked is_none") {
                LasToken::SectionHeader {
                    name,
                    line_number: line,
                } => {
                    if let Some(curr_section) = self.current_section.take() {
                        file.sections.push(curr_section);
                    }
                    self.current_section = Some(Section::new(name, line));
                }
                LasToken::DataLine { raw, line_number: line } => {
                    if let Some(section) = self.current_section.as_mut() {
                        section.parse_line(&raw, line)?;
                    }
                }
                _ => {}
            };

            i -= 1;
        }

        if let Some(section) = self.current_section.take() {
            file.sections.push(section);
        }

        Ok(file)
    }

    pub fn parse(&mut self) -> Result<LasFile, ParseError> {
        let mut file = LasFile { sections: vec![] };

        while let Some(token) = self.next_token()? {
            match token {
                LasToken::SectionHeader {
                    name,
                    line_number: line,
                } => {
                    if let Some(curr_section) = self.current_section.take() {
                        file.sections.push(curr_section);
                    }
                    self.current_section = Some(Section::new(name, line));
                }
                LasToken::DataLine { raw, line_number: line } => {
                    if let Some(section) = self.current_section.as_mut() {
                        section.parse_line(&raw, line)?;
                    }
                }
                _ => {}
            }
        }

        if let Some(curr_sect) = self.current_section.take() {
            file.sections.push(curr_sect);
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
}
