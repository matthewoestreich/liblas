use crate::LasioError::{self, *};

#[derive(Debug)]
pub enum MnemonicData {
  Float(f64),
  Text(String),
}

#[derive(Debug, Default)]
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
    MnemonicData::Text(String::new())
  }
}

impl Mnemonic {
  pub fn from_line(line: &str) -> Result<Self, LasioError> {
    let (before_colon, after_colon) = line
      .split_once(':')
      .ok_or_else(|| MissingRequiredDelimeter(":".to_string()))?;

    let description = after_colon.trim().to_string();

    let (name_part, after_dot) = before_colon
      .split_once('.')
      .ok_or_else(|| MissingRequiredDelimeter(".".to_string()))?;

    let name = name_part.trim().to_string();

    if name.is_empty() {
      return Err(MissingRequiredMnemonicField("name".to_string()));
    }

    let after_dot = after_dot.trim_start();

    let (unit, data_str) = match after_dot.split_once(char::is_whitespace) {
      Some((u, rest)) => {
        let data = rest.trim();
        if data.is_empty() {
          return Err(MissingData);
        }
        (Some(u.trim().to_string()), data)
      }
      None => {
        if after_dot.is_empty() {
          return Err(MissingData);
        }
        (None, after_dot)
      }
    };

    // Data should be either a float or a string.
    let data = match data_str.parse::<f64>() {
      Ok(f) => MnemonicData::Float(f),
      Err(_) => MnemonicData::Text(data_str.to_string()),
    };

    Ok(Mnemonic {
      name,
      unit: unit.filter(|u| !u.is_empty()),
      data,
      description,
    })
  }
}
