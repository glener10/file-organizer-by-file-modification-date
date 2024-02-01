//TODO: arg to define output directory
//TODO: arg to define copy ou cut files
//TODO: Create README file
//TODO: See how made better error handling in rust
//TODO: Separate in more folders
//TODO: Consider whether the file is not hidden
//TODO: Use Logging system
use chrono::{DateTime, Datelike, Local};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod errors;
use errors::AppError;

fn main() -> Result<(), AppError> {
  let args: Vec<String> = read_args()?;
  let dir_path = &args[2];

  let paths = list_files_in_directory(dir_path)?;
  for path in paths {
    let modification_year = get_file_modification_date(&path.to_string_lossy())?;

    let output_dir = PathBuf::from("output").join(format!("{}", modification_year));
    fs::create_dir_all(&output_dir)?;

    let output_file = output_dir.join(path.file_name().ok_or_else(|| {
      AppError::IOError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No file name",
      ))
    })?);

    fs::copy(&path, &output_file)?;
    println!("File copied: {:?}", output_file);
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

fn read_args() -> Result<Vec<String>, AppError> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 || args[1] != "-d" {
    print_helping_informations();
    return Err(AppError::ArgsError("Invalid arguments".to_string()));
  }
  Ok(args)
}

fn print_helping_informations() {
  println!("Use: cargo run -- -d <directory path>");
}
