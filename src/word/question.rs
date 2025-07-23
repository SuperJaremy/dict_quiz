use super::{AdjectiveSpecific, NounSpecific, PronounSpecific, VerbSpecific, Word, WordForms};
use rand::Rng;

const NOUN_PLURAL_QUESTION: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Write down the plural form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_plural(),
};

const NOUN_SINGULAR_QUESTION: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Write down the singular form of this word",
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_DEFINITE_QUESTION: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Write down the definite form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_definite(),
};

const NOUN_INDEFINITE_QUESTION: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Write down the indefinite form of this word",
    base: |word_forms| word_forms.get_definite(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_QUESTIONS: [QuestionTemplate<NounSpecific>; 4] = [
    NOUN_PLURAL_QUESTION,
    NOUN_SINGULAR_QUESTION,
    NOUN_DEFINITE_QUESTION,
    NOUN_INDEFINITE_QUESTION,
];

const ADJECTIVE_NEUTER_QUESTION: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Write down the neuter form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_neuter(),
};

const ADJECTIVE_QUESTIONS: [QuestionTemplate<AdjectiveSpecific>; 1] = [ADJECTIVE_NEUTER_QUESTION];

const VERB_PRESENT_QUESTION: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Write down the present form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_present(),
};

const VERB_QUESTIONS: [QuestionTemplate<VerbSpecific>; 1] = [VERB_PRESENT_QUESTION];

const PRONOUN_POSSESSIVE_QUESTION: QuestionTemplate<PronounSpecific> = QuestionTemplate {
    question: "Write down the possessive form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_possessive(),
};

const PRONOUN_QUESTIONS: [QuestionTemplate<PronounSpecific>; 1] = [PRONOUN_POSSESSIVE_QUESTION];

struct QuestionTemplate<'a, T> {
    question: &'a str,
    base: fn(&WordForms<T>) -> &str,
    answer: fn(&WordForms<T>) -> &str,
}

pub struct Question {
    question: String,
    base: String,
    answer: String,
}

impl<T> QuestionTemplate<'_, T> {
    fn question_from_template(&self, forms: &WordForms<T>) -> Question {
        Question {
            question: String::from(self.question),
            base: String::from((self.base)(forms)),
            answer: String::from((self.answer)(forms)),
        }
    }

    fn pick_question<'a>(arr: &'a [QuestionTemplate<T>]) -> &'a QuestionTemplate<'a, T> {
        let rand = rand::rng().random_range(0..arr.len());
        &arr[rand]
    }
}

impl Question {
    pub fn get_question_by_word(word: &Word) -> Option<Question> {
        match word {
            Word::NOUN(noun) => {
                let template = QuestionTemplate::pick_question(&NOUN_QUESTIONS);
                Some(template.question_from_template(noun))
            }
            Word::ADJECTIVE(adj) => {
                let template = QuestionTemplate::pick_question(&ADJECTIVE_QUESTIONS);
                Some(template.question_from_template(adj))
            }
            Word::VERB(verb) => {
                let template = QuestionTemplate::pick_question(&VERB_QUESTIONS);
                Some(template.question_from_template(verb))
            }
            Word::PRONOUN(pr) => {
                let template = QuestionTemplate::pick_question(&PRONOUN_QUESTIONS);
                Some(template.question_from_template(pr))
            }
            _ => None,
        }
    }

    pub fn get_question(&self) -> &str {
        &self.question
    }

    pub fn get_base(&self) -> &str {
        &self.base
    }

    pub fn get_answer(&self) -> &str {
        &self.answer
    }
}
