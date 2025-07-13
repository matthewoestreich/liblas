use std::error::Error;

use liblas::*;

fn main() -> Result<(), Box<dyn Error>> {
  let las_file_path = "sample_las_files/sample_1.las";
  let mut las = LasFile::new(las_file_path.into());
  las.parse()?;
  let json = las.to_json_str()?;
  println!("{json}");
  return Ok(());
}
