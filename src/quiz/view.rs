//! Trait for user interface

use anyhow::Result;

use crate::{quiz::QuizConfig, quiz::QuizResults, word::question::Question};

pub trait View {
    /// Display the contents of the question, read the user's input and
    /// compare it with the correct answer.
    fn ask_question(&self, question: &Question) -> Result<bool>;
    /// Prompt the user to set the quiz's parameters.
    fn build_config(&self) -> Result<QuizConfig>;
    /// Display the end results of the current quiz.
    fn display_results(&self, results: &QuizResults) -> Result<()>;
    /// Ask user wether to start a new quiz.
    fn try_again(&self) -> Result<bool>;
}
