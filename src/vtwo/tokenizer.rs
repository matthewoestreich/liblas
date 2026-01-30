use std::io::{self, BufRead};

pub struct LasTokenizer<R>
where
    R: BufRead,
{
    reader: R,
    buffer: String,
    line: usize,
}

impl<R> LasTokenizer<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
            line: 0,
        }
    }

    pub fn next_token(&mut self) -> io::Result<Option<LasToken>> {
        self.buffer.clear();

        let bytes = self.reader.read_line(&mut self.buffer)?;
        if bytes == 0 {
            return Ok(None); // EOF
        }

        self.line += 1;
        let line = self.buffer.trim_end_matches(&['\n', '\r'][..]);
        let trimmed = line.trim_start();

        if trimmed.is_empty() {
            return Ok(Some(LasToken::Blank { line_number: self.line }));
        }

        if let Some(text) = trimmed.strip_prefix('#') {
            return Ok(Some(LasToken::Comment {
                text: text.trim().to_string(),
                line_number: self.line,
            }));
        }

        if let Some(text) = trimmed.strip_prefix('~') {
            return Ok(Some(LasToken::SectionHeader {
                name: text.trim().to_string(),
                line_number: self.line,
            }));
        }

        Ok(Some(LasToken::DataLine {
            raw: line.to_string(),
            line_number: self.line,
        }))
    }
}

impl<R: BufRead> Iterator for LasTokenizer<R> {
    type Item = Result<LasToken, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(tok)) => Some(Ok(tok)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[derive(Debug)]
pub enum LasToken {
    SectionHeader {
        name: String, // "~Curve Information Section"
        line_number: usize,
    },
    Comment {
        text: String,
        line_number: usize,
    },
    DataLine {
        raw: String,
        line_number: usize,
    },
    Blank {
        line_number: usize,
    },
}
