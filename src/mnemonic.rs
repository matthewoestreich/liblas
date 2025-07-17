use crate::{
    LibLasError::{self, *},
    Token,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MnemonicData {
    Float(f64),
    Text(String),
    Int(i64),
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
        } else if let Ok(i) = data_str.parse::<i64>() {
            MnemonicData::Int(i)
        } else if data_str.contains('.')
            && let Ok(f) = data_str.parse::<f64>()
        {
            MnemonicData::Float(f) // Try to parse value to float
        } else {
            MnemonicData::Text(data_str.to_string()) // Otherwise it is a string
        };

        return Ok(this);
    }

    fn unit_to_string(&self) -> String {
        return match &self.unit {
            Some(v) => v.into(),
            None => " ".into(),
        };
    }

    fn value_to_string(&self) -> String {
        return match &self.value {
            MnemonicData::Int(i) => i.to_string(),
            MnemonicData::Float(f) => {
                if f.fract() == 0.0 {
                    format!("{f:.1}")
                } else {
                    f.to_string()
                }
            }
            MnemonicData::Text(t) => t.to_string(),
        };
    }

    pub fn to_str(&self) -> String {
        let mut output = String::new();
        if !self.comments.is_empty() {
            output = format!("{}\n", self.comments.join("\n"));
        }
        return format!(
            "{output}{}.{} {} : {}",
            self.name,
            self.unit_to_string(),
            self.value_to_string(),
            self.description
        );
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
