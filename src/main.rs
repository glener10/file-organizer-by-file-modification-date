use clap::{App, Arg, ArgMatches};

mod errors;
use errors::AppError;

mod usecases;
use usecases::operation::FileOperation;
use usecases::operation::FILE_OPERATION;
use usecases::organize::organize_files;

fn main() -> Result<(), AppError> {
  let matches = read_args()?;
  let dir_path = matches.value_of("directory").unwrap();
  let outputh_directory = matches.value_of("output").unwrap();

  organize_files(dir_path, outputh_directory)
}

fn read_args<'a>() -> Result<ArgMatches<'a>, AppError> {
  let matches = App::new("Files Organizer Per Year")
    .version("1.0")
    .author("Glener Pizzolato")
    .about("Organizes files by modification year")
    .arg(
      Arg::with_name("directory")
        .short("d")
        .long("directory")
        .help("Path to input directory")
        .value_name("INPUT_DIRECTORY")
        .required(true),
    )
    .arg(
      Arg::with_name("cut")
        .short("x")
        .long("cut")
        .help("Cut files instead of copying"),
    )
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("OUTPUT_DIRECTORY")
        .help("Output directory")
        .default_value("output"),
    )
    .get_matches();

  if matches.is_present("cut") {
    unsafe {
      FILE_OPERATION = FileOperation::Rename;
    }
  }
  Ok(matches)
}
