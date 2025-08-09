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

/// Read the words from the specified csv dictionary and
/// runs the quiz.
pub fn run(dict: OsString) -> Result<(), Box<dyn Error>> {
    let dict = read_csv(dict)?;
    let view = Console::new();

    loop {
        let mut config = view.build_config()?;
        let questions_num = cmp::min(dict.len(), config.question_num);
        config.question_num = questions_num;

        let mut rng = SmallRng::from_os_rng();
        let questions = (&dict[..])
            .choose_multiple(&mut rng, questions_num)
            .map(|word| (word, Question::get_question_by_word(word)));

        let mut wrongs = Vec::new();
        let mut correct: u32 = 0;

        for (w, q) in questions {
            if view.ask_question(q)? {
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

        view.display_results(&result)?;

        if !view.try_again()? {
            break;
        }
    }

    Ok(())
}

fn read_csv(dict_path: OsString) -> Result<Vec<Word>, Box<dyn Error>> {
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

/// Configuration parameters for a quiz instanse.
pub struct QuizConfig {
    question_num: usize,
}

impl QuizConfig {
    /// Creates a filled in configuration.
    /// # Examples
    /// ```
    /// use dict_quiz::QuizConfig;
    ///
    /// let conf = QuizConfig::new(10);
    /// ```
    pub fn new(question_num: u32) -> QuizConfig {
        QuizConfig {
            question_num: question_num as usize,
        }
    }
}

/// Quiz's end result to display
pub struct QuizResults<'a, 'b> {
    config: &'a QuizConfig,
    correct_num: u32,
    wrong_answers: Vec<&'b Word>,
}
