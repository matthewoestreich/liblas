use crate::parse::SectionKind;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum ParserState {
    #[default]
    Start,
    In(SectionKind),
}
