use console::Console;
use rand::prelude::IndexedRandom;
use view::View;

use crate::word::question::Question;
use crate::word::word_csv;
use crate::word::word_csv::WordCSV;
use std::cmp;
use std::ffi::OsString;

use crate::word::Word;

use std::error::Error;

use rand::rngs::SmallRng;
use rand::SeedableRng;

pub mod console;
pub mod view;
pub mod word;

pub fn run(dict: OsString) -> Result<(), Box<dyn Error>> {
    let dict = read_csv(dict)?;
    let view = Console::new();

    loop {
        let config = view.build_config();
        let questions_num = cmp::min(dict.len(), config.question_num);
        let mut rng = SmallRng::from_os_rng();
        let questions = (&dict[..])
            .choose_multiple(&mut rng, questions_num)
            .map(|word| (word, Question::get_question_by_word(word)));

        let mut wrongs = Vec::new();
        let mut correct: u32 = 0;

        for (w, q) in questions {
            if view.ask_question(q) {
                correct += 1;
            } else {
                wrongs.push(w);
            }
        }

        let result = QuizResults {
            config: &config,
            correct_num: correct,
            wrong_answers: wrongs,
        };

        view.display_results(&result);

        if !view.try_again() {
            break;
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

pub struct QuizConfig {
    question_num: usize,
}

impl QuizConfig {
    pub fn new(question_num: u32) -> QuizConfig {
        QuizConfig {
            question_num: question_num as usize,
        }
    }
}

pub struct QuizResults<'a, 'b> {
    config: &'a QuizConfig,
    correct_num: u32,
    wrong_answers: Vec<&'b Word>,
}
