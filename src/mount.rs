use crate::config::Config;
use crate::serv;
use rpassword::read_password;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn mount(config: &Config, has_service: bool) -> Result<(), Box<dyn std::error::Error>> {
  print!("Type a password: ");

  std::io::stdout().flush().unwrap();
  let password = read_password().unwrap();

  for volume in &config.ssec.volumes {
    println!("::> Mounting volume {} ...", &volume[0]);
    let child = Command::new(PathBuf::from(&config.veracrypt.path))
      .arg("/v")
      .arg(&volume[0])
      .arg("/l")
      .arg(&volume[1])
      .arg("/a")
      .arg("/p")
      .arg(&password)
      .arg("/q")
      .arg("/nowaitdlg")
      .arg("y")
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
    serv::starting(config)?;
  }

  Ok(())
}
