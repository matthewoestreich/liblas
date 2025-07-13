use crate::LibLasError::{self, *};
use serde::{
  Deserialize, Deserializer, Serialize,
  ser::{SerializeMap, Serializer},
};
use std::collections::HashMap;

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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AsciiColumn {
  #[serde(rename = "NAME")]
  pub name: String,

  #[serde(rename = "DATA")]
  pub data: Vec<f64>,
}

#[derive(Default, Debug)]
pub struct AsciiLogData {
  pub columns: Vec<AsciiColumn>,
}

impl AsciiLogData {
  pub fn from_lines(lines: Vec<String>) -> Result<Self, LibLasError> {
    let mut iter = lines.into_iter();

    let header_line = iter
      .next()
      .ok_or(MalformedAsciiData("No header line found".to_string()))?;

    let mut tokens = header_line.split_whitespace();
    let first_token = tokens
      .next()
      .ok_or(MalformedAsciiData("Empty header line".to_string()))?;
    if first_token != "~A" {
      return Err(MalformedAsciiData("Header line must start with ~A".to_string()));
    }

    let column_names: Vec<String> = tokens.map(|s| s.to_string()).collect();
    if column_names.is_empty() {
      return Err(MalformedAsciiData("No columns found in header line".to_string()));
    }

    let mut columns: Vec<AsciiColumn> = column_names
      .into_iter()
      .map(|name| AsciiColumn { name, data: Vec::new() })
      .collect();

    for line in iter {
      let values: Vec<&str> = line.split_whitespace().collect();

      if values.len() != columns.len() {
        return Err(MalformedAsciiData(format!(
          "Data row length '{}' does not match column count '{}'",
          values.len(),
          columns.len(),
        )));
      }

      for (col, val_str) in columns.iter_mut().zip(values.iter()) {
        let val: f64 = match val_str.parse() {
          Ok(num) => num,
          Err(_) => {
            return Err(MalformedAsciiData(format!("Invalid float value: '{val_str}'")));
          }
        };
        col.data.push(val);
      }
    }

    Ok(AsciiLogData { columns })
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
    map.end()
  }
}

impl<'de> Deserialize<'de> for AsciiLogData {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let map: HashMap<String, Vec<f64>> = HashMap::deserialize(deserializer)?;
    let columns = map.into_iter().map(|(name, data)| AsciiColumn { name, data }).collect();
    Ok(AsciiLogData { columns })
  }
}
