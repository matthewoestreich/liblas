use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum LasValue {
    Int(i64),
    Text(String),
}

impl fmt::Display for LasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LasValue::Int(i) => write!(f, "{i}"),
            LasValue::Text(t) => write!(f, "{t}"),
        }
    }
}

impl LasValue {
    pub fn parse(raw: &str) -> Option<LasValue> {
        let raw = raw.trim();
        if raw.is_empty() {
            None
        } else if let Ok(i) = raw.parse::<i64>() {
            Some(LasValue::Int(i))
        } else {
            Some(LasValue::Text(raw.to_string()))
        }
    }
}
