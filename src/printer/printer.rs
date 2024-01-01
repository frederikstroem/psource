use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
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

    fn get_ancestry_path(&self, path: &Path, ancestry: usize) -> String {
        // Get current working directory. This is the directory from which the program was executed.
        let cwd: PathBuf = env::current_dir().expect("Failed to get current working directory");

        // Collect ancestors into a vector.
        let mut ancestors_full_path = cwd.ancestors().collect::<Vec<&Path>>();
        // // Print
        // for (i, ancestor) in ancestors_full_path.iter().enumerate() {
        //     println!("{}: {}", i, ancestor.to_string_lossy());
        // }
        // // 0: /home/tux/projects/psource
        // // 1: /home/tux/projects
        // // 2: /home/tux
        // // 3: /home
        // // 4: /

        // Remove last element (root).
        ancestors_full_path.pop();
        // // Print
        // for (i, ancestor) in ancestors_full_path.iter().enumerate() {
        //     println!("{}: {}", i, ancestor.to_string_lossy());
        // }
        // // 0: /home/tux/projects/psource
        // // 1: /home/tux/projects
        // // 2: /home/tux
        // // 3: /home

        // Trim ancestors down to directory names only.
        let ancestors = ancestors_full_path
            .iter()
            .map(|ancestor| ancestor.file_name().unwrap())
            .collect::<Vec<&OsStr>>();
        // // Print
        // for (i, ancestor) in ancestors.iter().enumerate() {
        //     println!("{}: {}", i, ancestor.to_string_lossy());
        // }
        // // 0: psource
        // // 1: projects
        // // 2: tux
        // // 3: home

        // Create an iterator over the ancestors.
        let mut ancestors = ancestors.into_iter();

        // Create a string to hold the ancestry path.
        let mut ancestry_path = String::new();

        // Prepend <ANCESTRY> ancestors to the path, if they exist.
        for _ in 0..ancestry {
            match ancestors.next() {
                Some(ancestor) => {
                    ancestry_path.insert_str(0, &format!("{}/", ancestor.to_string_lossy()));
                },
                None => {
                    break;
                },
            }
        }

        // Prepend a trailing slash to the path.
        ancestry_path.insert_str(0, "/");

        // Append the path to the file
        ancestry_path.push_str(&format!("{}", path.to_string_lossy()));

        // Return the ancestry path
        ancestry_path
    }

    fn get_output(&self) -> anyhow::Result<String> {
        let paths = self.file_walker.walk(&self.cli.input)?;

        let mut output = String::new();
        for path in paths {
            let content = fs::read_to_string(&path)?;
            let ancestry_path = self.get_ancestry_path(&path, self.cli.ancestry);
            output.push_str(&format!("⚫ {}\n{}\n", ancestry_path, content));
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
