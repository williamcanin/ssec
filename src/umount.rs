use crate::config::Config;
use std::{error::Error, path::PathBuf, process::Command};

pub fn umount(config: &Config) -> Result<(), Box<dyn Error>> {
  for volume in &config.ssec.volumes {
    println!("::> Disassembly volume {} ...", &volume[0]);
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
        println!("::> Volume {} disassembled!", &volume[0]);
      }
      _ => {}
    }
  }

  Ok(())
}
