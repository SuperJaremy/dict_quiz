use std::process;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to a .csv dictionary
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let dict_path = cli.path.as_path();

    if let Err(err) = dict_quiz::run(dict_path) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
