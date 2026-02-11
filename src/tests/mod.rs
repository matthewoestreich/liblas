mod helpers;

use std::{fs::File, io::BufReader, time::Instant};

use crate::tokenizer::LasTokenizer;

use super::*;
use helpers::*;
use plotters::prelude::*;

const _MISSING_VERSION_SECTION: &str = "las_files/missing_version_section.las";
const _DUPLICATE_WELL_SECTIONS: &str = "las_files/duplicate_well_sections.las";
const _MISSING_REQUIRED_WELL_DATA: &str = "las_files/missing_required_well_info.las";
const _NO_FIRST_SPACE_AFTER_FIRST_DOT: &str = "las_files/no_first_space_after_first_dot.las";

// To export as YAML run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/gooddd.yaml --out-type yaml --force`
// `cargo run -- --las las_files/big.las --out exported_las/big.yaml --out-type yaml --force`

// To export as JSON run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/__good_sample_1.las__cli.json --out-type json --force`
// `cargo run -- --las las_files/big.las --out exported_las/big.json --out-type json --force`

#[test]
fn test_parsed_file_to_las_file() {
    let file_path = "las_files/_good_sample_1.las";
    let _parsed = parse_las_file(file_path).unwrap();
}

#[test]
fn test_good_sample() {
    let file_path = "las_files/_good_sample_1.las";
    let _parsed = super::parse(file_path).unwrap();
}

#[test]
#[should_panic]
fn test_num_curves_not_equal_num_ascii_logs() {
    let file_path = "las_files/num_curves_not_equal_num_ascii_logs.las";
    let _parsed = parse_las_file(file_path).unwrap();
}

#[test]
#[should_panic]
fn test_ascii_section_not_last() {
    let file_path = "las_files/ascii_not_last.las";
    _ = parse_las_file(file_path).unwrap();
}

#[test]
fn test_missing_mnemonic() {
    let file_path = "las_files/missing_mnemonic.las";
    match parse_las_file(file_path) {
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
    _ = parse_las_file(file_path).unwrap();
}

#[test]
fn test_json_deserialization() {
    let file_path = "las_files/_good_sample_1.las";
    let mut las_file = parse_las_file(file_path).unwrap();
    let json_str = las_file.to_json_str().expect("json");
    let back_to_las_file = LasFile::try_from_json_str(&json_str).expect("deserialize");
    assert_eq!(las_file, back_to_las_file,);
}

#[test]
#[ignore]
// run with 'cargo nextest run --release test_large_las_file --lib --nocapture --run-ignored=only'
fn test_large_las_file() {
    let las_file_size_in_mb = 50;
    let large_las_cursor = generate_temp_las(las_file_size_in_mb).unwrap(); // Cursor<Vec<u8>>
    let writer = std::io::sink();
    let start = Instant::now();
    super::parse_from_into(large_las_cursor, writer, OutputFormat::JSON).unwrap();
    let end = start.elapsed();
    println!("parsing large las file ({las_file_size_in_mb}mb) took : {end:?}");
}

#[test]
#[ignore]
// run with 'cargo nextest run test_export_good_yaml --lib --nocapture --run-ignored=only'
fn test_export_good_yaml() {
    let file_name = "_good_sample_1.las";
    let las_file_path = format!("las_files/{}", file_name);
    let mut las_file = super::parse(&las_file_path).unwrap();
    println!("{}", las_file.to_yaml_str().unwrap());
}

#[test]
#[ignore]
// run with 'cargo nextest run test_export_good_json --lib --nocapture --run-ignored=only'
fn test_export_good_json() {
    let file_name = "_good_sample_1.las";
    let las_file_path = format!("las_files/{}", file_name);
    let mut las_file = super::parse(&las_file_path).unwrap();
    println!("{}", las_file.to_json_str().unwrap());
}

#[test]
#[ignore = "for displaying raw las"]
// run with 'cargo nextest run test_to_las_str --lib --nocapture --run-ignored=only'
fn test_to_las_str() {
    let file_path = "las_files/_good_sample_1.las";
    let las_file = parse_las_file(file_path).unwrap();
    println!("{las_file}");
}

#[test]
#[ignore = "for generating plots"]
// run with 'cargo nextest run test_plotting --lib --nocapture --run-ignored=only'
fn test_plotting() {
    let file_name = "00-01-01-073-05W5-0.las";
    let file_path = &format!("las_files/{file_name}");
    let output_plot_png = &format!("./plots/{file_name}.png");
    let las_file = parse_las_file(file_path).unwrap();
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
