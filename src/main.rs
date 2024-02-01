use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 || args[1] != "-d" {
    println!("Uso: cargo run -- -d <caminho_do_diretorio>");
    return;
  }

  let dir_path = &args[2];

  if let Ok(entries) = fs::read_dir(dir_path) {
    for entry in entries {
      if let Ok(entry) = entry {
        let path = entry.path();

        if path.is_file() && !is_hidden(&path) {
          // Obtém a data de modificação do arquivo
          if let Ok(metadata) = fs::metadata(&path) {
            if let Ok(modified_time) = metadata.modified() {
              //OBTER ANO

              let output_dir = PathBuf::from("output").join(format!("{}", year));
              if let Err(err) = fs::create_dir_all(&output_dir) {
                println!("Erro ao criar diretório de saída: {}", err);
                return;
              }

              let output_file = output_dir.join(path.file_name().unwrap());

              if let Err(err) = fs::copy(&path, &output_file) {
                println!("Erro ao copiar o arquivo: {}", err);
              } else {
                println!("Arquivo copiado: {:?}", output_file);
              }
            }
          }
        }
      }
    }
  } else {
    println!("Erro ao ler o diretório: {}", dir_path);
  }
}

fn is_hidden(path: &Path) -> bool {
  path
    .file_name()
    .and_then(|name| name.to_str())
    .map_or(false, |name| name.starts_with('.'))
}
