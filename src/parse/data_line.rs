use crate::{parse::LasValue, write_comments};
use serde::{Deserialize, Serialize};
use std::fmt;

// The sections "VERSION", "WELL", "CURVE" and "PARAMETER" use line delimiters.
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataLine {
    pub mnemonic: String,
    pub unit: Option<String>,
    pub value: Option<LasValue>,
    pub description: Option<String>,
    pub comments: Option<Vec<String>>,
}

impl fmt::Display for DataLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_comments(f, &self.comments)?;
        write!(f, "{}.", self.mnemonic)?;
        if let Some(unit) = self.unit.as_ref() {
            write!(f, "{unit}")?;
        }
        write!(f, " ")?;
        if let Some(value) = self.value.as_ref() {
            write!(f, "{value} ")?;
        }
        write!(f, ":")?;
        if let Some(description) = self.description.as_ref() {
            write!(f, " {description}")?;
        }
        Ok(())
    }
}
