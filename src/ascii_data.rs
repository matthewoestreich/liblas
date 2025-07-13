/*
~A (ASCII Log Data)
• The data section will always be the last section in a file.
• Only one "~A" section can occur in an LAS 2.0 file.
• Embedded blank lines anywhere in the section are forbidden
• Each column of data must be separated by at least one space. Consistency of format on every
line, while not required, is expected by many LAS readers. Right Justification of each column of
data and the same width of all data fields is highly recommended.
• Line length in the data section of unwrapped files are no longer restricted
• In wrap mode, the index channel will be on its own line
• In wrap mode, a line of data will be no longer than 80 characters. This includes a carriage return
and line feed
*/

#[derive(Default, Debug)]
pub struct AsciiColumn {
  pub name: String,
  pub data: Vec<f64>,
}

#[derive(Default, Debug)]
pub struct AsciiLogData {
  pub columns: Vec<AsciiColumn>,
}
