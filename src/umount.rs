use std::{path::PathBuf, process::Command};

use crate::{config::Config, services};
pub fn umount(config: &Config, has_service: bool) -> Result<(), Box<dyn std::error::Error>> {
  if has_service {
    services::stoping(config)?;
    std::thread::sleep(std::time::Duration::from_secs(1));
  }

  for volume in &config.ssec.volumes {
    println!("::> Umounting volume {} ...", &volume[0]);
    let child = Command::new(PathBuf::from(&config.veracrypt.path))
      .arg("/d")
      .arg(&volume[1])
      .arg("/q")
      .arg("/nowaitdlg")
      .arg("y")
      .output()
      .unwrap();

    match child.status.code().unwrap() {
      1 => break,
      0 => {
        println!("::> Volume {} umounted!", &volume[0]);
      }
      _ => {}
    }
  }

  Ok(())
}
