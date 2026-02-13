use crate::{
    ParseError,
    parse::{Section, SectionEntry, SectionKind, Sink},
    sections::{CurveInformation, OtherInformation, ParameterInformation, VersionInformation, WellInformation},
};
use serde::Serialize;
use std::io::Write;

// We store every section outside of AsciiLogData within the 'current_section'.
// Those sections are very small in comparison to ascii data. We directly stream
// and write the ascii data to the writer, no allocations or buffering.
pub(crate) struct JsonSink<W>
where
    W: Write,
{
    writer: W,
    current_section: Option<Section>,
    is_first_ascii_row: bool,
}

impl<W> JsonSink<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            current_section: None,
            is_first_ascii_row: true,
        }
    }

    fn write_section<T>(&mut self, section_name: &str, section: &T) -> Result<(), ParseError>
    where
        T: Serialize,
    {
        write!(self.writer, "\"{section_name}\":")?;
        serde_json::to_writer(&mut self.writer, section).map_err(|e| ParseError::Error { message: e.to_string() })?;
        Ok(())
    }
}

impl<W> Sink for JsonSink<W>
where
    W: Write,
{
    fn start(&mut self) -> Result<(), ParseError> {
        write!(&mut self.writer, "{{")?;
        Ok(())
    }

    fn end(&mut self) -> Result<(), ParseError> {
        write!(self.writer, "}}}}")?;
        Ok(())
    }

    fn section_start(&mut self, section: Section) -> Result<(), ParseError> {
        if section.header.kind == SectionKind::AsciiLogData {
            write!(self.writer, "\"AsciiLogData\":{{\"headers\":")?;
            serde_json::to_writer(&mut self.writer, &section.ascii_headers)
                .map_err(|e| ParseError::Error { message: e.to_string() })?;
            write!(self.writer, ",\"rows\":[")?;
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

    fn ascii_row(&mut self, row: &[String]) -> Result<(), ParseError> {
        if !self.is_first_ascii_row {
            write!(self.writer, ",")?;
        }
        self.is_first_ascii_row = false;

        write!(self.writer, "[")?;
        for (i, r) in row.iter().enumerate() {
            if i != 0 {
                write!(self.writer, ",")?;
            }
            write!(self.writer, "\"{}\"", r)?;
        }
        write!(self.writer, "]")?;

        Ok(())
    }

    fn section_end(&mut self) -> Result<(), ParseError> {
        if let Some(section) = self.current_section.take() {
            let kind = section.header.kind;

            match kind {
                SectionKind::AsciiLogData => {
                    write!(self.writer, "],\"comments\":")?;
                    serde_json::to_writer(&mut self.writer, &section.comments)
                        .map_err(|e| ParseError::Error { message: e.to_string() })?;
                    write!(self.writer, ",\"header\":\"~{}\"", section.header.raw)?;
                }
                SectionKind::Well => self.write_section("WellInformation", &WellInformation::try_from(section)?)?,
                SectionKind::Curve => self.write_section("CurveInformation", &CurveInformation::try_from(section)?)?,
                SectionKind::Other => self.write_section("OtherInformation", &OtherInformation::try_from(section)?)?,
                SectionKind::Version => {
                    self.write_section("VersionInformation", &VersionInformation::try_from(section)?)?
                }
                SectionKind::Parameter => {
                    self.write_section("ParameterInformation", &ParameterInformation::try_from(section)?)?
                }
            };

            // AsciiLogData is expected to be the last section in a .las file.
            // That is how we can get away with making these assumptions.
            if kind != SectionKind::AsciiLogData {
                write!(self.writer, ",")?;
            }
        }

        Ok(())
    }
}
