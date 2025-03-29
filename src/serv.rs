use crate::config::Config;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn starting(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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
  Ok(())
}

pub fn stoping(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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
  Ok(())
}
