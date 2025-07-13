mod ascii_data;
mod curve_information;
mod errors;
mod las_file;
mod mnemonic;
mod other_information;
mod parameter_information;
mod version_information;
mod well_information;

pub use errors::*;

pub use ascii_data::*;
pub use curve_information::*;
pub use las_file::*;
pub use mnemonic::*;
pub use other_information::*;
pub use parameter_information::*;
pub use version_information::*;
pub use well_information::*;

#[derive(PartialEq)]
pub enum Section {
  Version,
  Well,
  Ascii,
  Option,
  Curve,
  None,
}
