use liblas::*;

fn main() -> Result<(), LibLasError> {
  let las_file_path = "test_las_files/minified.las";
  let las = LasFile::parse(las_file_path.into())?;
  //las.parse()?;
  let json = las.to_json_str()?;
  println!("{json}");
  return Ok(());
}
