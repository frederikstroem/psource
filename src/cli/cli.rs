use clap::Parser;

#[derive(Parser)]
#[command(version)]
#[command(about = "CLI tool to pretty print source code to stdout or directly to the clipboard. Skips binary files.", long_about = None)]
pub struct Cli {
    /// Print the source code to stdout
    #[arg(short, long, conflicts_with = "copy")]
    pub stdout: bool,

    /// Copy the source code to the clipboard
    #[arg(short, long, conflicts_with = "stdout")]
    pub copy: bool,

    /// Display the file's ancestry in the output path by including the specified number of parent directories relative to the current working directory, or 0 to omit the ancestry
    #[arg(short, long, default_value = "1", conflicts_with = "git_ancestry")]
    pub ancestry: usize,

    /// Display the file's ancestry in the output path, including parent directories from the current working directory within a Git repository to its root, unlike the fixed number specified by the 'ancestry' option
    #[arg(short, long = "git-ancestry", conflicts_with = "ancestry")]
    pub git_ancestry: bool,

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
