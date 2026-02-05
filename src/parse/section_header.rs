use crate::parse::SectionKind;

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct SectionHeader {
    pub raw: String,
    pub kind: SectionKind,
}

#[allow(dead_code)]
impl SectionHeader {
    pub fn new(name: String, kind: SectionKind) -> Self {
        Self { raw: name, kind }
    }
}
