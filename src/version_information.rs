use crate::{
    LibLasError::{self, ReadingNextLine},
    Mnemonic, PeekableFileReader, Token,
};
use serde::{self, Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Debug, Default, Deserialize)]
pub struct VersionInformation {
    #[serde(rename = "VERS")]
    pub version: Mnemonic,
    #[serde(rename = "WRAP")]
    pub wrap: Mnemonic,
    pub additional: Vec<Mnemonic>,
    pub comments: Vec<String>,
}

impl VersionInformation {
    pub fn parse(reader: &mut PeekableFileReader, current_comments: &mut Vec<String>) -> Result<Self, LibLasError> {
        let mut this = Self::default();

        // Comments were above the "~Version Information" section.
        if !current_comments.is_empty() {
            this.comments = current_comments.to_vec();
            // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
            current_comments.clear();
        }

        while let Some(Ok(peeked_line)) = reader.peek() {
            if peeked_line.trim().to_string().starts_with(&Token::Tilde()) {
                break;
            }

            let next_line = reader.next().ok_or(ReadingNextLine)??.trim().to_string();

            if next_line.starts_with(&Token::Comment()) {
                current_comments.push(next_line.clone());
                continue;
            }

            if next_line.starts_with("VERS") {
                this.version = Mnemonic::from_str(&next_line, current_comments)?;
            } else if next_line.starts_with("WRAP") {
                this.wrap = Mnemonic::from_str(&next_line, current_comments)?;
            } else {
                let x = Mnemonic::from_str(&next_line, current_comments)?;
                this.additional.push(x);
            }
        }

        return Ok(this);
    }

    pub fn to_str(&self) -> String {
        let mut output = "~Version Information".to_string();
        if !self.comments.is_empty() {
            output = format!("{}\n{output}", self.comments.join("\n"));
        }
        output = format!("{output}\n{}", self.version.to_str());
        output = format!("{output}\n{}", self.wrap.to_str());
        if !self.additional.is_empty() {
            self.additional
                .iter()
                .for_each(|a| output = format!("{output}\n{}", a.to_str()));
        }
        return output;
    }

    pub fn new(version: Mnemonic, wrap: Mnemonic, extra: Vec<Mnemonic>, comments: Vec<String>) -> Self {
        return Self {
            version,
            wrap,
            additional: extra,
            comments,
        };
    }
}

impl Serialize for VersionInformation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Estimate the number of fields
        let mut map = serializer.serialize_map(Some(2 + self.additional.len() + 1))?;
        map.serialize_entry("VERS", &self.version)?;
        map.serialize_entry("WRAP", &self.wrap)?;
        for mnemonic in &self.additional {
            map.serialize_entry(&mnemonic.name, mnemonic)?;
        }
        map.serialize_entry("comments", &self.comments)?;
        return map.end();
    }
}
