use crate::config::Config;
use crate::services;
use rpassword::read_password;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn mount(config: &Config, has_service: bool) -> Result<(), Box<dyn std::error::Error>> {
  print!("Type a password: ");

  std::io::stdout().flush().unwrap();
  let password = read_password().unwrap();

  for volume in &config.ssec.volumes {
    println!("::> Mounting volume {} in {} ...", &volume[0], &volume[1]);
    let child = Command::new(PathBuf::from(&config.veracrypt.path))
      .args([
        "/q",
        "/nowaitdlg",
        "y",
        "/c",
        "n",
        "/h",
        "n",
        "/v",
        &volume[0],
        "/l",
        &volume[1],
        "/a",
        "/p",
        &password,
      ])
      .output()
      .unwrap();

    match child.status.code().unwrap() {
      1 => break,
      0 => {
        println!("::> Volume {} mounted!", &volume[0]);
      }
      _ => {}
    }
  }

  if has_service {
    std::thread::sleep(std::time::Duration::from_secs(1));
    services::starting(config)?;
  }

  Ok(())
}
