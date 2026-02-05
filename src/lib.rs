#[cfg(test)]
mod tests;

mod errors;
mod las_file;

pub(crate) mod parse;
pub(crate) mod tokenizer;

pub mod sections;
pub use errors::*;
pub use las_file::*;

use crate::parse::*;
use std::fmt;

pub(crate) fn any_present<T>(items: &[&Option<T>]) -> bool {
    items.iter().any(|o| o.is_some())
}

pub(crate) fn write_kv_opt(f: &mut fmt::Formatter<'_>, kv: &Option<KeyValueData>) -> fmt::Result {
    if let Some(v) = kv {
        writeln!(f, "{v}")?;
    }
    Ok(())
}

pub(crate) fn write_comments(f: &mut fmt::Formatter<'_>, comments: &Option<Vec<String>>) -> fmt::Result {
    if let Some(cs) = comments {
        for c in cs {
            let fc = format!("# {c}").trim().to_string();
            writeln!(f, "{fc}")?;
        }
    }
    Ok(())
}
