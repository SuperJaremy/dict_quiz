use crate::{word::question::Question, QuizConfig, QuizResults};

pub trait View {
    fn ask_question(&self, question: Question) -> bool;
    fn build_config(&self) -> QuizConfig;
    fn display_results(&self, results: &QuizResults);
    fn try_again(&self) -> bool;
}
