//! Console-based quiz interface.

use std::io;

use anyhow::Context;
use anyhow::Result;

use crate::quiz::QuizConfig;
use crate::quiz::QuizResults;
use crate::quiz::view::View;
use crate::word::question::Question;
use crate::word::question::categories::CATEGORY_ADVANCED;
use crate::word::question::categories::CATEGORY_BEGINNER;
use crate::word::question::categories::CATEGORY_INTERMEDIATE;
use crate::word::question::categories::CATEGORY_PROFICIENT;

pub struct Console;

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Console {
    fn clear_screen() -> Result<()> {
        clearscreen::clear().with_context(|| "Failed to clear terminal screen")?;
        Ok(())
    }

    fn wait_for_input() -> Result<()> {
        println!("Type ENTER to continue");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .with_context(|| "Failed to read user's input from terminal")?;

        Ok(())
    }

    /// Creates new interface instance.
    /// As all the instance work with the same stdin,
    /// there should be only one active instance.
    /// #Examples
    /// ```
    /// use dict_quiz::quiz::console::Console;
    ///
    /// let console = Console::new();
    /// ```
    pub fn new() -> Console {
        Console {}
    }
}

impl View for Console {
    fn ask_question(&self, question: &Question) -> Result<bool> {
        Console::clear_screen()?;
        println!("Word: {}", question.get_base());
        println!("Meaning: {}", question.get_meaning());
        println!("Example: {}", question.get_example());
        println!("Question: {}", question.get_question());

        let mut answ = String::new();
        io::stdin()
            .read_line(&mut answ)
            .with_context(|| "Failed to read user's answer")?;

        let res = if !question.check_answer(&answ) {
            println!("❌Incorrect. The correct answer is");
            println!("{}", question.get_answer());
            false
        } else {
            println!("✅Correct!");
            true
        };

        Console::wait_for_input()?;

        Ok(res)
    }

    fn build_config(&self) -> Result<QuizConfig> {
        Console::clear_screen()?;
        println!("How many questions you'd like in this quiz?");

        let mut number: Option<usize> = None;

        while number.is_none() {
            let mut n = String::new();
            io::stdin()
                .read_line(&mut n)
                .with_context(|| "Failed to read question number")?;
            let n: usize = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Type in a number");
                    continue;
                }
            };
            number = Some(n);
        }

        Console::clear_screen()?;
        println!("Choose difficulty level [1-4]:");

        let mut level: Option<i32> = None;

        while level.is_none() {
            let mut n = String::new();
            io::stdin()
                .read_line(&mut n)
                .with_context(|| "Failed to read difficulty number")?;
            let n: i32 = match n.trim().parse() {
                Ok(level) if level < 5 && level > 0 => level,
                _ => {
                    println!("Type in an integer number, 0 < number < 5");
                    continue;
                }
            };
            level = Some(n);
        }

        let category = match level {
            Some(1) => CATEGORY_BEGINNER,
            Some(2) => CATEGORY_INTERMEDIATE,
            Some(3) => CATEGORY_ADVANCED,
            Some(4) => CATEGORY_PROFICIENT,
            _ => panic!("level is chosen in loop"),
        };

        Ok(QuizConfig {
            question_num: number.expect("number is set in the while loop"),
            category,
        })
    }

    fn display_results(&self, results: &QuizResults) -> Result<()> {
        Console::clear_screen()?;
        println!("Your results:");
        let total = results.quiz.0.config.question_num;
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
                println!();
            }
        }

        Console::wait_for_input()?;
        Ok(())
    }

    fn try_again(&self) -> Result<bool> {
        Console::clear_screen()?;

        println!("Try again? [y/n]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .with_context(|| "Failed to read user's reply")?;

        Ok(input.trim().to_lowercase() == "y")
    }
}
