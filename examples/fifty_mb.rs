use std::{fs::File, time::Instant};

//
// Can parse a 50mb las file, and write it to a json file, in ~500ms!
//
// `cargo run --release --example fifty_mb`
//

fn main() {
    let main_start = Instant::now();
    let absolute_path_to_repo = "/Users/matthew/Documents/GitHub/_Rust/liblas";
    let file_name = "big.las";
    let file_path = format!("{}/las_files/{}", absolute_path_to_repo, file_name);
    let out_path = format!("{}/exported_las/___{}.json", absolute_path_to_repo, file_name);
    let out_file = File::create(out_path.clone()).unwrap();
    println!("INPUT : {}\nOUTPUT : {}", file_path, out_path.clone());
    let writer = std::io::BufWriter::new(out_file);
    let start = Instant::now();
    liblas::parse_into(&file_path, writer, liblas::OutputFormat::JSON).unwrap();
    let elapsed = start.elapsed();
    println!("parsed and wrote {file_name} in {elapsed:?}");
    let main_elapsed = main_start.elapsed();
    println!("Program finished in {main_elapsed:?}");
}
