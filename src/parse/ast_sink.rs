use crate::{
    ParseError,
    parse::{Section, SectionEntry, Sink},
};

/// Sink for parsing into "abstract syntax tree"
pub(crate) struct AstSink {
    pub sections: Vec<Section>,
    current_section: Option<Section>,
}

impl AstSink {
    pub fn new() -> Self {
        Self {
            sections: vec![],
            current_section: None,
        }
    }
}

impl Sink for AstSink {
    fn section_start(&mut self, section: Section) -> Result<(), ParseError> {
        self.current_section = Some(section);
        Ok(())
    }

    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.as_mut() {
            sec.entries.push(entry);
        }
        Ok(())
    }

    fn ascii_row(&mut self, row: &[String]) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.as_mut() {
            sec.ascii_rows.push(row.to_vec());
        }
        Ok(())
    }

    fn section_end(&mut self) -> Result<(), ParseError> {
        if let Some(sec) = self.current_section.take() {
            self.sections.push(sec);
        }
        Ok(())
    }
}
