//! Console-based quiz interface.

use std::io;

use std::error::Error;

use crate::view::View;
use crate::word::question::Question;
use crate::QuizConfig;
use crate::QuizResults;

pub struct Console;

impl Console {
    fn clear_screen() -> Result<(), Box<dyn Error>> {
        clearscreen::clear()?;
        Ok(())
    }

    fn wait_for_input() -> Result<(), Box<dyn Error>> {
        println!("Type ENTER to continue");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        Ok(())
    }

    /// Creates new interface instance.
    /// As all the instance work with the same stdin,
    /// there should be only one active instance.
    /// #Examples
    /// ```
    /// use dict_quiz::console::Console;
    ///
    /// let console = Console::new();
    /// ```
    pub fn new() -> Console {
        Console {}
    }
}

impl View for Console {
    fn ask_question(&self, question: Question) -> Result<bool, Box<dyn Error>> {
        Console::clear_screen()?;
        println!("Word: {}", question.get_base());
        println!("Question: {}", question.get_question());

        let mut answ = String::new();
        io::stdin().read_line(&mut answ)?;

        let res;
        if answ.trim().to_lowercase() != question.get_answer() {
            println!("❌Incorrect. The correct answer is");
            println!("{}", question.get_answer());
            res = false;
        } else {
            println!("✅Correct!");
            res = true;
        }

        Console::wait_for_input()?;

        Ok(res)
    }

    fn build_config(&self) -> Result<QuizConfig, Box<dyn Error>> {
        Console::clear_screen()?;
        println!("How many questions you'd like in this quiz?");

        let mut number: Option<usize> = None;

        while number.is_none() {
            let mut n = String::new();
            io::stdin().read_line(&mut n)?;
            let n: usize = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Type in a number");
                    continue;
                }
            };
            number = Some(n);
        }

        Ok(QuizConfig {
            question_num: number.expect("number is set in the while loop"),
        })
    }

    fn display_results(&self, results: &QuizResults) -> Result<(), Box<dyn Error>> {
        Console::clear_screen()?;
        println!("Your results:");
        let total = results.config.question_num;
        let correct = results.correct_num as usize;
        println!("Score: {correct} / {total}");

        if total == correct {
            println!("Amazing!");
        } else {
            println!("You should repeat the following words:");
            for w in &results.wrong_answers {
                for (name, form) in w.get_forms() {
                    print!("\t{name}: {form};");
                }
                println!();
            }
        }

        Console::wait_for_input()?;
        Ok(())
    }

    fn try_again(&self) -> Result<bool, Box<dyn Error>> {
        Console::clear_screen()?;

        println!("Try again? [y/n]");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_lowercase() == "y")
    }
}
