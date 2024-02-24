use std::path::PathBuf;
use walkdir::WalkDir;

use crate::errors::AppError;

pub fn list_files_in_directory(dir_path: &str) -> Result<Vec<PathBuf>, AppError> {
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
        return Err(AppError::ReadDirectory(error_message));
      }
    }
  }
  println!("Done!");
  Ok(file_paths)
}
