fn main() {
    let las_file = liblas::parse("las_files/_good_sample_1.las").unwrap();
    println!("{:?}", las_file.version_information.additional);
}
