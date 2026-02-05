mod helpers;

use super::*;
use helpers::*;
use plotters::prelude::*;

const _MISSING_VERSION_SECTION: &str = "las_files/missing_version_section.las";
const _DUPLICATE_WELL_SECTIONS: &str = "las_files/duplicate_well_sections.las";
const _MISSING_REQUIRED_WELL_DATA: &str = "las_files/missing_required_well_info.las";
const _NO_FIRST_SPACE_AFTER_FIRST_DOT: &str = "las_files/no_first_space_after_first_dot.las";

// To export as YAML run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/gooddd.yaml --out-type yaml --force`

// To export as JSON run:
// `cargo run -- --las las_files/_good_sample_1.las --out exported_las/gooddd.json --out-type json --force`

#[test]
fn test_parsed_file_to_las_file() {
    let file_path = "las_files/_good_sample_1.las";
    let _parsed = parse_las_file(open_file(file_path)).unwrap();
}

#[test]
fn test_good_sample() {
    let file_path = "las_files/_good_sample_1.las";
    let _parsed = parse_las_file(open_file(file_path)).unwrap();
    _print_parsed_las_file(&_parsed);
}

#[test]
#[should_panic]
fn test_num_curves_not_equal_num_ascii_logs() {
    let file_path = "las_files/num_curves_not_equal_num_ascii_logs.las";
    let _parsed = parse_las_file(open_file(file_path)).unwrap();
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

#[test]
fn test_json_deserialization() {
    let file_path = "las_files/_good_sample_1.las";
    let parsed = parse_las_file(open_file(file_path)).unwrap();
    let mut las_file = LasFile::try_from(parsed).expect("parsed");
    let json_str = las_file.to_json_str().expect("json");
    let back_to_las_file = LasFile::try_from_json_str(&json_str).expect("deserialize");
    assert_eq!(las_file, back_to_las_file,);
}

#[test]
#[ignore = "for displaying raw las"]
// run with 'cargo nextest run test_to_las_str --lib --nocapture --run-ignored=only'
fn test_to_las_str() {
    let file_path = "las_files/_good_sample_1.las";
    let parsed = parse_las_file(open_file(file_path)).unwrap();
    let las_file = LasFile::try_from(parsed).expect("las file");
    println!("{las_file}");
}

#[test]
#[ignore = "for generating plots"]
// run with 'cargo nextest run test_plotting --lib --nocapture --run-ignored=only'
fn test_plotting() {
    let file_name = "00-01-01-073-05W5-0.las";
    let file_path = &format!("las_files/{file_name}");
    let output_plot_png = &format!("./plots/{file_name}.png");
    let _parsed = parse_las_file(open_file(file_path)).unwrap();
    let las_file = LasFile::try_from(_parsed).unwrap();
    plot_las(&las_file, output_plot_png, 5).expect("plot");
}
