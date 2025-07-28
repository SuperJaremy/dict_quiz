use std::io;

use crate::view::View;
use crate::word::question::Question;
use crate::QuizConfig;
use crate::QuizResults;

pub struct Console;

impl Console {
    fn clear_screen() {
        clearscreen::clear().expect("failed to clear screen");
    }

    fn wait_for_input() {
        println!("Type ENTER to continue");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read line");
    }

    pub fn new() -> Console {
        Console {}
    }
}

impl View for Console {
    fn ask_question(&self, question: Question) -> bool {
        Console::clear_screen();
        println!("Word: {}", question.get_base());
        println!("Question: {}", question.get_question());

        let mut answ = String::new();
        io::stdin()
            .read_line(&mut answ)
            .expect("failed to read line");

        let res;
        if answ.trim().to_lowercase() != question.get_answer() {
            println!("❌Incorrect. The correct answer is");
            println!("{}", question.get_answer());
            res = false;
        } else {
            println!("✅Correct!");
            res = true;
        }

        Console::wait_for_input();

        res
    }

    fn build_config(&self) -> QuizConfig {
        Console::clear_screen();
        println!("How many questions you'd like in this quiz?");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("failed to read line");
        let number: usize = number.trim().parse().expect("failed to parse number");

        QuizConfig {
            question_num: number,
        }
    }

    fn display_results(&self, results: &QuizResults) {
        Console::clear_screen();
        println!("Your results:");
        let total = results.config.question_num;
        let correct = results.correct_num as usize;
        println!("Score: {correct} / {total}");

        if total == correct {
            println!("Amazing!");
            return;
        } else {
            println!("You should repeat the following words:");
            for w in &results.wrong_answers {
                for f in w.get_forms() {
                    print!("{f} ");
                }
                println!();
            }
        }

        Console::wait_for_input();
    }

    fn try_again(&self) -> bool {
        Console::clear_screen();

        println!("Try again? [y/n]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        input.trim().to_lowercase() == "y"
    }
}
