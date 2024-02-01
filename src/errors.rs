use std::io;

#[derive(Debug)]
pub enum AppError {
  ReadDirectoryError(String),
  IOError(io::Error),
}

impl From<io::Error> for AppError {
  fn from(error: io::Error) -> Self {
    AppError::IOError(error)
  }
}
