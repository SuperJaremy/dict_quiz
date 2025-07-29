use std::error::Error;

use crate::{word::question::Question, QuizConfig, QuizResults};

pub trait View {
    fn ask_question(&self, question: Question) -> Result<bool, Box<dyn Error>>;
    fn build_config(&self) -> Result<QuizConfig, Box<dyn Error>>;
    fn display_results(&self, results: &QuizResults) -> Result<(), Box<dyn Error>>;
    fn try_again(&self) -> Result<bool, Box<dyn Error>>;
}
