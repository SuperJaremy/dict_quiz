use std::error::Error;

use rand::prelude::IndexedRandom;
use crate::cmp;
use crate::Pick;
use view::View;
use crate::{word::question::{Question, categories::Category}};
use rand::{SeedableRng, rngs::SmallRng};

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
pub struct QuizResults<'a, 'b> {
	config: &'a QuizConfig,
    correct_num: u32,
    wrong_answers: Vec<&'b Box<dyn Pick>>,
}

pub struct Quiz<'a> {
	questions: Vec<(&'a Box<dyn Pick>, Question)>,
	config: QuizConfig,
	view: &'a Box<dyn View>,
}

impl<'a> Quiz<'_> {
	pub fn new(mut config: QuizConfig, dict: &'a [Box<dyn Pick>], view: &'a Box<dyn View>) -> Quiz<'a> {
		let questions_num = cmp::min(dict.len(), config.question_num);
		config.question_num = questions_num;

		let mut rng = SmallRng::from_os_rng();
		let questions = dict[..]
            .choose_multiple(&mut rng, questions_num)
            .map(|word| (word, word.get_question(&config.category))).collect();
        Quiz { questions, config, view}
	}

	pub fn start(self) -> Result<(), Box<dyn Error>> {
		let mut wrongs = Vec::new();
        let mut correct: u32 = 0;

		for (w, q) in self.questions {
            if self.view.ask_question(q)? {
                correct += 1;
            } else {
                wrongs.push(w);
            }
        }

        let result = QuizResults {
            config: &self.config,
            correct_num: correct,
            wrong_answers: wrongs,
        };

        self.view.display_results(&result)?;

        Ok(())
	}
}