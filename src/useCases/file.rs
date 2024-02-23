use chrono::{DateTime, Datelike, Local};
use std::fs::{self};
use std::path::Path;

use crate::errors::AppError;

pub fn get_file_extension(path: &str) -> Result<String, AppError> {
  let path = Path::new(path);

  if let Some(extension) = path.extension() {
    if let Some(extension_str) = extension.to_str() {
      return Ok(String::from(extension_str));
    }
  }
  Err(AppError::FileError(String::from("File has no extension")))
}

pub fn get_file_modification_date(path: &str) -> Result<i32, AppError> {
  let path = Path::new(path);

  let modification_date_in_system_time = fs::metadata(path)?.modified()?;
  let modification_date = DateTime::<Local>::from(modification_date_in_system_time);

  let year = modification_date.year();
  Ok(year)
}
