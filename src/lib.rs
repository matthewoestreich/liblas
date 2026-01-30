pub mod errors;
pub mod parser;
pub mod section;
pub mod tokenizer;

use section::Section;

#[derive(Debug)]
pub struct LasFile {
    pub sections: Vec<Section>,
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::*;

    const _GOOD_SAMPLE_1: &str = "las_files/_good_sample_1.las";
    const _MISSING_VERSION_SECTION: &str = "las_files/missing_version_section.las";
    const _ASCII_NOT_LAST: &str = "las_files/ascii_not_last.las";
    const _DUPLICATE_WELL_SECTIONS: &str = "las_files/duplicate_well_sections.las";

    #[test]
    fn testing() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(_GOOD_SAMPLE_1);
        let file = File::open(file_path).expect("open file error");
        let reader = BufReader::new(file);

        let las_tokenizer = tokenizer::LasTokenizer::new(reader);
        let mut las_parser = parser::LasParser::new(las_tokenizer);
        let parsed_file = las_parser.parse().expect("no parse error");

        for section in parsed_file.sections {
            println!("{}", section.header.raw);
            for entry in section.entries {
                println!("\t{entry:?}");
            }
        }

        //while let Ok(token_opt) = las_tokenizer.next_token()
        //    && let Some(token) = token_opt
        //{
        //    println!("{token:?}");
        //}
    }
}
