use crate::{errors::LibLasError::*, *};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LasFile {
    #[serde(rename = "VersionInformation")]
    pub version_information: VersionInformation,
    #[serde(rename = "WellInformation")]
    pub well_information: WellInformation,
    #[serde(rename = "AsciiLogData")]
    pub ascii_log_data: AsciiLogData,
    #[serde(rename = "CurveInformation")]
    pub curve_information: CurveInformation,
    #[serde(rename = "OtherInformation")]
    pub other_information: Option<OtherInformation>,
    #[serde(rename = "ParameterInformation")]
    pub parameter_information: Option<ParameterInformation>,
    #[serde(skip)] // Holds a section and the index in which that section was parsed.
    parsed_sections: HashMap<Section, usize>,
}

impl LasFile {
    pub fn parse(file_path: PathBuf) -> Result<Self, LibLasError> {
        let mut this = Self {
            parsed_sections: HashMap::new(),
            ..Default::default()
        };

        let file = File::open(file_path).or(Err(OpeningLasFile))?;
        let reader = BufReader::new(file);
        let mut line_reader = reader.lines().peekable();

        // Any comments that live before any given section will be stored here and passed into each sections "parser".
        let mut current_comments: Vec<String> = vec![];

        while let Some(read_line) = line_reader.next() {
            let current_line = read_line.or(Err(ReadingNextLine))?;

            if current_line.starts_with(&Token::Comment()) {
                current_comments.push(current_line.clone());
                let parsed_comments = this.parse_comments(&mut line_reader)?;
                current_comments.extend(parsed_comments);
            } else if current_line.starts_with(&Token::VersionInformationSection()) {
                if !this.parsed_sections.is_empty() {
                    return Err(VersionInformationNotFirst);
                }
                this.check_section_not_parsed(&Section::VersionInformation)?;
                this.version_information = VersionInformation::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(Section::VersionInformation);
            } else if current_line.starts_with(&Token::WellInformationSection()) {
                this.check_section_not_parsed(&Section::WellInformation)?;
                this.well_information = WellInformation::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(Section::WellInformation);
            } else if current_line.starts_with(&Token::OtherSection()) {
                this.check_section_not_parsed(&Section::OtherInformation)?;
                this.other_information = Some(OtherInformation::parse(&mut line_reader, &mut current_comments)?);
                this.mark_section_parsed(Section::OtherInformation);
            } else if current_line.starts_with(&Token::ParameterInformationSection()) {
                this.check_section_not_parsed(&Section::ParameterInformation)?;
                this.parameter_information =
                    Some(ParameterInformation::parse(&mut line_reader, &mut current_comments)?);
                this.mark_section_parsed(Section::ParameterInformation);
            } else if current_line.starts_with(&Token::CurveInformationSection()) {
                this.check_section_not_parsed(&Section::CurveInformation)?;
                this.curve_information = CurveInformation::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(Section::CurveInformation);
            } else if current_line.starts_with(&Token::AsciiSection()) {
                this.check_section_not_parsed(&Section::AsciiLogData)?;
                this.ascii_log_data = AsciiLogData::parse(
                    &mut line_reader,
                    current_line,
                    &this.curve_information,
                    &mut current_comments,
                )?;
                this.mark_section_parsed(Section::AsciiLogData);
            }
        }

        return Ok(this);
    }

    pub fn new(
        version_information: VersionInformation,
        well_information: WellInformation,
        curve_information: CurveInformation,
        ascii_log_data: AsciiLogData,
        other_information: Option<OtherInformation>,
        parameter_information: Option<ParameterInformation>,
    ) -> Self {
        // The index is the order each section will be reconstructed in.
        let mut map = HashMap::from([
            (Section::VersionInformation, 0),
            (Section::WellInformation, 1),
            (Section::CurveInformation, 2),
        ]);

        if parameter_information.is_some() {
            map.insert(Section::ParameterInformation, map.len());
        }
        if other_information.is_some() {
            map.insert(Section::OtherInformation, map.len());
        }

        map.insert(Section::AsciiLogData, map.len());

        return Self {
            version_information,
            well_information,
            curve_information,
            other_information,
            parameter_information,
            ascii_log_data,
            parsed_sections: map,
        };
    }

    pub fn to_json_str(&self) -> Result<String, LibLasError> {
        return serde_json::to_string_pretty(self).map_err(|e| return ConvertingToJson(e.to_string()));
    }

    // - Convert this structure back into .las format
    // - We reconstruct the .las file in the same order we parsed it.
    // - If this las file was programmatically created, we use the order in the `fn new`.
    pub fn to_las_str(&self) -> String {
        let mut order: Vec<String> = vec![String::new(); Section::COUNT];

        if let Some(&index) = self.parsed_sections.get(&Section::VersionInformation) {
            order[index] = self.version_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&Section::WellInformation) {
            order[index] = self.well_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&Section::CurveInformation) {
            order[index] = self.curve_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&Section::ParameterInformation)
            && let Some(param_info) = &self.parameter_information
            && let Some(param_info_str) = param_info.to_str()
        {
            order[index] = param_info_str;
        }
        if let Some(&index) = self.parsed_sections.get(&Section::OtherInformation)
            && let Some(other_info) = &self.other_information
            && let Some(other_info_str) = other_info.to_str()
        {
            order[index] = other_info_str;
        }
        if let Some(&index) = self.parsed_sections.get(&Section::AsciiLogData) {
            order[index] = self.ascii_log_data.to_str();
        }

        return order.join("\n");
    }

    fn check_section_not_parsed(&self, section: &Section) -> Result<(), LibLasError> {
        if self.parsed_sections.contains_key(section) {
            return Err(DuplicateSectionFound(format!("{section:?}")));
        }
        return Ok(());
    }

    fn mark_section_parsed(&mut self, section: Section) {
        let index = self.parsed_sections.len();
        self.parsed_sections.insert(section, index);
    }

    // Consumes all comment lines up until the next line (which is viewed by peeking) isn't a comment.
    fn parse_comments(&self, line_reader: &mut PeekableFileReader) -> Result<Vec<String>, LibLasError> {
        let mut comments: Vec<String> = vec![];
        while let Some(Ok(peeked_line)) = line_reader.peek() {
            if !peeked_line.trim().to_string().starts_with(&Token::Comment()) {
                break;
            }
            let next_line = line_reader.next().ok_or(ReadingNextLine)??.trim().to_string();
            comments.push(next_line);
        }
        return Ok(comments);
    }
}
