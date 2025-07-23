use super::{
    AdjectiveSpecific, AdverbSpecific, NounSpecific, PronounSpecific, VerbSpecific, Word, WordForms,
};
use rand::Rng;

const NOUN_TRANSLATION_QUESTION_E: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Translate this word in English",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const NOUN_TRANSLATION_QUESTION_S: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: "Translate this word in Swedish",
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

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

const NOUN_QUESTIONS: [QuestionTemplate<NounSpecific>; 6] = [
    NOUN_TRANSLATION_QUESTION_E,
    NOUN_TRANSLATION_QUESTION_S,
    NOUN_PLURAL_QUESTION,
    NOUN_SINGULAR_QUESTION,
    NOUN_DEFINITE_QUESTION,
    NOUN_INDEFINITE_QUESTION,
];

const ADJECTIVE_TRANSLATION_QUESTION_E: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Translate this word in English",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const ADJECTIVE_TRANSLATION_QUESTION_S: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Translate this word in Swedish",
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_NEUTER_QUESTION: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Write down the neuter form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_neuter(),
};

const ADJECTIVE_NON_NEUTER_QUESTION: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Write down the non-neuter form of this word",
    base: |word_forms| word_forms.get_neuter(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_PLURAL_QUESTION: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Write down the plural form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_plural(),
};

const ADJECTIVE_SINGULAR_QUESTION: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: "Write down the singular form of this word",
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_QUESTIONS: [QuestionTemplate<AdjectiveSpecific>; 6] = [
    ADJECTIVE_TRANSLATION_QUESTION_E,
    ADJECTIVE_TRANSLATION_QUESTION_S,
    ADJECTIVE_NEUTER_QUESTION,
    ADJECTIVE_NON_NEUTER_QUESTION,
    ADJECTIVE_PLURAL_QUESTION,
    ADJECTIVE_SINGULAR_QUESTION,
];

const VERB_TRANSLATION_QUESTION_E: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Translate this word in English",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const VERB_TRANSLATION_QUESTION_S: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Translate this word in Swedish",
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_PRESENT_QUESTION: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Write down the present form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_present(),
};

const VERB_PAST_QUESTION: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Write down the past form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_past(),
};

const VERB_INFINITIVE_QUESTION: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: "Write down the infinitive of this word",
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_QUESTIONS: [QuestionTemplate<VerbSpecific>; 5] = [
    VERB_TRANSLATION_QUESTION_E,
    VERB_TRANSLATION_QUESTION_S,
    VERB_PRESENT_QUESTION,
    VERB_PAST_QUESTION,
    VERB_INFINITIVE_QUESTION,
];

const ADVERB_TRANSLATION_QUESTION_E: QuestionTemplate<AdverbSpecific> = QuestionTemplate {
    question: "Translate this word in English",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const ADVERB_TRANSLATION_QUESTION_S: QuestionTemplate<AdverbSpecific> = QuestionTemplate {
    question: "Translate this word in Swedish",
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const ADVERB_QUESTIONS: [QuestionTemplate<AdverbSpecific>; 2] =
    [ADVERB_TRANSLATION_QUESTION_E, ADVERB_TRANSLATION_QUESTION_S];

const PRONOUN_TRANSLATION_QUESTION_E: QuestionTemplate<PronounSpecific> = QuestionTemplate {
    question: "Translate this word in English",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const PRONOUN_TRANSLATION_QUESTION_S: QuestionTemplate<PronounSpecific> = QuestionTemplate {
    question: "Translate this word in Swedish",
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const PRONOUN_POSSESSIVE_QUESTION: QuestionTemplate<PronounSpecific> = QuestionTemplate {
    question: "Write down the possessive form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_possessive(),
};

const PRONOUN_SUBJECTIVE_QUESTION: QuestionTemplate<PronounSpecific> = QuestionTemplate {
    question: "Write down the subjective form of this word",
    base: |word_forms| word_forms.get_possessive(),
    answer: |word_forms| word_forms.get_word(),
};

const PRONOUN_QUESTIONS: [QuestionTemplate<PronounSpecific>; 4] = [
    PRONOUN_TRANSLATION_QUESTION_E,
    PRONOUN_TRANSLATION_QUESTION_S,
    PRONOUN_POSSESSIVE_QUESTION,
    PRONOUN_SUBJECTIVE_QUESTION,
];

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
    pub fn get_question_by_word(word: &Word) -> Question {
        match word {
            Word::NOUN(noun) => {
                let template = QuestionTemplate::pick_question(&NOUN_QUESTIONS);
                template.question_from_template(noun)
            }
            Word::ADJECTIVE(adj) => {
                let template = QuestionTemplate::pick_question(&ADJECTIVE_QUESTIONS);
                template.question_from_template(adj)
            }
            Word::VERB(verb) => {
                let template = QuestionTemplate::pick_question(&VERB_QUESTIONS);
                template.question_from_template(verb)
            }
            Word::PRONOUN(pr) => {
                let template = QuestionTemplate::pick_question(&PRONOUN_QUESTIONS);
                template.question_from_template(pr)
            }
            Word::ADVERB(adv) => {
                let template = QuestionTemplate::pick_question(&ADVERB_QUESTIONS);
                template.question_from_template(adv)
            }
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
