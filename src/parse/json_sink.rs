use crate::{
    ParseError,
    parse::{LasFloat, Section, SectionEntry, SectionKind, Sink},
};
use serde::{Serialize, ser::Serializer as SeSerializer};
use serde_json::{Serializer, ser::PrettyFormatter};
use std::io::Write;

/// Streaming JSON sink for LAS files.
pub struct JsonSink<W: Write> {
    writer: W,
    //serializer: Serializer<W, PrettyFormatter<'static>>,
    current_ascii_headers: Option<Vec<String>>,
    in_ascii_section: bool,
}

impl<W: Write> JsonSink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            //serializer: Serializer::pretty(writer),
            current_ascii_headers: None,
            in_ascii_section: false,
        }
    }
}

impl<W: Write> Sink for JsonSink<W> {
    fn start_section(&mut self, section: Section) -> Result<(), ParseError> {
        serde_json::to_writer_pretty(&mut self.writer, &section)
            .map_err(|e| ParseError::Error { message: e.to_string() })?;
        /*
        // If the previous section was ASCII, end the rows array
        if self.in_ascii_section {
            // Close ASCII rows array
            self.in_ascii_section = false;
        }

        match section.header.kind {
            SectionKind::AsciiLogData => {
                self.current_ascii_headers = Some(
                    section
                        .entries
                        .iter()
                        .filter_map(|e| {
                            if let SectionEntry::Delimited(d) = e {
                                Some(d.mnemonic.clone())
                            } else {
                                None
                            }
                        })
                        .collect(),
                );
                self.in_ascii_section = true;

                // Serialize ASCII section with headers
                let ascii_start = serde_json::json!({
                    "headers": self.current_ascii_headers.as_ref().unwrap(),
                    "rows": [],
                    "comments": section.comments,
                    "header": section.header.raw.clone()
                });
                serde_json::to_writer_pretty(&mut self.writer, &ascii_start)
                    .map_err(|e| ParseError::Error { message: e.to_string() })?;
                //serde_json::to_writer(&mut self.serializer.get_mut(), &ascii_start)
                //    .map_err(|e| ParseError::Error { message: e.to_string() })?;
            }
            _ => {
                // Serialize regular sections immediately
                serde_json::to_writer_pretty(&mut self.writer, &section)
                    .map_err(|e| ParseError::Error { message: e.to_string() })?;
            }
        }
        */

        Ok(())
    }

    fn entry(&mut self, entry: SectionEntry) -> Result<(), ParseError> {
        if let SectionEntry::Delimited(kv) = entry {
            serde_json::to_writer_pretty(&mut self.writer, &kv)
                .map_err(|e| ParseError::Error { message: e.to_string() })?;
        }
        Ok(())
    }

    fn ascii_row(&mut self, row: &[LasFloat]) -> Result<(), ParseError> {
        /*
        if !self.in_ascii_section {
            return Err(ParseError::Error {
                message: "ASCII row outside of ASCII section".to_string(),
            });
        }
        */

        // Convert numbers to strings to match your JSON example
        let row_as_strings: Vec<String> = row.iter().map(|f| format!("{}", f)).collect();

        // Append row to rows array (streaming; we just serialize each row immediately)
        serde_json::to_writer_pretty(&mut self.writer, &row_as_strings)
            .map_err(|e| ParseError::Error { message: e.to_string() })
    }

    fn end_section(&mut self) -> Result<(), ParseError> {
        // If ASCII, mark it ended
        self.in_ascii_section = false;
        self.current_ascii_headers = None;
        Ok(())
    }
}

