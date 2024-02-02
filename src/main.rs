//TODO: Consider whether the file is not hidden
//TODO: Use Logging system
//TODO: Loggin of time listing directory
use chrono::{DateTime, Datelike, Local};
use clap::{App, Arg, ArgMatches};
use rand::Rng;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod errors;
use errors::AppError;

#[derive(Debug)]
pub enum FileOperation {
  Copy,
  Rename,
}

impl FileOperation {
  pub fn execute(&self, path: &str, output_file: &str) -> Result<(), std::io::Error> {
    match self {
      FileOperation::Copy => {
        fs::copy(path, output_file)?;
        Ok(())
      }
      FileOperation::Rename => {
        fs::rename(path, output_file)?;
        Ok(())
      }
    }
  }
}

static mut FILE_OPERATION: FileOperation = FileOperation::Copy;

fn main() -> Result<(), AppError> {
  let matches = read_args()?;
  let dir_path = matches.value_of("directory").unwrap();
  let outputh_directory = matches.value_of("output").unwrap();

  let paths = list_files_in_directory(dir_path)?;
  let paths_len = paths.len();
  let mut count = 0;
  let mut files_transfered = HashSet::<String>::new();
  for path in paths {
    let modification_year = get_file_modification_date(&path.to_string_lossy())?;

    let output_dir = PathBuf::from(outputh_directory).join(format!("{}", modification_year));
    fs::create_dir_all(&output_dir)?;

    let mut file_name = path.file_name().ok_or_else(|| {
      AppError::IOError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No file name",
      ))
    })?;

    let file_name_str = file_name.to_string_lossy().to_string();

    let new_name_with_random_id: OsString;

    if !files_transfered.insert(file_name_str.clone()) {
      let mut rng = rand::thread_rng();
      let random_id: u32 = rng.gen();
      new_name_with_random_id = OsString::from(format!("{}_{}", random_id, file_name_str));
      file_name = OsStr::new(&new_name_with_random_id);
    }

    let output_file = output_dir.join(file_name);

    unsafe {
      FILE_OPERATION
        .execute(&path.to_string_lossy(), &output_file.to_string_lossy())
        .unwrap();
    }
    count += 1;
    let percent = ((count as f64 / paths_len as f64) * 100.0).round();
    println!(
      "{}% Concluded, File {} of {}  -  File Name: {:?}",
      percent, count, paths_len, file_name
    );
  }

  Ok(())
}

fn get_file_modification_date(path: &str) -> Result<i32, AppError> {
  let path = Path::new(path);

  let modification_date_in_system_time = fs::metadata(path)?.modified()?;
  let modification_date = DateTime::<Local>::from(modification_date_in_system_time);

  let year = modification_date.year();
  Ok(year)
}

fn list_files_in_directory(dir_path: &str) -> Result<Vec<PathBuf>, AppError> {
  let mut file_paths = Vec::new();

  for entry in WalkDir::new(dir_path).follow_links(true) {
    match entry {
      Ok(entry) => {
        if entry.file_type().is_file() {
          file_paths.push(entry.path().to_path_buf());
        }
      }
      Err(err) => {
        let error_message = format!("An error occurred while reading the directory: {}", err);
        return Err(AppError::ReadDirectoryError(error_message));
      }
    }
  }

  Ok(file_paths)
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
