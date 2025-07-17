use crate::{
    CurveInformation,
    LibLasError::{self, *},
    PeekableFileReader, Token,
};
use serde::{
    self, Deserialize, Serialize,
    ser::{SerializeMap, Serializer},
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AsciiColumn {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "DATA")]
    pub data: Vec<f64>,
}

impl AsciiColumn {
    pub fn new(name: String, data: Vec<f64>) -> Self {
        return Self { name, data };
    }
}

#[derive(Debug, Deserialize)]
pub struct AsciiLogData {
    pub data: Vec<AsciiColumn>,
    pub comments: Vec<String>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub(crate) has_column_names: bool,
}

impl Default for AsciiLogData {
    fn default() -> Self {
        return Self {
            data: vec![],
            comments: vec![],
            has_column_names: true,
        };
    }
}

impl AsciiLogData {
    pub fn parse(
        reader: &mut PeekableFileReader,
        header_line: String,
        curve_info: &CurveInformation,
        current_comments: &mut Vec<String>,
    ) -> Result<Self, LibLasError> {
        let header = header_line.clone();
        let column_names = Self::parse_header(header_line, curve_info)?;

        let mut this = Self {
            comments: vec![],
            data: column_names
                .into_iter()
                .map(|name| return AsciiColumn { name, data: Vec::new() })
                .collect(),
            has_column_names: header.split_whitespace().nth(1).is_some(),
        };

        // Comments were above the "~A" section
        if !current_comments.is_empty() {
            this.comments = current_comments.to_vec();
            // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
            current_comments.clear();
        }

        while let Some(Ok(peeked_line)) = reader.peek() {
            if peeked_line.trim().to_string().starts_with(&Token::Tilde()) {
                break;
            }

            let next_line = reader.next().ok_or(ReadingNextLine)??;

            // TODO : SKIPPING COMMENTS FOR NOW
            if next_line.starts_with(&Token::Comment()) {
                continue;
            }

            let values: Vec<&str> = next_line.split_whitespace().collect();

            if values.len() != this.data.len() {
                return Err(MalformedAsciiData(format!(
                    "Data row length '{}' does not match column count '{}'",
                    values.len(),
                    this.data.len(),
                )));
            }

            // Since we parse row by row, we 'zip' the header up with each data row.
            // This way we know which header to put each part of the data row into.
            for (col, val_str) in this.data.iter_mut().zip(values.iter()) {
                col.data.push(val_str.parse()?); // Parse string into float64
            }
        }

        // Since ASCII Log Data is required to be last section in las files,
        // if we encounter anything after ASCII data here, we error out.
        // From the spec: "Comment lines can appear anywhere above the ~A section"
        if reader.next().is_some() {
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
        // If we are in a minified las file we need to pull the headers from the "~Curve Information" instead.
        let mut header_tokens = header_line.split_whitespace();
        let first_token = header_tokens
            .next()
            .ok_or(MalformedAsciiData("Empty header line".into()))?;
        if first_token != Token::AsciiSection() {
            return Err(MalformedAsciiData("Header line must start with ~A".into()));
        }

        let mut column_names: Vec<String> = header_tokens.map(|s| return s.to_string()).collect();
        if column_names.is_empty() {
            if curve_info.curves.is_empty() {
                return Err(InvalidLasFile("Missing '~Curve Information'. If a .las file excludes ASCII Log Data headers, a '~Curve Information' section is required!".into()));
            }
            column_names = curve_info.curves.iter().map(|c| return c.name.to_string()).collect();
        }

        // From the LAS specification:
        // "The index curve (i.e. first curve) must be depth (DEPT|DEPTH), time (TIME) or index (INDEX).
        let valid_index_channel_names: Vec<String> = vec!["dept".into(), "depth".into(), "time".into(), "index".into()];
        if !valid_index_channel_names.contains(&column_names[0].to_lowercase()) {
            return Err(InvalidLasFile(
        "The index curve (i.e. first curve) must be depth ('DEPT' or 'DEPTH'), time ('TIME') or index ('INDEX')."
          .into(),
      ));
        }

        return Ok(column_names);
    }

    pub fn to_str(&self) -> String {
        let num_rows = self.data[0].data.len();
        let num_cols = self.data.len();

        let mut result = String::from("~A ");
        if self.has_column_names {
            for (i, col) in self.data.iter().enumerate() {
                result.push_str(&format!("{:<10}", col.name)); // 10-character padded name
                if i != num_cols - 1 {
                    result.push(' ');
                }
            }
        }
        result.push('\n');

        for row_idx in 0..num_rows {
            for col_idx in 0..num_cols {
                let val = self.data[col_idx].data[row_idx];
                result.push_str(&format!("{val:<10.4}")); // width 10, 4 decimals
                if col_idx != num_cols - 1 {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        if !self.comments.is_empty() {
            result = format!("{}\n{result}", self.comments.join("\n"));
        }

        return result;
    }

    pub fn new(data: Vec<AsciiColumn>, comments: Vec<String>) -> Self {
        return Self {
            data,
            comments,
            has_column_names: true,
        };
    }
}

impl Serialize for AsciiLogData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.data.len()))?;
        for col in &self.data {
            map.serialize_entry(&col.name, &col.data)?;
        }
        return map.end();
    }
}
