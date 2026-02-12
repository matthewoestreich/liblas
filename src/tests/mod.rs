mod helpers;

use super::*;
use crate::tokenizer::LasTokenizer;
use helpers::*;
use plotters::prelude::*;
use std::{fs::File, io::BufReader, time::Instant};

// To export as YAML run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/gooddd.yaml --out-type yaml --force`
// `cargo run -- --las las_files/big.las --out exported_las/big.yaml --out-type yaml --force`

// To export as JSON run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/__good_sample_1.las__cli.json --out-type json --force`
// `cargo run -- --las las_files/big.las --out exported_las/big.json --out-type json --force`

#[test]
fn test_parsed_file_to_las_file() {
    let file_path = "las_files/_good_sample_1.las";
    _ = parse(file_path).unwrap();
}

#[test]
fn test_good_sample() {
    let file_path = "las_files/_good_sample_1.las";
    _ = parse(file_path).unwrap();
}

#[test]
fn test_version_info_not_first() {
    let file_path = "las_files/version_info_not_first.las";
    match parse(file_path) {
        Err(ParseError::VersionInformationNotFirst { .. }) => {} // noop
        Ok(_) => panic!("Expected ParseError::VersionInformationNotFirst error but got Ok"),
        Err(e) => panic!("Expected ParseError::VersionInformationNotFirst error but got {e:?}"),
    }
}

#[test]
fn test_ascii_data_row_has_incorrect_length() {
    let file_path = "las_files/ascii_data_row_incorrect_length.las";
    match parse(file_path) {
        Err(ParseError::AsciiColumnsMismatch { .. }) => {} // noop
        Ok(_) => panic!("Expected ParseError::AsciiColumnsMismatch error but got Ok"),
        Err(e) => panic!("Expected ParseError::AsciiColumnsMismatch error but got {e:?}"),
    }
}

#[test]
fn test_duplicate_well_section() {
    let file_path = "las_files/duplicate_well_sections.las";
    match parse(file_path) {
        Err(ParseError::DuplicateSection { section, .. }) => {
            let expected_section = SectionKind::Well;
            if section != expected_section {
                panic!("Got correct error but wrong section! Got {section:?} wanted {expected_section:?}");
            }
        }
        Ok(_) => panic!("Expected ParseError::DuplicateSection error but got Ok"),
        Err(e) => panic!("Expected ParseError::DuplicateSection error but got {e:?}"),
    }
}

#[test]
fn test_missing_required_well_info() {
    let file_path = "las_files/missing_required_well_info.las";
    match parse(file_path) {
        Err(ParseError::WellDataMissingRequiredValueForMnemonic { mnemonic }) => {
            let expected_mnemonic = "STRT";
            if mnemonic != expected_mnemonic {
                panic!("Got correct error but wrong mnemonic! Got {mnemonic} wanted {expected_mnemonic}");
            }
        }
        Ok(_) => panic!("Expected ParseError::WellDataMissingRequiredValueForMnemonic error but got Ok"),
        Err(e) => panic!("Expected ParseError::WellDataMissingRequiredValueForMnemonic error but got {e:?}"),
    }
}

#[test]
fn test_num_curves_not_equal_num_ascii_logs() {
    let file_path = "las_files/num_curves_not_equal_num_ascii_logs.las";
    match parse(file_path) {
        Err(ParseError::AsciiColumnsMismatch { .. }) => {} // noop
        Ok(_) => panic!("Expected ParseError::AsciiColumnsMismatch error but got Ok"),
        Err(e) => panic!("Expected ParseError::AsciiColumnsMismatch error but got {e:?}"),
    }
}

#[test]
fn test_ascii_section_not_last() {
    let file_path = "las_files/ascii_not_last.las";
    match parse(file_path) {
        Err(ParseError::AsciiLogDataSectionNotLast { .. }) => {} // noop
        Ok(_) => panic!("Expected ParseError::AsciiLogDataSectionNotLast error but got Ok"),
        Err(e) => panic!("Expected ParseError::AsciiLogDataSectionNotLast error but got {e:?}"),
    }
}

