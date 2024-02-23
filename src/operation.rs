use std::fs::{self};

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

pub static mut FILE_OPERATION: FileOperation = FileOperation::Copy;
