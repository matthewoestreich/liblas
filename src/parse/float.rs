use crate::ParseError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone)]
pub struct LasFloat {
    pub value: f64,
    pub raw: String,
}

impl FromStr for LasFloat {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.parse::<f64>().map_err(|_| ParseError::InvalidAsciiFloatValue {
            raw_value: s.to_string(),
        })?;
        Ok(LasFloat {
            raw: s.to_string(),
            value,
        })
    }
}

impl fmt::Display for LasFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl Serialize for LasFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.raw)
    }
}

impl<'de> Deserialize<'de> for LasFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse::<LasFloat>()
            .map_err(serde::de::Error::custom)
    }
}

impl PartialEq for LasFloat {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for LasFloat {}