#[test]
fn test_missing_mnemonic() {
    let file_path = "las_files/missing_mnemonic.las";
    match parse(file_path) {
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
    _ = parse(file_path).unwrap();
}

#[test]
fn test_json_deserialization() {
    let file_path = "las_files/_good_sample_1.las";
    let mut las_file = parse(file_path).unwrap();
    let json_str = las_file.to_json_str().expect("json");
    let back_to_las_file = LasFile::try_from_json_str(&json_str).expect("deserialize");
    assert_eq!(las_file, back_to_las_file,);
}

#[test]
fn test_yaml_deserialization() {
    let file_path = "las_files/_good_sample_1.las";
    let mut las_file = parse(file_path).unwrap();
    let yaml_str = las_file.to_yaml_str().expect("json");
    let back_to_las_file = LasFile::try_from_yaml_str(&yaml_str).expect("deserialize");
    assert_eq!(las_file, back_to_las_file,);
}

#[test]
#[ignore]
//
// run with 'cargo nextest run --release test_large_las_file --lib --nocapture --run-ignored=only'
//
// We generate a .las file in RAM, of the specified size, and directly parse from there into an empty sink.
//
fn test_large_las_file() {
    // CHANGE THIS TO MAKE THE LAS FILE BIGGER
    let las_file_size_in_mb = 50;
    let reader = generate_temp_las(las_file_size_in_mb).unwrap();
    let writer = std::io::sink();
    let start = Instant::now();
    parse_from_into(reader, writer, OutputFormat::JSON).unwrap();
    let end = start.elapsed();
    println!("parsing large las file ({las_file_size_in_mb}mb) took : {end:?}");
}

#[test]
#[ignore]
// run with 'cargo nextest run test_export_good_yaml --lib --nocapture --run-ignored=only'
fn test_export_good_yaml() {
    let file_name = "_good_sample_1.las";
    let las_file_path = format!("las_files/{}", file_name);
    let mut las_file = parse(&las_file_path).unwrap();
    println!("{}", las_file.to_yaml_str().unwrap());
}

#[test]
#[ignore]
// run with 'cargo nextest run test_export_good_json --lib --nocapture --run-ignored=only'
fn test_export_good_json() {
    let file_name = "_good_sample_1.las";
    let las_file_path = format!("las_files/{}", file_name);
    let mut las_file = parse(&las_file_path).unwrap();
    println!("{}", las_file.to_json_str().unwrap());
}

#[test]
#[ignore = "for displaying raw las"]
// run with 'cargo nextest run test_to_las_str --lib --nocapture --run-ignored=only'
fn test_to_las_str() {
    let file_path = "las_files/_good_sample_1.las";
    let las_file = parse(file_path).unwrap();
    println!("{las_file}");
}

#[test]
#[ignore = "for generating plots"]
// run with 'cargo nextest run test_plotting --lib --nocapture --run-ignored=only'
fn test_plotting() {
    let file_name = "00-01-01-073-05W5-0.las";
    let file_path = &format!("las_files/{file_name}");
    let output_plot_png = &format!("./plots/{file_name}.png");
    let las_file = parse(file_path).unwrap();
    plot_las(&las_file, output_plot_png, 5).expect("plot");
}

#[test]
#[ignore]
// run with 'cargo nextest run test_good_sample_stream_json --lib --nocapture --run-ignored=only'
fn test_good_sample_stream_json() {
    let file_name = "_good_sample_1.las";
    let file_path = format!("las_files/{}", file_name);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let out_path = format!("exported_las/{}.json", file_name);
    let out_file = File::create(out_path).unwrap();
    let writer = std::io::BufWriter::new(out_file);
    let mut sink = JsonSink::new(writer);

    //let stdout = std::io::stdout();
    //let handle = stdout.lock();
    //let mut sink = JsonSink::new(handle);

    let tokenizer = LasTokenizer::new(reader);
    let mut parser = LasParser::new(tokenizer);
    parser.parse_into(&mut sink).unwrap();
}

#[test]
#[ignore]
// run with 'cargo nextest run test_good_sample_stream_yaml --lib --nocapture --run-ignored=only'
fn test_good_sample_stream_yaml() {
    let file_name = "_good_sample_1.las";
    let file_path = format!("las_files/{}", file_name);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let out_path = format!("exported_las/{}.yaml", file_name);
    let out_file = File::create(out_path).unwrap();
    let writer = std::io::BufWriter::new(out_file);
    let mut sink = YamlSink::new(writer);

    //let stdout = std::io::stdout();
    //let handle = stdout.lock();
    //let mut sink = JsonSink::new(handle);

    let tokenizer = LasTokenizer::new(reader);
    let mut parser = LasParser::new(tokenizer);
    parser.parse_into(&mut sink).unwrap();
}
