use serde::Deserialize;
use std::{env, error::Error, ffi::OsString, io, process};

#[derive(Debug, Deserialize)]
struct Word {
    #[serde(rename = "word")]
    word: String,
    #[serde(rename = "translation")]
    translation: String,
    #[serde(rename = "class")]
    class: String,
    #[serde(rename = "definite(noun)")]
    definite: Option<String>,
    #[serde(rename = "plural(noun;adj)")]
    plural: Option<String>,
    #[serde(rename = "neuter(adj)")]
    neuter: Option<String>,
    #[serde(rename = "present(verb)")]
    present: Option<String>,
    #[serde(rename = "past(verb)")]
    past: Option<String>,
    #[serde(rename = "possessive(pronoun)")]
    possesive: Option<String>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_frist_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: Word = result?;
        if ask_translation(&record) {
            println!("Correct!");
        } else {
            println!("Incorrect");
        }
    }

    Ok(())
}

fn get_frist_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 arguemnt, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn ask_translation(word: &Word) -> bool {
    println!("{}", word.word);
    println!("Translate this word in English");

    let mut answer = String::new();

    loop {
        match io::stdin().read_line(&mut answer) {
            Ok(_) => break,
            Err(_) => {
                println!("Try again");
                answer.clear();
            }
        }
    }

    word.translation == answer.trim()
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
