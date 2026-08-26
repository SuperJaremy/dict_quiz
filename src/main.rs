use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to a .csv dictionary
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dict_path = cli.path.as_path();

    dict_quiz::run(dict_path)?;
    Ok(())
}
