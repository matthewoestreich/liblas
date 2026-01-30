#![allow(clippy::implicit_return)]
pub mod errors;
pub mod parser;
pub mod tokenizer;
pub mod types;

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use super::*;

    #[test]
    fn testing() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/las/_good_sample_1.las");
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
