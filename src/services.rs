use crate::config::Config;
use crate::process::windows::service;

pub fn starting(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.mount.services;

  for s in services {
    println!("::> Starting command {} ...", s);
    service(s, "start", true, true, false )?;
    println!("::> Command {} started!", s);
  }
  Ok(())
}

pub fn stoping(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.umount.services;
  for s in services {
    println!("::> Stoping command {} ...", s);
    service(s, "stop", true, true, false )?;
    println!("::> Command {} stoped!", s);
  }
  Ok(())
}
