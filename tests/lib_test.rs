use liblas::*;

//#[test]
//fn test_minified_las() -> Result<(), LibLasError> {
//  let las_result = LasFile::parse("tests/test_las_files/minified.las".into());
//  let las = las_result?;
//  //let json = las.to_json_str()?;
//  println!("{:?}", las.ascii_log_data.columns.iter().find(|e| e.name == "DEPT"));
//  return Ok(());
//}

#[test]
fn test_json_str() -> Result<(), LibLasError> {
  let las_result = LasFile::parse("tests/test_las_files/good_sample_1.las".into());
  //assert!(las_result.is_ok());
  let las = las_result?;
  let json_result = las.to_json_str();
  if json_result.is_err() {
    let e = format!("{:?}", json_result.err());
    println!("{e}");
    return Err(LibLasError::GeneralError(e));
  }
  //let json = json_result?;
  //println!("{json}");
  return Ok(());
}

#[test]
fn comments_at_start() -> Result<(), LibLasError> {
  let las = LasFile::parse("tests/las/comments_at_start_of_file.las".into())?;
  let json = las.to_json_str()?;
  println!("{json}");
  return Ok(());
}

#[test]
fn version_info_not_first() -> Result<(), LibLasError> {
  let las = LasFile::parse("tests/las/version_info_not_first.las".into())?;
  let json = las.to_json_str()?;
  println!("{json}");
  return Ok(());
}

//#[test]
//fn test_misc() -> Result<(), LibLasError> {
//  let las_result = LasFile::parse("test_las_files/ascii_not_last.las".into());
//  assert!(las_result.is_err());
//  return Ok(());
//}

//#[test]
// MAKE A TEST FOR THIS - first column in ascii data -
// From the LAS specification : "The index curve (i.e. first curve) must be depth, time or index.
// The only valid mnemonics for the index channel are DEPT, DEPTH, TIME, or INDEX."
