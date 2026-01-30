use crate::{errors::LibLasErrorOld::*, *};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LasFileOld {
    #[serde(rename = "VersionInformation")]
    pub version_information: VersionInformationOld,
    #[serde(rename = "WellInformation")]
    pub well_information: WellInformationOld,
    #[serde(rename = "AsciiLogData")]
    pub ascii_log_data: AsciiLogDataOld,
    #[serde(rename = "CurveInformation")]
    pub curve_information: CurveInformationOld,
    #[serde(rename = "OtherInformation")]
    pub other_information: Option<OtherInformationOld>,
    #[serde(rename = "ParameterInformation")]
    pub parameter_information: Option<ParameterInformationOld>,
    #[serde(skip)] // Holds a section and the index in which that section was parsed.
    parsed_sections: HashMap<SectionOld, usize>,
}

impl LasFileOld {
    pub fn parse(file_path: PathBuf) -> Result<Self, LibLasErrorOld> {
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

            if current_line.starts_with(&TokenOld::Comment()) {
                current_comments.push(current_line.clone());
                let parsed_comments = this.parse_comments(&mut line_reader)?;
                current_comments.extend(parsed_comments);
            } else if current_line.starts_with(&TokenOld::VersionInformationSection()) {
                if !this.parsed_sections.is_empty() {
                    return Err(VersionInformationNotFirst);
                }
                this.check_section_not_parsed(&SectionOld::VersionInformation)?;
                this.version_information = VersionInformationOld::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(SectionOld::VersionInformation);
            } else if current_line.starts_with(&TokenOld::WellInformationSection()) {
                this.check_section_not_parsed(&SectionOld::WellInformation)?;
                this.well_information = WellInformationOld::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(SectionOld::WellInformation);
            } else if current_line.starts_with(&TokenOld::OtherSection()) {
                this.check_section_not_parsed(&SectionOld::OtherInformation)?;
                this.other_information = Some(OtherInformationOld::parse(&mut line_reader, &mut current_comments)?);
                this.mark_section_parsed(SectionOld::OtherInformation);
            } else if current_line.starts_with(&TokenOld::ParameterInformationSection()) {
                this.check_section_not_parsed(&SectionOld::ParameterInformation)?;
                this.parameter_information =
                    Some(ParameterInformationOld::parse(&mut line_reader, &mut current_comments)?);
                this.mark_section_parsed(SectionOld::ParameterInformation);
            } else if current_line.starts_with(&TokenOld::CurveInformationSection()) {
                this.check_section_not_parsed(&SectionOld::CurveInformation)?;
                this.curve_information = CurveInformationOld::parse(&mut line_reader, &mut current_comments)?;
                this.mark_section_parsed(SectionOld::CurveInformation);
            } else if current_line.starts_with(&TokenOld::AsciiSection()) {
                this.check_section_not_parsed(&SectionOld::AsciiLogData)?;
                this.ascii_log_data = AsciiLogDataOld::parse(
                    &mut line_reader,
                    current_line,
                    &this.curve_information,
                    &mut current_comments,
                )?;
                this.mark_section_parsed(SectionOld::AsciiLogData);
            }
        }

        return Ok(this);
    }

    pub fn new(
        version_information: VersionInformationOld,
        well_information: WellInformationOld,
        curve_information: CurveInformationOld,
        ascii_log_data: AsciiLogDataOld,
        other_information: Option<OtherInformationOld>,
        parameter_information: Option<ParameterInformationOld>,
    ) -> Self {
        // The index is the order each section will be reconstructed in.
        let mut map = HashMap::from([
            (SectionOld::VersionInformation, 0),
            (SectionOld::WellInformation, 1),
            (SectionOld::CurveInformation, 2),
        ]);

        if parameter_information.is_some() {
            map.insert(SectionOld::ParameterInformation, map.len());
        }
        if other_information.is_some() {
            map.insert(SectionOld::OtherInformation, map.len());
        }

        map.insert(SectionOld::AsciiLogData, map.len());

        let mut ald = ascii_log_data;
        if ald.data[0].name.is_empty() {
            ald.has_column_names = false;
        }

        return Self {
            version_information,
            well_information,
            curve_information,
            other_information,
            parameter_information,
            ascii_log_data: ald,
            parsed_sections: map,
        };
    }

    pub fn to_json_str(&self) -> Result<String, LibLasErrorOld> {
        return serde_json::to_string_pretty(self).map_err(|e| return ConvertingToJson(e.to_string()));
    }

    // - Convert this structure back into .las format
    // - We reconstruct the .las file in the same order we parsed it.
    // - If this las file was programmatically created, we use the order in the `fn new`.
    pub fn to_las_str(&self) -> String {
        let mut order: Vec<String> = vec![String::new(); SectionOld::COUNT];

        if let Some(&index) = self.parsed_sections.get(&SectionOld::VersionInformation) {
            order[index] = self.version_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&SectionOld::WellInformation) {
            order[index] = self.well_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&SectionOld::CurveInformation) {
            order[index] = self.curve_information.to_str();
        }
        if let Some(&index) = self.parsed_sections.get(&SectionOld::ParameterInformation)
            && let Some(param_info) = &self.parameter_information
            && let Some(param_info_str) = param_info.to_str()
        {
            order[index] = param_info_str;
        }
        if let Some(&index) = self.parsed_sections.get(&SectionOld::OtherInformation)
            && let Some(other_info) = &self.other_information
            && let Some(other_info_str) = other_info.to_str()
        {
            order[index] = other_info_str;
        }
        if let Some(&index) = self.parsed_sections.get(&SectionOld::AsciiLogData) {
            order[index] = self.ascii_log_data.to_str();
        }

        return order.join("\n");
    }

    fn check_section_not_parsed(&self, section: &SectionOld) -> Result<(), LibLasErrorOld> {
        if self.parsed_sections.contains_key(section) {
            return Err(DuplicateSectionFound(format!("{section:?}")));
        }
        return Ok(());
    }

    fn mark_section_parsed(&mut self, section: SectionOld) {
        let index = self.parsed_sections.len();
        self.parsed_sections.insert(section, index);
    }

    // Consumes all comment lines up until the next line (which is viewed by peeking) isn't a comment.
    fn parse_comments(&self, line_reader: &mut PeekableFileReader) -> Result<Vec<String>, LibLasErrorOld> {
        let mut comments: Vec<String> = vec![];
        while let Some(Ok(peeked_line)) = line_reader.peek() {
            if !peeked_line.trim().to_string().starts_with(&TokenOld::Comment()) {
                break;
            }
            let next_line = line_reader.next().ok_or(ReadingNextLine)??.trim().to_string();
            comments.push(next_line);
        }
        return Ok(comments);
    }
}
