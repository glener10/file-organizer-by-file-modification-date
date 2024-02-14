use chrono::{DateTime, Datelike, Local};
use clap::{App, Arg, ArgMatches};
use rand::Rng;
use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
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

struct ExtensionCounter {
  extension: String,
  count: usize,
}

fn main() -> Result<(), AppError> {
  let matches = read_args()?;
  let dir_path = matches.value_of("directory").unwrap();
  let outputh_directory = matches.value_of("output").unwrap();

  let paths = list_files_in_directory(dir_path)?;
  let paths_len = paths.len();

  let mut count_files = 0;
  let mut count_files_with_same_name = 0;

  let mut files_transfered = HashSet::<String>::new();
  let mut files_with_repeat_name: Vec<String> = Vec::new();
  let mut extension_counters: Vec<ExtensionCounter> = Vec::new();

  println!("Organizing Files...");
  for path in paths {
    let file_extension = get_file_extension(&path.to_string_lossy())?;
    let mut extension_exists = false;

    for counter in &mut extension_counters {
      if counter.extension == file_extension {
        counter.count += 1;
        extension_exists = true;
        break;
      }
    }
    if !extension_exists {
      let new_counter = ExtensionCounter {
        extension: file_extension.clone(),
        count: 1,
      };
      extension_counters.push(new_counter);
    }

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
      count_files_with_same_name += 1;
      let mut rng = rand::thread_rng();
      let random_id: u32 = rng.gen();
      new_name_with_random_id = OsString::from(format!("{}_{}", random_id, file_name_str));
      file_name = OsStr::new(&new_name_with_random_id);
      files_with_repeat_name.push(file_name.to_string_lossy().to_string());
      files_transfered.insert(file_name.to_string_lossy().to_string());
    }

    let output_file = output_dir.join(file_name);

    unsafe {
      FILE_OPERATION
        .execute(&path.to_string_lossy(), &output_file.to_string_lossy())
        .unwrap();
    }
    count_files += 1;
    let percent = ((count_files as f64 / paths_len as f64) * 100.0).round();
    println!(
      "{}% Concluded, File {} of {}  -  File Name: {:?}",
      percent, count_files, paths_len, file_name
    );
  }
  println!("\nFinish!\n");

  if count_files_with_same_name > 0 {
    let output_file_path = format!("{}/filesWithRepeatedName.txt", outputh_directory);
    let output_file = File::create(&output_file_path)?;

    for result in &files_with_repeat_name {
      writeln!(&output_file, "{}", result)?;
    }

    println!(
      "Total of {} file with the repeated name\n",
      count_files_with_same_name
    );
  }

  if count_files > 0 {
    for counter in &extension_counters {
      println!(
        "Total of {} files with '.{}' extension",
        counter.count, counter.extension
      );
    }
    println!("\n\nTotal of {} files organized", count_files);
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

fn get_file_extension(path: &str) -> Result<String, AppError> {
  let path = Path::new(path);

  if let Some(extension) = path.extension() {
    if let Some(extension_str) = extension.to_str() {
      return Ok(String::from(extension_str));
    }
  }
  Err(AppError::FileError(String::from("File has no extension")))
}

fn list_files_in_directory(dir_path: &str) -> Result<Vec<PathBuf>, AppError> {
  println!("Reading directory...");
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
  println!("Done!");
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
