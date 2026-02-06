mod data_line;
mod float;
mod parser;
mod section;
mod section_entry;
mod section_header;
mod section_kind;
mod value;

pub use data_line::*;
pub use float::*;
pub use value::*;

pub(crate) use parser::*;
pub(crate) use section::*;
pub(crate) use section_entry::*;
pub(crate) use section_header::*;
pub(crate) use section_kind::*;

const REQUIRED_SECTIONS: [SectionKind; 4] = [
    SectionKind::Version,
    SectionKind::Well,
    SectionKind::Curve,
    SectionKind::AsciiLogData,
];

fn str_contains(str: &str, chars: &[char]) -> Vec<char> {
    let mut matches = vec![];
    for &c in chars {
        if str.contains(c) {
            matches.push(c);
        }
    }
    matches
}

#[derive(Debug)]
pub(crate) struct ParsedLasFile {
    pub sections: Vec<Section>,
}

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}
