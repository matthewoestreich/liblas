use crate::parse::KeyValueData;

#[derive(Debug)]
pub(crate) enum SectionEntry {
    Delimited(KeyValueData),
    Raw {
        text: String,
        comments: Option<Vec<String>>,
    },
}
