use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use dirs;
use config;

use crate::cli::Cli;
use super::OutputTarget;

pub struct Config {
    output_target: OutputTarget,
}

impl Config {
    pub fn new(cli: &Cli) -> anyhow::Result<Self> {
        Self::verify_cli(&cli)?;

        let config_file = Self::get_config_file()?;

        let output_target = if cli.stdout {
            OutputTarget::Stdout
        } else if cli.copy || config_file.get_bool("clipboard_is_default_output_target")? {
            OutputTarget::Clipboard
        } else {
            OutputTarget::Stdout
        };

        Ok(
            Self {
                output_target,
            }
        )
    }

    fn verify_cli(cli: &Cli) -> anyhow::Result<()> {
        if cli.stdout && cli.copy {
            return Err(anyhow::anyhow!("Cannot use both --stdout and --copy at the same time."));
        }

        Ok(())
    }

    fn get_config_file() -> anyhow::Result<config::Config> {
        let config_dir = match dirs::config_dir() {
            Some(config_dir) => config_dir,
            None => return Err(anyhow::anyhow!("Could not find config directory.")),
        };
        let config_path = config_dir.join("psource");
        let config_file_name = "config.toml";
        let config_file_path = config_path.join(config_file_name);

        if !config_file_path.exists() {
            // If the configuration file doesn't exist, create it with default values
            Self::create_default_config_file(&config_file_path)?;
        }

        let config_file = config::Config::builder()
            .add_source(config::File::with_name(config_file_path.to_str().unwrap()))
            .set_default("clipboard_is_default_output_target", false)?
            .build()
        ?;

        Ok(config_file)
    }

    fn create_default_config_file(config_file_path: &PathBuf) -> anyhow::Result<()> {
        std::fs::create_dir_all(config_file_path.parent().unwrap())?;
        let mut file = File::create(config_file_path)?;
        writeln!(file, "# Copy the source code to the clipboard instead of printing it to stdout (default: false)")?;
        writeln!(file, "clipboard_is_default_output_target = false")?;
        Ok(())
    }

    pub fn get_output_target(&self) -> &OutputTarget {
        &self.output_target
    }

}
