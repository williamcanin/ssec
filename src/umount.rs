use std::{os::windows::process::CommandExt, path::PathBuf, process::Command};

use crate::config::Config;
pub fn umount(config: &Config, has_service: bool) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.umount.services;

  for service in services {
    let args: Vec<&str> = service.split_whitespace().collect();

    #[cfg(target_os = "windows")]
    println!("::> Stop command {} ...", &args[2]);

    #[cfg(target_os = "linux")]
    println!("::> Stop command {} ...", &args[3]);

    let mut cmd = Command::new(args[0]);
    cmd.arg(args[1]);
    cmd.arg(args[2]);
    #[cfg(target_os = "linux")]
    cmd.arg(args[3]);

    #[cfg(target_os = "windows")]
    cmd.creation_flags(0x08000000);

    cmd.spawn()?;

    #[cfg(target_os = "windows")]
    println!("::> Command {} stoped!", &args[2]);

    #[cfg(target_os = "linux")]
    println!("::> Command {} stoped!", &args[3]);
  }

  if has_service {
    std::thread::sleep(std::time::Duration::from_secs(1));

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
  }

  Ok(())
}
