use std::env;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "-d" {
        println!("Uso: cargo run -- -d <caminho_do_diretorio>");
        return;
    }

    let dir_path = &args[2];

    if let Err(err) = list_files_in_directory(dir_path) {
        println!("Erro: {}", err);
    }
}

fn list_files_in_directory(dir_path: &str) -> Result<(), String> {
    for entry in WalkDir::new(dir_path).follow_links(true) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    println!("{}", entry.path().display());
                }
            }
            Err(err) => {
                return Err(format!("Erro ao listar arquivos: {}", err));
            }
        }
    }

    Ok(())
}
