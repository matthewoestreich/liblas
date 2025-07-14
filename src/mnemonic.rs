use crate::LibLasError::{self, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MnemonicData {
  Float(f64),
  Text(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Mnemonic {
  // Name of mnemonic
  pub name: String,
  // Unit of measurement (eg. FT (feet), DEGF (degrees farenheit))
  pub unit: Option<String>,
  // Data mnemonic holds
  pub data: MnemonicData,
  // Description of mnemonic
  pub description: String,
}

impl Default for MnemonicData {
  fn default() -> Self {
    return MnemonicData::Text(String::new());
  }
}

impl Mnemonic {
  pub fn from_line(line: &str) -> Result<Self, LibLasError> {
    let (before_colon, after_colon) = line
      .split_once(':')
      .ok_or_else(|| return MissingRequiredDelimeter(":".to_string()))?;

    let description = after_colon.trim().to_string();

    let (name_part, after_dot) = before_colon
      .split_once('.')
      .ok_or_else(|| return MissingRequiredDelimeter(".".to_string()))?;

    let name = name_part.trim().to_string();

    if name.is_empty() {
      return Err(MissingRequiredMnemonicField("name".to_string()));
    }

    let after_dot = after_dot.trim_end();

    let (unit, data_str) = if after_dot.trim_start().is_empty() {
      (None, "")
    } else if after_dot.chars().next().unwrap().is_whitespace() {
      (None, after_dot.trim())
    } else {
      match after_dot.trim_start().split_once(char::is_whitespace) {
        Some((u, rest)) => (Some(u.trim().to_string()), rest.trim()),
        None => (Some(after_dot.trim().to_string()), ""), // unit present, no data
      }
    };

    let data = if data_str.is_empty() {
      MnemonicData::Text("".to_string())
    } else if let Ok(f) = data_str.parse::<f64>() {
      MnemonicData::Float(f)
    } else {
      MnemonicData::Text(data_str.to_string())
    };

    return Ok(Mnemonic {
      name,
      unit: unit.filter(|u| return !u.is_empty()),
      data,
      description,
    });
  }
}
