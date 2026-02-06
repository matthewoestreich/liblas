use crate::parse::DataLine;

#[derive(Debug)]
pub(crate) enum SectionEntry {
    Delimited(DataLine),
    Raw {
        text: String,
        comments: Option<Vec<String>>,
    },
}
