use std::{env, io, path::PathBuf};

pub const VERSION: &str = "0.1.0";

pub fn get_executable_dir() -> Option<PathBuf> {
  env::current_exe()
    .ok()
    .and_then(|path| path.parent().map(PathBuf::from))
}

pub fn read(message: String) -> Result<String, Box<dyn std::error::Error>> {
  println!("{}", message);
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(message)
}

pub fn capitalize(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
    None => String::new(),
  }
}
