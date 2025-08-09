mod config;
mod mount;
mod process;
mod services;
mod umount;
mod utils;

use clap::{Parser, Subcommand};
use std::{error::Error, path::PathBuf};

#[derive(Subcommand)]
enum Commands {
  /// Mount the volumes
  Mount,
  /// Umount the volumes
  Umount,
  /// Starting services
  ServeStop,
  /// Stoping services
  ServeStart,
}

fn about() -> String {
  format!(
    "Ssec (version: {}) - Mounting VeraCrypt volumes efficiently.",
    utils::VERSION
  )
}

#[derive(Parser)]
#[command(name = "ssec")]
#[command(version = utils::VERSION)]
#[command(about = about(), long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

use config::{Config, load_config};

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();

  match utils::get_executable_dir() {
    Some(dir) => {
      let json_file = PathBuf::from(&dir).join("config.json");
      let config: Config = load_config(&PathBuf::from(dir), &json_file)?;

      match cli.command {
        Some(Commands::Mount) => {
          mount::mount(&config)?;
          if config.commands.mount.enable {
            services::starting(&config)?;
          }
        }

        Some(Commands::Umount) => {
          if config.commands.umount.enable {
            services::stoping(&config)?;
          }
          umount::umount(&config)?;
        }

        Some(Commands::ServeStart) => {
          services::starting(&config)?;
        }

        Some(Commands::ServeStop) => {
          services::stoping(&config)?;
        }

        None => {
          println!("Use `help` to see the available options.");
        }
      }
    }

    None => eprintln!("ERROR: Could not get executable directory."),
  }

  // utils::read("Press ENTER to exit:".to_string())?;

  Ok(())
}
