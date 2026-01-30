use crate::{
    LibLasErrorOld::{self, ReadingNextLine},
    PeekableFileReader, TokenOld,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OtherInformationOld {
    pub text: String,
    pub comments: Vec<String>,
}

impl OtherInformationOld {
    pub fn parse(reader: &mut PeekableFileReader, current_comments: &mut Vec<String>) -> Result<Self, LibLasErrorOld> {
        let mut this = Self::default();

        // Comments were above the "~Other Info" section
        if !current_comments.is_empty() {
            this.comments = current_comments.to_vec();
            // Clear comments because any additional comments may be intended for a mnemonic or a diff section entirely.
            current_comments.clear();
        }

        while let Some(Ok(peeked_line)) = reader.peek() {
            if peeked_line.trim().to_string().starts_with(&TokenOld::Tilde()) {
                break;
            }

            let line = &reader.next().ok_or(ReadingNextLine)??.trim().to_string();

            if line.starts_with(&TokenOld::Comment()) {
                current_comments.push(line.clone());
                continue;
            }

            let final_line = if line.ends_with(" ") { line } else { &format!("{line} ") };

            this.text.push_str(final_line);
        }

        this.text = this.text.trim().to_string();
        return Ok(this);
    }

    pub fn to_str(&self) -> Option<String> {
        if self.comments.is_empty() && self.text.is_empty() {
            return None;
        }
        let mut output = "~Other Information".to_string();
        if !self.comments.is_empty() {
            output = format!("{}\n{output}", self.comments.join(" "));
        }
        if !self.text.is_empty() {
            output = format!("{output}\n{}", self.text);
        }
        return Some(output);
    }

    pub fn new(text: String, comments: Vec<String>) -> Self {
        return Self { text, comments };
    }
}
