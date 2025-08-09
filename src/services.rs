use crate::config::Config;
use crate::process::windows::service;

pub fn starting(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.mount.services;
  service(services, "start", true, true, false)?;
  Ok(())
}

pub fn stoping(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.umount.services;
  service(services, "stop", true, true, false)?;
  Ok(())
}
