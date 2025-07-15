use crate::{
  LibLasError::{self, *},
  Token,
};
use serde::{Deserialize, Serialize};

/**
 - [[[MNEM]]] = mnemonic. This mnemonic can be of any length but must not contain any internal
   spaces, dots, or colons. Spaces are permitted in front of the mnemonic and between the
   end of the mnemonic and the dot.
 - [[[UNITS]]] = units of the mnemonic (if applicable). The units, if used, must be located directly
   after the dot. There must be no spaces between the units and the dot. The units can be of
   any length but must not contain any colons or internal spaces.
 - [[[DATA]]] = value of, or data relating to the mnemonic. This value or input can be of any length
   and can contain spaces, dots or colons as appropriate. It must be preceded by at least one
   space to demarcate it from the units and must be to the left of the last colon in the line.
 - [[[DESCRIPTION]]] = description or definition of the mnemonic. It is always located to the right
   of the last colon. The length of the line is no longer limited.
*/

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MnemonicData {
  Float(f64),
  Text(String),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Mnemonic {
  // Name of mnemonic
  pub name: String,
  // Unit of measurement (eg. FT (feet), DEGF (degrees farenheit))
  pub unit: Option<String>,
  // Value for this mnemonic
  pub value: MnemonicData,
  // Description of mnemonic
  pub description: String,
  // Comments for a mnemonic
  pub comments: Vec<String>,
}

impl Default for MnemonicData {
  fn default() -> Self {
    return MnemonicData::Text(String::new());
  }
}

impl Mnemonic {
  pub fn from_str(line: &str, current_comments: &mut Vec<String>) -> Result<Self, LibLasError> {
    let mut this = Self::default();

    if !current_comments.is_empty() {
      this.comments = current_comments.to_vec();
      current_comments.clear();
    }

    // Split at the *last* colon to isolate description
    let (before_colon, after_colon) = line
      .rsplit_once(&Token::Colon())
      .ok_or_else(|| return MissingRequiredDelimeter(Token::Colon()))?;

    this.description = after_colon.trim().to_string();

    // Find the position of the '.' in the left-hand part
    let dot_index = before_colon
      .find(&Token::Period())
      .ok_or_else(|| return MissingRequiredDelimeter(Token::Period()))?;

    // Everything before '.' is mnemonic (trim it)
    this.name = before_colon[..dot_index].trim().to_string();
    if this.name.is_empty() {
      return Err(MissingRequiredMnemonicField("name".into()));
    }

    // After the '.' is unit (no spaces allowed until value starts)
    let after_dot = &before_colon[dot_index + 1..];
    let (unit, data_str) = if after_dot.is_empty() {
      (None, "") // No unit, no value
    } else if after_dot.starts_with(char::is_whitespace) {
      // Space immediately after the dot → no unit
      (None, after_dot.trim())
    } else {
      // Possibly unit followed by value
      match after_dot.split_once(char::is_whitespace) {
        Some((u, rest)) => (Some(u.trim().to_string()), rest.trim()),
        None => (Some(after_dot.trim().to_string()), ""),
      }
    };

    this.unit = unit;

    this.value = if data_str.is_empty() {
      MnemonicData::Text("".to_string())
    } else if let Ok(f) = data_str.parse::<f64>() {
      MnemonicData::Float(f) // Try to parse value to float
    } else {
      MnemonicData::Text(data_str.to_string()) // Otherwise it is a string
    };

    return Ok(this);
  }

  pub fn new(name: String, unit: Option<String>, value: MnemonicData, description: String) -> Self {
    return Self {
      name,
      unit,
      description,
      value,
      comments: vec![],
    };
  }
}
