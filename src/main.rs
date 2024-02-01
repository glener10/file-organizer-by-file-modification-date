use std::env;
use std::path::PathBuf;
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
              //TODO: Take modification date of path file
              //TODO: Create output destination if doenst exists
              //TODO: Save File in the output destination
                println!("{}", path.display());
            }
            Ok(())
        }
        Err(err) => Err(format!("Error: {}", err)),
    } {
        println!("{}", err);
    }
}

fn list_files_in_directory(dir_path: &str) -> Result<Vec<PathBuf>, String> {
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(dir_path).follow_links(true) {
        match entry {
            Ok(entry) => {
              //TODO: Consider whether the file is not hidden
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
