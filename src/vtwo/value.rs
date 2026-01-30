#[derive(Debug, Clone)]
pub enum LasValue {
    Int(i64),
    Float(f64),
    Text(String),
}

impl LasValue {
    pub fn parse(raw: &str) -> LasValue {
        let raw = raw.trim();

        if let Ok(i) = raw.parse::<i64>() {
            LasValue::Int(i)
        } else if raw.contains('.')
            && let Ok(f) = raw.parse::<f64>()
        {
            LasValue::Float(f)
        } else {
            LasValue::Text(raw.to_string())
        }
    }
}

impl From<&str> for LasValue {
    fn from(value: &str) -> Self {
        LasValue::parse(value)
    }
}
