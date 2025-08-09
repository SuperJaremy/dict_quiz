//! Trait for user interface

use std::error::Error;

use crate::{word::question::Question, QuizConfig, QuizResults};

pub trait View {
    /// Display the contents of the question, read the user's input and
    /// compare it with the correct answer.
    fn ask_question(&self, question: Question) -> Result<bool, Box<dyn Error>>;
    /// Prompt the user to set the quiz's parameters.
    fn build_config(&self) -> Result<QuizConfig, Box<dyn Error>>;
    /// Display the end results of the current quiz.
    fn display_results(&self, results: &QuizResults) -> Result<(), Box<dyn Error>>;
    /// Ask user wether to start a new quiz.
    fn try_again(&self) -> Result<bool, Box<dyn Error>>;
}
