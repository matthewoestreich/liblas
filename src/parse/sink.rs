use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry},
};

pub(crate) trait Sink {
    fn start_section(&mut self, section: Section) -> Result<(), ParseError>;
    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError>;
    fn ascii_row(&mut self, row: &[LasFloat]) -> Result<(), ParseError>;
    fn end_section(&mut self) -> Result<(), ParseError>;
}
