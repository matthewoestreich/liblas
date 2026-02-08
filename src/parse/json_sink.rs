use std::io::Write;

use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry, SectionKind, Sink},
    sections::{CurveInformation, OtherInformation, ParameterInformation, VersionInformation, WellInformation},
};

// We store every section outside of AsciiLogData within the 'current_section'.
// Those sections are very small in comparison to ascii data. We directly stream
// and write the ascii data to the writer, no allocations or buffering.
pub struct JsonSink<W: Write> {
    writer: W,
    current_section: Option<Section>,
    is_first_ascii_row: bool,
}

impl<W: Write> JsonSink<W> {
    pub fn new(writer: W) -> Self {
        let mut this = Self {
            writer,
            current_section: None,
            is_first_ascii_row: true,
        };
        write!(&mut this.writer, "{{").expect("no error");
        this
    }
}

impl<W: Write> Sink for JsonSink<W> {
    fn start_section(&mut self, section: Section) -> Result<(), ParseError> {
        if section.header.kind == SectionKind::AsciiLogData {
            writeln!(self.writer, "  \"AsciiLogData\": {{ ")?;
            writeln!(self.writer, "    \"headers\": ")?;
            serde_json::to_writer_pretty(&mut self.writer, &section.ascii_headers)
                .map_err(|e| ParseError::Error { message: e.to_string() })?;
            writeln!(self.writer, ",")?;
            writeln!(self.writer, "\"rows\": [")?;
        }
        self.current_section = Some(section);
        Ok(())
    }

    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError> {
        if let Some(curr_sect) = self.current_section.as_mut() {
            curr_sect.entries.push(entry);
        }
        Ok(())
    }

    fn ascii_row(&mut self, row: &[LasFloat]) -> Result<(), ParseError> {
        if !self.is_first_ascii_row {
            write!(self.writer, ",")?;
        }
        self.is_first_ascii_row = false;
        serde_json::to_writer_pretty(&mut self.writer, row)
            .map_err(|e| ParseError::Error { message: e.to_string() })?;
        Ok(())
    }

    fn end_section(&mut self) -> Result<(), ParseError> {
        if let Some(section) = self.current_section.take() {
            let kind = section.header.kind;

            match kind {
                SectionKind::Version => {
                    write!(self.writer, "  \"VersionInformation\": ")?;
                    serde_json::to_writer_pretty(&mut self.writer, &VersionInformation::try_from(section)?)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                }
                SectionKind::Well => {
                    write!(self.writer, "  \"WellInformation\": ")?;
                    serde_json::to_writer_pretty(&mut self.writer, &WellInformation::try_from(section)?)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                }
                SectionKind::Curve => {
                    write!(self.writer, "  \"CurveInformation\": ")?;
                    serde_json::to_writer_pretty(&mut self.writer, &CurveInformation::try_from(section)?)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                }
                SectionKind::Parameter => {
                    write!(self.writer, "  \"ParameterInformation\": ")?;
                    serde_json::to_writer_pretty(&mut self.writer, &ParameterInformation::try_from(section)?)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                }
                SectionKind::Other => {
                    write!(self.writer, "  \"OtherInformation\": ")?;
                    serde_json::to_writer_pretty(&mut self.writer, &OtherInformation::try_from(section)?)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                }
                SectionKind::AsciiLogData => {
                    writeln!(self.writer, "]")?;
                    writeln!(self.writer, "}}")?;
                }
            };

            // AsciiLogData is expected to be the last section in a .las file.
            // That is how we can get away with making these assumptions.
            if kind != SectionKind::AsciiLogData {
                writeln!(self.writer, ",")?;
            } else {
                writeln!(self.writer, "}}")?;
            }
        }

        Ok(())
    }
}
