use std::{fs::File, time::Instant};

//
// Can parse a 50mb las file, and write it to a json file, in ~500ms!
//
// `cargo run --release --example fifty_mb`
//

fn main() {
    let file_name = "big.las";
    let file_path = format!("las_files/{}", file_name);
    let out_path = format!("exported_las/___{}.json", file_name);
    let out_file = File::create(out_path).unwrap();
    let start = Instant::now();
    liblas::parse_into(&file_path, out_file, liblas::OutputFormat::JSON).unwrap();
    let elapsed = start.elapsed();
    println!("parsed and wrote {file_name} in {elapsed:?}");
}
