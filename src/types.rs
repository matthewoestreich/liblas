use std::{
  fs::File,
  io::{BufReader, Lines},
  iter::Peekable,
};

pub type PeekableFileReader = Peekable<Lines<BufReader<File>>>;

#[derive(PartialEq)]
pub enum Section {
  Version,
  Well,
  Ascii,
  Option,
  Curve,
  None,
}
