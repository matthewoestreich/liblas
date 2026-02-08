use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry},
};

pub(crate) trait Sink {
    // Fires when we encounter a new section.
    fn section_start(&mut self, section: Section) -> Result<(), ParseError>;
    // Fires when we encounter a section entry.
    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError>;
    // Fires when we encounter an ascii data row.
    fn ascii_row(&mut self, row: &[LasFloat]) -> Result<(), ParseError>;
    // Fires when we are done parsing a section.
    fn section_end(&mut self) -> Result<(), ParseError>;
    // Fires when the parser starts parsing.
    fn start(&mut self) -> Result<(), ParseError> {
        Ok(())
    }
    // Fires when the parser is done parsing.
    fn end(&mut self) -> Result<(), ParseError> {
        Ok(())
    }
}
