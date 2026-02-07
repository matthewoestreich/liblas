use crate::{
    ParseError,
    parse::{LasFloat, ParsedLasFile, Section, SectionEntry, Sink},
};

pub(crate) struct Builder {
    file: ParsedLasFile,
    current_section: Option<Section>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            file: ParsedLasFile { sections: vec![] },
            current_section: None,
        }
    }

    pub fn finish(self) -> ParsedLasFile {
        self.file
    }
}

impl Sink for Builder {
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
            self.file.sections.push(sec);
        }
        Ok(())
    }
}
