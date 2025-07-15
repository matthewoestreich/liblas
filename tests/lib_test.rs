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


//#[test]
// MAKE A TEST FOR THIS - first column in ascii data -
// From the LAS specification : "The index curve (i.e. first curve) must be depth, time or index.
// The only valid mnemonics for the index channel are DEPT, DEPTH, TIME, or INDEX."