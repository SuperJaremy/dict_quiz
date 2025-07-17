// Copied from csv crate tutorial
use std::{env, error::Error, ffi::OsString, process};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_frist_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn get_frist_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 arguemnt, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
