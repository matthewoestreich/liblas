use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry, SectionKind, Sink},
    sections::{CurveInformation, OtherInformation, ParameterInformation, VersionInformation, WellInformation},
};
use serde::Serialize;
use std::io::Write;

// We store every section outside of AsciiLogData within the 'current_section'.
// Those sections are very small in comparison to ascii data. We directly stream
// and write the ascii data to the writer, no allocations or buffering.
pub struct YamlSink<W>
where
    W: Write,
{
    writer: W,
    current_section: Option<Section>,
}

impl<W> YamlSink<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            current_section: None,
        }
    }

    fn write_section<T>(&mut self, section_name: &str, section: &T) -> Result<(), ParseError>
    where
        T: Serialize,
    {
        writeln!(self.writer, "{section_name}:")?;

        let mut buf = Vec::new();
        serde_yaml_ng::to_writer(&mut buf, section).map_err(|e| ParseError::Error { message: e.to_string() })?;

        let s = String::from_utf8(buf).map_err(|e| ParseError::Error { message: e.to_string() })?;
        for line in s.lines() {
            writeln!(self.writer, "  {line}")?;
        }

        Ok(())
    }
}

impl<W> Sink for YamlSink<W>
where
    W: Write,
{
    fn section_start(&mut self, section: Section) -> Result<(), ParseError> {
        if section.header.kind == SectionKind::AsciiLogData {
            writeln!(self.writer, "AsciiLogData:")?;
            if let Some(ascii_headers) = section.ascii_headers.as_ref() {
                writeln!(self.writer, "  headers:")?;
                for header in ascii_headers {
                    writeln!(self.writer, "  - {header}")?;
                }
                writeln!(self.writer, "  rows:")?;
            }
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
        if row.is_empty() {
            return Err(ParseError::Error {
                message:
                    "[yaml_sink] Encountered empty ascii row! THIS SHOULD NEVER HAPPEN, PARSER SHOULD CATCH THIS FIRST!"
                        .to_string(),
            });
        }

        writeln!(self.writer, "  - - '{}'", row[0].raw)?;
        for row in row[1..].iter() {
            writeln!(self.writer, "    - '{}'", row.raw)?;
        }
        Ok(())
    }

    fn section_end(&mut self) -> Result<(), ParseError> {
        if let Some(section) = self.current_section.take() {
            let kind = section.header.kind;

            match kind {
                SectionKind::Well => self.write_section("WellInformation", &WellInformation::try_from(section)?)?,
                SectionKind::Curve => self.write_section("CurveInformation", &CurveInformation::try_from(section)?)?,
                SectionKind::Other => self.write_section("OtherInformation", &OtherInformation::try_from(section)?)?,
                SectionKind::Version => {
                    self.write_section("VersionInformation", &VersionInformation::try_from(section)?)?;
                }
                SectionKind::Parameter => {
                    self.write_section("ParameterInformation", &ParameterInformation::try_from(section)?)?
                }
                SectionKind::AsciiLogData => {
                    if let Some(comments) = section.comments.as_ref() {
                        writeln!(self.writer, "  comments:")?;
                        for comment in comments {
                            writeln!(self.writer, "  - {comment}")?;
                        }
                    }
                    writeln!(self.writer, "  header: ~{}", section.header.raw)?;
                }
            };
        }

        Ok(())
    }
}
