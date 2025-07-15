use liblas::*;

#[test]
fn test_minified_las() -> Result<(), LibLasError> {
  let las_result = LasFile::parse("test_las_files/minified.las".into());
  let las = las_result?;
  //let json = las.to_json_str()?;
  println!("{:?}", las.ascii_log_data.columns.iter().find(|e| e.name == "DEPT"));
  return Ok(());
}

//#[test]
//fn test_misc() -> Result<(), LibLasError> {
//  let las_result = LasFile::parse("test_las_files/ascii_not_last.las".into());
//  assert!(las_result.is_err());
//  return Ok(());
//}
