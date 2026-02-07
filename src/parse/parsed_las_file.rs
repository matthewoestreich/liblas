use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry, Sink},
};

pub(crate) struct ParsedLasFile {
    pub sections: Vec<Section>,
    current_section: Option<Section>,
}

impl ParsedLasFile {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            current_section: None,
        }
    }
}

impl Sink for ParsedLasFile {
    fn start_section(&mut self, section: Section) -> Result<(), ParseError> {
        self.current_section = Some(section);
        Ok(())
    }

    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.as_mut() {
            sec.entries.push(entry);
        }
        Ok(())
    }

    fn ascii_row(&mut self, row: &[LasFloat]) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.as_mut() {
            sec.ascii_rows.push(row.to_vec());
        }
        Ok(())
    }

    fn end_section(&mut self) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.take() {
            self.sections.push(sec);
        }
        Ok(())
    }
}
