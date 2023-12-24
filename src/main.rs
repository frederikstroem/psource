use psource::cli::Cli;
use psource::config::Config;
use psource::printer::Printer;

fn main() -> anyhow::Result<()> {
    let cli = Cli::new();

    let config = Config::new(&cli)?;
    let printer = Printer::new(&cli, &config)?;
    printer.print()?;

    Ok(())
}
