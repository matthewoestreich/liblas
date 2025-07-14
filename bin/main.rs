use liblas::*;

fn main() -> Result<(), LibLasError> {
  let las_file_path = "test_las_files/minified.las";
  let mut las = LasFile::new(las_file_path.into());
  las.parse()?;
  let json = las.to_json_str()?;
  println!("{json}");
  return Ok(());
}
