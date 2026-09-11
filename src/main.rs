use anyhow::Result;
use clap::Parser;
use flexi_logger::{FileSpec, Logger};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to a .csv dictionary
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let _ = Logger::try_with_env_or_str("info")?
        .log_to_file(FileSpec::default())
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .start()?;

    let cli = Cli::parse();
    let dict_path = cli.path.as_path();

    dict_quiz::run(dict_path)?;
    Ok(())
}
