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

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() != 3 || args[1] != "-d" {
    println!("Use: cargo run -- -d <directory path>");
    return;
  }

  let dir_path = &args[2];

  if let Err(err) = match list_files_in_directory(dir_path) {
    Ok(paths) => {
      for path in paths {
        match get_image_modification_date(&path.to_string_lossy()) {
          Ok(modification_year) => {
            let output_dir = PathBuf::from("output").join(format!("{}", modification_year));
            if let Err(err) = fs::create_dir_all(&output_dir) {
              println!("Error in create output directory: {}", err);
              return;
            }

            let output_file = output_dir.join(path.file_name().unwrap());

            if let Err(err) = fs::copy(&path, &output_file) {
              println!("Error to copy file: {}", err);
            } else {
              println!("File copied: {:?}", output_file);
            }
          }
          Err(err) => eprintln!("Error to obtain modification date: {}", err),
        }
      }
      Ok(())
    }
    Err(err) => Err(format!("Error: {}", err)),
  } {
    println!("{}", err);
  }
}

fn get_image_modification_date(path: &str) -> Result<i32, std::io::Error> {
  let path = Path::new(path);

  let modification_date_in_system_time = fs::metadata(path)?.modified()?;
  let modification_date = DateTime::<Local>::from(modification_date_in_system_time);

  let year = modification_date.year();
  Ok(year)
}

fn list_files_in_directory(dir_path: &str) -> Result<Vec<PathBuf>, String> {
  let mut file_paths = Vec::new();

  for entry in WalkDir::new(dir_path).follow_links(true) {
    match entry {
      Ok(entry) => {
        if entry.file_type().is_file() {
          file_paths.push(entry.path().to_path_buf());
        }
      }
      Err(err) => {
        return Err(format!("Error to list files: {}", err));
      }
    }
  }

  Ok(file_paths)
}
