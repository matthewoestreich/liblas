use std::io;

use crate::tokenizer::LasToken;

#[derive(Debug)]
pub enum ParseError {
    Io(io::Error),
    UnexpectedToken {
        token: LasToken,
        line: usize,
    },
    MissingRequiredKey {
        key: String,
        line_number: usize,
        line: String,
    },
    MissingDelimiter {
        delimiter: String,
        line_number: usize,
        line: String,
    },
    InvalidNumber {
        value: String,
        line: usize,
    },
    Other {
        message: String,
        line: usize,
    },
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}
