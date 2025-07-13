use std::error::Error;

use liblas::*;

fn main() -> Result<(), Box<dyn Error>> {
  let las_file_path = "sample_las_files/AEP_Pol_PES_BL_5H_RCBL-modified.las";
  let mut las = LasFile::new(las_file_path.into());
  las.parse()?;
  dbg!(las);
  return Ok(());
}
