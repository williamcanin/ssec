use crate::config::Config;
use crate::process::windows::service;

pub fn starting(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.mount.services;
  println!("::> Starting services...");
  service(services, "start", true, true, false)?;
  println!("::> Services started successfully!");
  Ok(())
}

pub fn stoping(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
  let services = &config.commands.umount.services;
  println!("::> Stopping services...");
  service(services, "stop", true, true, false)?;
  println!("::> Services stopped successfully!");
  Ok(())
}
