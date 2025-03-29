use crate::config::Config;
use rpassword::read_password;
use std::io::Write;
use std::os::windows::process::CommandExt;
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

    let services = &config.commands.mount.services;

    for service in services {
      let args: Vec<&str> = service.split_whitespace().collect();

      #[cfg(target_os = "windows")]
      println!("::> Starting service: {} ...", &args[2]);

      #[cfg(target_os = "linux")]
      println!("::> Starting service: {} ...", &args[3]);

      let mut cmd = Command::new(args[0]);
      cmd.arg(args[1]);
      cmd.arg(args[2]);
      #[cfg(target_os = "linux")]
      cmd.arg(args[3]);

      #[cfg(target_os = "windows")]
      cmd.creation_flags(0x08000000);
      cmd.spawn()?;

      #[cfg(target_os = "windows")]
      println!("::> Service: {} started!", &args[2]);

      #[cfg(target_os = "linux")]
      println!("::> Service: {} started!", &args[3]);
    }
  }

  Ok(())
}
