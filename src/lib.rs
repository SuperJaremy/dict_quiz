use crate::word::question::Question;
use crate::word::word_csv;
use crate::word::word_csv::WordCSV;
use std::ffi::OsString;
use std::io;

use crate::word::Word;

use std::error::Error;

pub mod word;

pub fn run(dict: OsString) -> Result<(), Box<dyn Error>> {
    let dict = read_csv(dict)?;

    for w in &dict {
        if ask_question(w) {
            println!("✅Correct!");
        } else {
            println!("❌Incorrect");
        }
    }
    Ok(())
}

fn read_csv(dict_path: OsString) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(dict_path)?;
    let mut res = Vec::new();

    for result in rdr.deserialize() {
        let record: WordCSV = match result {
            Ok(word) => word,
            Err(_) => {
                eprintln!("Warning! Incorrect entry in csv file");
                continue;
            }
        };
        let word = match word_csv::word_csv_to_word(record) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Warning! Incorrect entry in csv file, {}", e);
                continue;
            }
        };
        res.push(word);
    }

    Ok(res)
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
