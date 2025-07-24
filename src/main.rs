use word::{question::Question, word_csv};

use crate::word::Word;
use std::{env, error::Error, ffi::OsString, io, process};

pub mod word;

fn run() -> Result<(), Box<dyn Error>> {
    let dict = get_frist_arg()?;
    let dict = word_csv::read_csv(dict)?;

    for w in &dict {
        if ask_question(w) {
            println!("✅Correct!");
        } else {
            println!("❌Incorrect");
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

fn ask_question(word: &Word) -> bool {
    let question = Question::get_question_by_word(word);
    println!("{}", question.get_base());
    println!("{}", question.get_question());

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

    question.get_answer() == answer.trim().to_lowercase()
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
