use serde::Deserialize;
use std::{
  fs::{self, exists},
  path::PathBuf,
  process::exit,
};

#[derive(Debug, Deserialize)]
pub struct Config {
  pub veracrypt: Veracrypt,
  pub ssec: Seec,
  pub commands: Commands,
}

#[derive(Debug, Deserialize)]
pub struct Veracrypt {
  pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Seec {
  pub volumes: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Commands {
  pub mount: Mount,
  pub umount: Umount,
}

#[derive(Debug, Deserialize)]
pub struct Mount {
  pub enable: bool,
  pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Umount {
  pub enable: bool,
  pub services: Vec<String>,
}

pub fn load_config(
  root: &PathBuf,
  filename: &PathBuf,
) -> Result<Config, Box<dyn std::error::Error>> {
  if !exists(filename)? {
    eprintln!("Não existe o arquivo 'config.json' em {}", root.display());
    exit(1);
  }
  let file_content = fs::read_to_string(filename)?;
  let config: Config = serde_json::from_str(&file_content).expect("Erro ao desserializar JSON");
  Ok(config)
}
