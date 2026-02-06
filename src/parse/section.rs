use crate::{
    ParseError,
    parse::{DataLine, LasFloat, LasValue, SectionEntry, SectionHeader, SectionKind, str_contains},
};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct Section {
    pub header: SectionHeader,
    pub line: usize,
    pub entries: Vec<SectionEntry>,
    pub ascii_headers: Option<Vec<String>>,
    pub ascii_rows: Vec<Vec<LasFloat>>,
    pub comments: Option<Vec<String>>,
}

impl Section {
    pub fn new(name: String, line: usize) -> Self {
        Self {
            header: SectionHeader {
                kind: SectionKind::from(name.as_str()),
                raw: name,
            },
            line,
            entries: vec![],
            ascii_headers: None,
            ascii_rows: vec![],
            comments: None,
        }
    }

    pub fn parse_line(
        &mut self,
        raw: &str,
        line_number: usize,
        comments: Option<Vec<String>>,
    ) -> Result<(), ParseError> {
        if self.header.kind == SectionKind::AsciiLogData {
            return self.parse_ascii_log_line(raw, line_number);
        }

        if self.header.kind == SectionKind::Other {
            self.entries.push(SectionEntry::Raw {
                text: raw.trim().to_string(),
                comments,
            });
            return Ok(());
        }

        // ############################
        // -- LAS DATA LINE LAYOUT --
        // ############################
        //
        // MNEM.UNITS    DATA   : DESCRIPTION
        //     |     |          |
        //     |     |          +-- last ':' on line
        //     |     +-- first space ' ' AFTER first '.'
        //     +-- first '.' in line
        //
        // -- MNEM -- (required)
        // "mnemonic. This mnemonic can be of any length but must not contain any internal
        // spaces, dots, or colons. Spaces are permitted in front of the mnemonic and between the
        // end of the mnemonic and the dot."
        //
        // -- UNITS -- (optional)
        // "units of the mnemonic (if applicable). The units, if used, must be located directly
        // after the dot. There must be no spaces between the units and the dot. The units can be of
        // any length but must not contain any colons or internal spaces."
        //
        // -- DATA (aka VALUE) -- (optional)
        // "value of, or data relating to the mnemonic. This value or input can be of any length
        // and can contain spaces, dots or colons as appropriate. It must be preceded by at least one
        // space to demarcate it from the units and must be to the left of the last colon in the line."
        //
        // -- DESCRIPTION -- (optional)
        // "description or definition of the mnemonic. It is always located to the right
        // of the last colon. The length of the line is no longer limited."

        let mut space: Option<usize> = None;
        let mut period: Option<usize> = None;
        let mut colon: Option<usize> = None;

        for (i, bytes) in raw.bytes().enumerate() {
            match bytes as char {
                // We only need to make note of the index for the first space in a line that comes AFTER the first period in a line.
                ' ' if space.is_none() && period.is_some() => {
                    space = Some(i);
                }
                // Only record index of first period in a line.
                '.' if period.is_none() => {
                    period = Some(i);
                }
                // We need to use the last colon on a line as a delimiter. Therefore, update the colon index each time we see one.
                ':' => {
                    colon = Some(i);
                }
                _ => {}
            };
        }

        let mut mnemonic: Option<String> = None;
        let mut unit: Option<String> = None;
        let mut value: Option<LasValue> = None;
        let mut description: Option<String> = None;

        if let Some(period_index) = period {
            let raw_mnemonic = raw[..period_index].trim().to_string();
            Self::validate_mnemonic(&raw_mnemonic, raw, line_number)?;
            mnemonic = Some(raw_mnemonic);

            if let Some(space_index) = space
                && let Some(colon_index) = colon
            {
                let raw_unit = raw[period_index..space_index].trim_start_matches('.').to_string();
                Self::validate_unit(&raw_unit, raw, line_number)?;
                unit = Some(raw_unit);

                if space_index > colon_index {
                    return Err(ParseError::MissingDelimiter {
                        delimiter: "Missing ' ' in line! Line must contain at least one space between first period on line and last colon on line!".to_string(),
                        line_number,
                        line: raw.to_string(),
                    });
                }

                value = LasValue::parse(&raw[space_index..colon_index]);
            }
        }

        if let Some(colon_index) = colon {
            // Everything from the last recorded colon index until end of line is description.
            description = Some(raw[colon_index + 1..raw.len()].trim().to_string());
        }

        self.entries.push(SectionEntry::Delimited(DataLine {
            comments,
            value,
            unit: unit.filter(|u| !u.is_empty()),
            description: description.filter(|d| !d.is_empty()),
            mnemonic: mnemonic.ok_or(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            })?,
        }));

        Ok(())
    }

    fn parse_ascii_log_line(&mut self, raw: &str, line_number: usize) -> Result<(), ParseError> {
        // If we are missing headers here it means we haven't parsed the Curve section yet.
        // Since ASCII section has to be the last section (per CWLS v2.0) it means we have
        // and invalid LAS file.
        let headers = self
            .ascii_headers
            .as_ref()
            .ok_or(ParseError::AsciiLogDataSectionNotLast { line_number })?;

        let values: Vec<LasFloat> = raw
            .split_whitespace()
            .map(|s| {
                s.parse::<LasFloat>().map_err(|_| ParseError::InvalidAsciiValue {
                    line_number,
                    raw_value: s.to_string(),
                })
            })
            .collect::<Result<_, _>>()?;

        if values.len() != headers.len() {
            return Err(ParseError::AsciiColumnsMismatch {
                line_number,
                num_cols_in_headers: headers.len(),
                num_cols_in_row: values.len(),
            });
        }

        self.ascii_rows.push(values);
        Ok(())
    }

    fn validate_mnemonic(raw_mnemonic: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
        if raw_mnemonic.is_empty() {
            return Err(ParseError::MissingRequiredKey {
                key: "mnemonic".to_string(),
                line_number,
                line: raw.to_string(),
            });
        }
        let invalid_mnemonic_chars = str_contains(raw_mnemonic, &['.', ':', ' ']);
        if !invalid_mnemonic_chars.is_empty() {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "mnemonic".to_string(),
                line_number,
                invalid_chars: invalid_mnemonic_chars,
                line: raw.to_string(),
            });
        }
        Ok(())
    }

    fn validate_unit(raw_unit: &str, raw: &str, line_number: usize) -> Result<(), ParseError> {
        if raw_unit.starts_with(" ") {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "units".to_string(),
                line_number,
                invalid_chars: Vec::from([' ']),
                line: raw.to_string(),
            });
        }
        let invalid_unit_chars = str_contains(raw_unit, &[' ', ':']);
        if !invalid_unit_chars.is_empty() {
            return Err(ParseError::DelimetedValueContainsInvalidChars {
                key: "units".to_string(),
                line_number,
                invalid_chars: invalid_unit_chars,
                line: raw.to_string(),
            });
        }
        Ok(())
    }
}
