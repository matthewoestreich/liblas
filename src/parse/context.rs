use std::collections::HashMap;

use crate::parse::{SectionKind, state::ParserState};

#[derive(Debug, Default)]
pub(crate) struct ParserContext {
    pub sections: HashMap<SectionKind, usize>,
    pub curve_mnemonics: Vec<String>,
    pub comments: PendingComments,
    pub state: ParserState,
}

#[derive(Debug, Default)]
pub struct PendingComments(Option<Vec<String>>);

impl PendingComments {
    pub fn push(&mut self, c: String) {
        self.0.get_or_insert_with(Vec::new).push(c);
    }

    pub fn take(&mut self) -> Option<Vec<String>> {
        self.0.take()
    }
}
