use clap::Parser;
use liblas::*;
use std::{
  fs::{OpenOptions, create_dir_all},
  io::Write,
  path::PathBuf,
  process::exit,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Relative to binary location.
  /// Path to .las file.
  #[arg(short, long, required = true)]
  las: String,

  /// Relative to binary location.
  /// Output path with file name ending in .json.
  /// Only new files will be automatically created!
  /// If the path contains non-existent directories, you will need to use the '--force' switch.
  #[arg(short, long, required = true)]
  out: String,

  /// Will create directories within '--out' path if they do not exist.
  /// If file already exists we will overwrite it.
  #[arg(short, long)]
  force: bool,
}

fn create_file_path(path: PathBuf) {
  let mut p = path;
  p.pop();
  let _ = create_dir_all(&p);
}

fn main() {
  let args = Args::parse();
  if !args.las.ends_with(".las") {
    println!("Error : '--las' path '{}' must be to a .las file!", args.las);
    exit(1);
  }
  if !args.out.ends_with(".json") {
    println!("Error : '--out' path '{}' must be to a .json file!", args.out);
    exit(1);
  }

  let las = LasFile::parse(args.las.clone().into()).unwrap_or_else(|e| {
    println!("Error parsing .las file : {e}");
    exit(1);
  });

  let las_json = las.to_json_str().unwrap_or_else(|e| {
    println!("Error converting .las file to .json : {e}");
    exit(1);
  });

  let mut file_options = OpenOptions::new();
  file_options.write(true);

  if args.force {
    create_file_path(args.out.clone().into());
    file_options.truncate(true); // Truncate the file if it exists, overwriting its contents
    file_options.create(true);
  } else {
    file_options.create_new(true); // Only create if it is a new file
  }

  let mut file = file_options.open(&args.out).unwrap_or_else(|e| {
    println!("Error creating or opening '--out' file : {e}\nYou may need to use the '--force' switch to:\n - Create non-existent directory (or directories) within '--out' path\n - Overwrite existing .json file specified in '--out' path");
    exit(1);
  });

  file.write_all(las_json.as_bytes()).unwrap_or_else(|e| {
    println!("Error writing to '--out' file : {e}");
    exit(1);
  });

  println!("Success! Exported JSON file to '{}'", args.out);
}
