use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "CLI tool to pretty print source code to stdout or directly to the clipboard. Skips binary files.", long_about = None)]
pub struct Cli {
    /// Print the source code to stdout
    #[arg(short, long)]
    pub stdout: bool,

    /// Copy the source code to the clipboard
    #[arg(short, long)]
    pub copy: bool,

    // /// Exclude files matching the given regex
    // #[arg(short, long)]
    // pub exclude: Option<String>,

    // /// Include only files matching the given regex
    // #[arg(short, long)]
    // pub include: Option<String>,

    /// Input files and directories
    #[arg(required = true)]
    pub input: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}
