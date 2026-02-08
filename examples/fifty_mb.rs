use liblas::LasFile;

//
// Can parse a 50mb las file, and write it to a json file, in 0.4 seconds!
//
// `cargo run --release --example fifty_mb`
//

fn main() {
    let file_name = "big.las";
    let file_path = format!("las_files/{}", file_name);
    let out_path = format!("exported_las/___{}.json", file_name);
    LasFile::parse_into_json(&file_path, &out_path).unwrap();
}
