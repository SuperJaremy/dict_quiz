use std::{env, error::Error, ffi::OsString, process};

fn get_frist_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 arguemnt, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    let dict = get_frist_arg().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = dict_quiz::run(dict) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
