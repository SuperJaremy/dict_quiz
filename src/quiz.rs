use std::rc::Rc;

use anyhow::Result;
use log::info;

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
    /// use dict_quiz::quiz::QuizConfig;
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

pub struct QuizState {
    current_q: usize,
    num_q: usize,
}

/// Quiz's end result to display
pub struct QuizResults<'a> {
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

        info!("Creating quiz with {} questions", questions_num);

        let questions = dict[..]
            .choose_multiple(&mut rng, questions_num)
            .map(|word| {
                let w = word.as_ref();
                (w, w.get_question(&config.category))
            })
            .collect();

        info!("Quiz created");

        Quiz(Rc::new(QuizData {
            questions,
            config,
            view,
        }))
    }

    pub fn start(self) -> Result<()> {
        let mut wrongs = Vec::new();
        let mut correct: u32 = 0;

        info!("Starting quiz");

        for (i, (w, q)) in (&self.0.questions).iter().enumerate() {
            if self.0.view.ask_question(
                q,
                QuizState {
                    current_q: (i + 1),
                    num_q: self.0.config.question_num,
                },
            )? {
                correct += 1;
            } else {
                wrongs.push(*w);
            }
        }

        info!("Quiz finished");

        let result = QuizResults {
            quiz: Quiz(self.0.clone()),
            correct_num: correct,
            wrong_answers: wrongs,
        };

        self.0.view.display_results(&result)?;

        info!("Results displayed");

        Ok(())
    }
}
