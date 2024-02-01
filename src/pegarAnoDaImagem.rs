use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use image::GenericImageView;
use chrono::{DateTime, Local, Datelike};

fn obter_data_modificacao_imagem(diretorio: &str) -> Result<SystemTime, std::io::Error> {
    let path = Path::new(diretorio);

    // Obtém a data de modificação do arquivo
    let data_modificacao = fs::metadata(path)?.modified()?;

    Ok(data_modificacao)
}

fn obter_informacoes_imagem(diretorio: &str) -> Result<(), image::ImageError> {
    let img = image::open(diretorio)?;

    // Obtém as dimensões da imagem
    let (largura, altura) = img.dimensions();

    // Imprime as informações da imagem
    println!("Dimensões: {} x {}", largura, altura);

    Ok(())
}

fn main() {
    let diretorio_imagem = "C:/Users/glener/Documents/TESTE/2020.jpeg";

    match obter_data_modificacao_imagem(diretorio_imagem) {
      Ok(data_modificacao) => {
          // Converte a SystemTime para DateTime
          let data_modificacao_dt = DateTime::<Local>::from(data_modificacao);
  
          // Obtém o ano da data de modificação
          let ano = data_modificacao_dt.year();
  
          println!("Data de modificação da imagem: {}", data_modificacao_dt);
          println!("Ano da data de modificação: {}", ano);
  
          // Obtém informações adicionais da imagem
          if let Err(err) = obter_informacoes_imagem(diretorio_imagem) {
              eprintln!("Erro ao obter informações da imagem: {}", err);
          }
      }
      Err(err) => eprintln!("Erro ao obter data de modificação: {}", err),
  }
}
