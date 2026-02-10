use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum LasValue {
    Int(i64),
    Float(String),
    Text(String),
}

impl fmt::Display for LasValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LasValue::Int(i) => write!(f, "{i}"),
            LasValue::Float(lf) => write!(f, "{lf}"),
            LasValue::Text(t) => write!(f, "{t}"),
        }
    }
}

impl LasValue {
    pub fn parse(raw: &str) -> Option<LasValue> {
        let raw = raw.trim();
        if let Ok(i) = raw.parse::<i64>() {
            Some(LasValue::Int(i))
        } else if raw.contains('.')
        //&& let Ok(f) = raw.parse::<f64>()
        {
            Some(LasValue::Float(raw.to_string()))
        } else if raw.is_empty() {
            None
        } else {
            Some(LasValue::Text(raw.to_string()))
        }
    }
}
