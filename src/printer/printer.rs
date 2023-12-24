use std::fs;
use std::time::Duration;
use arboard::Clipboard;

use crate::{config::{Config, OutputTarget}, cli::Cli};
use crate::printer::file_walker::FileWalker;

pub struct Printer<'a> {
    cli: &'a Cli,
    config: &'a Config,
    file_walker: FileWalker,
}

impl<'a> Printer<'a> {
    pub fn new(cli: &'a Cli, config: &'a Config) -> anyhow::Result<Self> {
        Ok(
            Self {
                cli,
                config,
                file_walker: FileWalker::new(),
            }
        )
    }

    fn get_output(&self) -> anyhow::Result<String> {
        let paths = self.file_walker.walk(&self.cli.input)?;

        let mut output = String::new();
        for path in paths {
            let content = fs::read_to_string(&path)?;
            output.push_str(&format!("⚫ {}\n{}\n", path.display(), content));
        }

        Ok(output)
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let output = self.get_output()?;

        match self.config.get_output_target() {
            OutputTarget::Stdout => {
                println!("{}", &output);
            },
            OutputTarget::Clipboard => {
                let mut clipboard = Clipboard::new().unwrap();
                clipboard.set_text(&output).unwrap();

                // https://github.com/1Password/arboard/issues/114
                // https://github.com/sigoden/aichat/issues/160
                // std::thread::sleep(Duration::from_secs(1));
                std::thread::sleep(Duration::from_millis(50));
            },
        }

        Ok(())
    }
}
