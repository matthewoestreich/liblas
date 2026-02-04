mod parser;
mod section;

pub(crate) use parser::*;
pub(crate) use section::*;

#[derive(Debug)]
pub(crate) struct ParsedLasFile {
    pub sections: Vec<Section>,
}

const REQUIRED_SECTIONS: [SectionKind; 4] = [
    SectionKind::Version,
    SectionKind::Well,
    SectionKind::Curve,
    SectionKind::AsciiLogData,
];

#[derive(Debug, PartialEq, Eq)]
enum ParserState {
    Start,
    Working,
    // We set to end before parsing ASCII log data. Since it HAS to be the last section in a las file.
    End,
}
