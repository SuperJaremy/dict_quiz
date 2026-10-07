use anyhow::Result;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use flexi_logger::{FileSpec, Logger};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to a .csv dictionary
    path: std::path::PathBuf,
    #[command(flatten)]
    verbosity: Verbosity,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let dict_path = cli.path.as_path();

    let _ = Logger::with(cli.verbosity.log_level_filter())
        .log_to_file(FileSpec::default())
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .start()?;

    dict_quiz::run(dict_path)?;
    Ok(())
}
