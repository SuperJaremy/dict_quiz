use std::error::Error;
use std::rc::Rc;

use crate::Pick;
use crate::cmp;
use crate::word::question::{Question, categories::Category};
use rand::prelude::IndexedRandom;
use rand::{SeedableRng, rngs::SmallRng};
use view::View;

pub mod console;
pub mod view;

/// Configuration parameters for a quiz instanse.
pub struct QuizConfig {
    question_num: usize,
    category: Category,
}

impl QuizConfig {
    /// Creates a filled in configuration.
    /// # Examples
    /// ```
    /// use dict_quiz::QuizConfig;
    /// use dict_quiz::word::question::categories::CATEGORY_BEGINNER;
    /// let conf = QuizConfig::new(10, CATEGORY_BEGINNER);
    /// ```
    pub fn new(question_num: u32, category: Category) -> QuizConfig {
        QuizConfig {
            question_num: question_num as usize,
            category,
        }
    }
}

/// Quiz's end result to display
pub struct QuizResults<'a> {
    // quiz: &'a Quiz<'a>,
    quiz: Quiz<'a>,
    correct_num: u32,
    wrong_answers: Vec<&'a dyn Pick>,
}

struct QuizData<'a> {
    questions: Vec<(&'a dyn Pick, Question)>,
    config: QuizConfig,
    view: &'a dyn View,
}

pub struct Quiz<'a>(Rc<QuizData<'a>>);

impl<'a> Quiz<'_> {
    pub fn new(mut config: QuizConfig, dict: &'a [Box<dyn Pick>], view: &'a dyn View) -> Quiz<'a> {
        let questions_num = cmp::min(dict.len(), config.question_num);
        config.question_num = questions_num;

        let mut rng = SmallRng::from_os_rng();
        let questions = dict[..]
            .choose_multiple(&mut rng, questions_num)
            .map(|word| {
                let w = word.as_ref();
                (w, w.get_question(&config.category))
            })
            .collect();
        Quiz(Rc::new(QuizData {
            questions,
            config,
            view,
        }))
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        let mut wrongs = Vec::new();
        let mut correct: u32 = 0;

        for (w, q) in &self.0.questions {
            if self.0.view.ask_question(q)? {
                correct += 1;
            } else {
                wrongs.push(*w);
            }
        }

        let result = QuizResults {
            quiz: Quiz(self.0.clone()),
            correct_num: correct,
            wrong_answers: wrongs,
        };

        self.0.view.display_results(&result)?;

        Ok(())
    }
}
