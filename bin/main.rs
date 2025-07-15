use std::fs;

use liblas::*;

fn main() -> Result<(), LibLasError> {
  let file_name = "5070-14-SESW-mod";
  let las_file_path = format!("tests/las/{file_name}.las");
  let las = LasFile::parse(las_file_path.into())?;
  //las.parse()?;
  let json = las.to_json_str()?;
  let output_path = format!("json_las/{file_name}.json");
  fs::write(output_path, json)?;
  return Ok(());
}
