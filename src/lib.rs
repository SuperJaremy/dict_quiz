//! The application reads Swedish words and their forms from
//! a csv file and runs randomized quizes on these words.
//!
//! # Setup
//! In order for application  to work you should provide
//! a path to a vaild dictionary in csv format with fields
//! specified in `word/word_csv.rs` as the first command line argument.
//!
//! # Gameplay
//! First, you'll be prompted to type in the number of questions
//! you want in your quiz. Each question is related to exaclty
//! one word, which means that the same word can never be asked
//! twice in one quize. Therefore, the number of questions is limited
//! to the number of words in dictionary.
//!
//! After that the specified number of questions will be asked
//! one at a time. In each question you are expected to fill in
//! you answer. If the answer is incorrect, the correct answer
//! will be displayed for you to remember.
//!
//! In the end of the quiz you'll see the statistics screen
//! with the number of questions you've got right. All the
//! words you've filled in incorrect will be presented as
//! well, so you could take another look at them and learn.
//!
//! # Questions
//! All questions are based on the classes of each word. You'll
//! be asked to either translate the word in English or in Swedish,
//! or transofrm a word from one of its forms to another. The question order
//! as well as the questions themselves are randomized.

use crate::quiz::Quiz;
use crate::word::question::Pick;
use quiz::console::Console;
use quiz::view::View;

use crate::word::word_csv;
use crate::word::word_csv::WordCSV;
use std::cmp;
use std::ffi::OsString;

use std::error::Error;


pub mod word;
pub mod quiz;

/// Read the words from the specified csv dictionary and
/// runs the quiz.
pub fn run(dict: OsString) -> Result<(), Box<dyn Error>> {
    let dict = read_csv(dict)?;
    let view = Console::new();

    loop {
        let config = view.build_config()?;

        let quiz = Quiz::new(config, &dict, &view);
        quiz.start()?;

        if !view.try_again()? {
            break;
        }
    }

    Ok(())
}

fn read_csv(dict_path: OsString) -> Result<Vec<Box<dyn Pick>>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(dict_path)?;
    let mut res = Vec::new();

    for (line, result) in rdr.deserialize().enumerate() {
        let record: WordCSV = match result {
            Ok(word) => word,
            Err(_) => {
                eprintln!("Warning! Incorrect entry in csv file in line {line}");
                continue;
            }
        };
        let word = match word_csv::word_csv_to_word(record) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("Warning! Incorrect entry in csv file in line {line}, {}", e);
                continue;
            }
        };
        res.push(word);
    }

    Ok(res)
}

