pub mod errors;
pub mod parser;
pub mod tokenizer;

use parser::Section;

#[derive(Debug)]
pub struct LasFile {
    pub sections: Vec<Section>,
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader, path::PathBuf};

    use crate::errors::ParseError;

    use super::*;

    const _MISSING_VERSION_SECTION: &str = "las_files/missing_version_section.las";
    const _DUPLICATE_WELL_SECTIONS: &str = "las_files/duplicate_well_sections.las";
    const _MISSING_REQUIRED_WELL_DATA: &str = "las_files/missing_required_well_info.las";
    const _NO_FIRST_SPACE_AFTER_FIRST_DOT: &str = "las_files/no_first_space_after_first_dot.las";

    fn open_file(file_path: &str) -> BufReader<File> {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file_path);
        let file = File::open(file_path).expect("open file error");
        BufReader::new(file)
    }

    fn parse_las_file(reader: BufReader<File>) -> Result<LasFile, ParseError> {
        let las_tokenizer = tokenizer::LasTokenizer::new(reader);
        let mut las_parser = parser::LasParser::new(las_tokenizer);
        las_parser.parse()
    }

    #[test]
    fn test_good_sample() {
        let file_path = "las_files/_good_sample_1.las";
        _ = parse_las_file(open_file(file_path)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ascii_section_not_last() {
        let file_path = "las_files/ascii_not_last.las";
        _ = parse_las_file(open_file(file_path)).unwrap();
    }

    #[test]
    fn test_missing_mnemonic() {
        let file_path = "las_files/missing_mnemonic.las";
        match parse_las_file(open_file(file_path)) {
            Err(ParseError::MissingRequiredKey { key, .. }) => {
                let expected_key = "mnemonic";
                if key != expected_key {
                    panic!("Got correct error but incorrect key! Expected '{expected_key}' got '{key}'");
                }
            }
            Ok(_) => panic!("Expected MissingRequiredKey error but got Ok"),
            Err(e) => panic!("Expeccted error MissingRequiredKey but got {e:?}"),
        }
    }

    #[test]
    fn test_no_space_before_last_colon() {
        let file_path = "las_files/no_space_before_last_colon.las";
        _ = parse_las_file(open_file(file_path)).unwrap();
    }

    // Helper - put at bottom to not take up space
    fn _print_parsed_las_file(parsed_file: &LasFile) {
        for section in &parsed_file.sections {
            println!("{:?}", section.header.kind);
            for entry in &section.entries {
                println!("\t{entry:?}");
            }

            let Some(headers) = &section.ascii_headers else {
                continue;
            };

            if !section.ascii_rows.is_empty() {
                let n_cols = headers.len();
                let mut col_widths = vec![0; n_cols];

                // Convert rows to strings with fixed decimal precision
                let string_table: Vec<Vec<String>> = section
                    .ascii_rows
                    .iter()
                    .map(|row| row.iter().map(|v| format!("{:.3}", v)).collect())
                    .collect();

                // First, compute max width per column, considering headers too
                for (i, h) in headers.iter().enumerate() {
                    col_widths[i] = h.len();
                }
                for row in &string_table {
                    for (i, cell) in row.iter().enumerate() {
                        col_widths[i] = col_widths[i].max(cell.len() + 3);
                    }
                }

                // Print headers
                print!("\t");
                for (i, h) in headers.iter().enumerate() {
                    print!("{:<width$} ", h, width = col_widths[i]); // Left-align headers
                }
                println!();

                // Print rows
                for row in &string_table {
                    print!("\t");
                    for (i, cell) in row.iter().enumerate() {
                        print!("{:<width$} ", cell, width = col_widths[i]);
                    }
                    println!();
                }
            }
        }
    }
}
