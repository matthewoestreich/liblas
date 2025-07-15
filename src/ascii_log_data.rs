/*
~A (ASCII Log Data)
• The data section will always be the last section in a file.
• Only one "~A" section can occur in an LAS 2.0 file.
• Embedded blank lines anywhere in the section are forbidden
• Each column of data must be separated by at least one space. Consistency of format on every
line, while not required, is expected by many LAS readers. Right Justification of each column of
data and the same width of all data fields is highly recommended.
• Line length in the data section of unwrapped files are no longer restricted
• In wrap mode, the index channel will be on its own line
• In wrap mode, a line of data will be no longer than 80 characters. This includes a carriage return
and line feed
*/
use crate::{
  CurveInformation,
  LibLasError::{self, *},
  PeekableFileReader,
};
use serde::{
  self, Deserialize, Deserializer, Serialize,
  ser::{SerializeMap, Serializer},
};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsciiColumn {
  #[serde(rename = "NAME")]
  pub name: String,
  #[serde(rename = "DATA")]
  pub data: Vec<f64>,
}

#[derive(Debug)]
pub struct AsciiLogData {
  pub columns: Vec<AsciiColumn>,
  pub(crate) is_parsed: bool,
}

impl Default for AsciiLogData {
  fn default() -> Self {
    return Self {
      columns: vec![],
      is_parsed: false,
    };
  }
}

impl AsciiLogData {
  pub fn parse(
    reader: &mut PeekableFileReader,
    header_line: String,
    curve_info: &CurveInformation,
  ) -> Result<Self, LibLasError> {
    let column_names = Self::parse_header(header_line, curve_info)?;
    let mut this = AsciiLogData {
      columns: column_names
        .into_iter()
        .map(|name| return AsciiColumn { name, data: Vec::new() })
        .collect(),
      is_parsed: true,
    };

    while let Some(Ok(peeked_line)) = reader.peek() {
      if peeked_line.starts_with('~') {
        break;
      }

      let next_line = reader.next().ok_or(ReadingNextLine)??;

      // TODO : SKIPPING COMMENTS FOR NOW
      if next_line.starts_with("#") {
        continue;
      }

      let values: Vec<&str> = next_line.split_whitespace().collect();

      if values.len() != this.columns.len() {
        return Err(MalformedAsciiData(format!(
          "Data row length '{}' does not match column count '{}'",
          values.len(),
          this.columns.len(),
        )));
      }

      // Since we parse row by row, we 'zip' the header up with each data row.
      // This way we know which header to put each part of the data row into.
      for (col, val_str) in this.columns.iter_mut().zip(values.iter()) {
        col.data.push(val_str.parse()?); // Parse string into float64
      }
    }

    // Since ASCII Log Data is required to be last section in las files,
    // if we encounter anything other than a comment here, we error out.
    while let Some(Ok(nl)) = reader.next() {
      if nl.starts_with("#") {
        continue;
      }
      return Err(InvalidLasFile(
        "ASCII Log Data must be the last section in a .las file!".into(),
      ));
    }
    return Ok(this);
  }

  fn parse_header(header_line: String, curve_info: &CurveInformation) -> Result<Vec<String>, LibLasError> {
    // For pulling headers from "~A" header line. Example "~A" line (as string):
    //        "~A  Depth        GR        AMP3FT      TT3FT       AMPS1"
    // In "minified" versions of .las files, the headers (everything after "~A") may not exist.
    // This means that the "~Curve Information" section is required.
    // This is why we have to pass in curve info to the `parse` method. In case we need it.
    // If we are in a minified las file we need to pull the headers from the "~Curve Information" instead.
    let mut header_tokens = header_line.split_whitespace();
    let first_token = header_tokens
      .next()
      .ok_or(MalformedAsciiData("Empty header line".into()))?;
    if first_token != "~A" {
      return Err(MalformedAsciiData("Header line must start with ~A".into()));
    }

    let mut column_names: Vec<String> = header_tokens.map(|s| return s.to_string()).collect();
    if column_names.is_empty() {
      if curve_info.curves.is_empty() {
        return Err(InvalidLasFile("Missing '~Curve Information'. If a .las file excludes ASCII Log Data headers, a '~Curve Information' section is required!".into()));
      }
      column_names = curve_info.curves.iter().map(|c| return c.name.to_string()).collect();
    }

    // From the LAS specification : "The index curve (i.e. first curve) must be depth, time or index.
    // The only valid mnemonics for the index channel are DEPT, DEPTH, TIME, or INDEX."
    let valid_index_channel_names: Vec<String> = vec!["DEPT".into(), "DEPTH".into(), "TIME".into(), "INDEX".into()];
    if !valid_index_channel_names.contains(&column_names[0]) {
      return Err(InvalidLasFile(
        "The index curve (i.e. first curve) must be depth ('DEPT' or 'DEPTH'), time ('TIME') or index ('INDEX')."
          .into(),
      ));
    }

    return Ok(column_names);
  }
}

impl Serialize for AsciiLogData {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut map = serializer.serialize_map(Some(self.columns.len()))?;
    for col in &self.columns {
      map.serialize_entry(&col.name, &col.data)?;
    }
    return map.end();
  }
}

impl<'de> Deserialize<'de> for AsciiLogData {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let map: HashMap<String, Vec<f64>> = HashMap::deserialize(deserializer)?;
    let columns = map
      .into_iter()
      .map(|(name, data)| return AsciiColumn { name, data })
      .collect();
    return Ok(AsciiLogData {
      columns,
      is_parsed: true,
    });
  }
}
