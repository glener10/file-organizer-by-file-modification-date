use std::io;

#[derive(Debug)]
pub enum AppError {
  ReadDirectory(String),
  File(String),
  OrganizeFiles(String),
  IO(io::Error),
}

impl From<io::Error> for AppError {
  fn from(error: io::Error) -> Self {
    AppError::IO(error)
  }
}
