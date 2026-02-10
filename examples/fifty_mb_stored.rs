use std::{
    thread,
    time::{Duration, Instant},
};

//
// Can parse a 50mb las file, and write it to a json file, in ~500ms!
//
// `cargo run --release --example fifty_mb`
//

fn main() {
    thread::sleep(Duration::from_secs(5));
    let main_start = Instant::now();
    for _ in 0..10 {
        let absolute_path_to_repo = "/Users/matthew/Documents/GitHub/_Rust/liblas";
        let file_name = "big.las";
        let file_path = format!("{}/las_files/{}", absolute_path_to_repo, file_name);
        let start = Instant::now();
        let _las_file = liblas::parse(&file_path).unwrap();
        let elapsed = start.elapsed();
        println!("parsed and wrote {file_name} in {elapsed:?}");
    }
    let main_elapsed = main_start.elapsed();
    println!("Program finished in {main_elapsed:?}");
}
