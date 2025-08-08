use super::{
    AdjectiveSpecific, AdverbSpecific, NounSpecific, PersonalPronounSpecific, VerbSpecific, Word,
    WordForms,
};
use rand::Rng;

const TRANSLATION_QUESTION_E: &str = "Translate this word in English";
const TRANSLATION_QUESTION_S: &str = "Translate this word in Swedish";
const BASE_QUESTION: &str = "Write down the base form of this word";

const NOUN_TRANSLATION_QUESTION_E: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const NOUN_TRANSLATION_QUESTION_S: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_BASE_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_indefinite_plural(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_BASE_QUESTION_DEFINITE_PLURAL: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_definite_plural(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_BASE_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_definite_singular(),
    answer: |word_forms| word_forms.get_word(),
};

const NOUN_INDEFINITE_PLURAL_QUESTION: &str = "Write down the indefinite plural form of this word";

const NOUN_INDEFINITE_PLURAL_QUESTION_BASE: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: NOUN_INDEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_indefinite_plural(),
};

const NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_INDEFINITE_PLURAL_QUESTION,
        base: |word_forms| word_forms.get_definite_singular(),
        answer: |word_forms| word_forms.get_indefinite_plural(),
    };

const NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_PLURAL: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_INDEFINITE_PLURAL_QUESTION,
        base: |word_forms| word_forms.get_definite_plural(),
        answer: |word_forms| word_forms.get_indefinite_plural(),
    };

const NOUN_DEFINITE_SINGULAR_QUESTION: &str = "Write down the definite singular form of this word";

const NOUN_DEFINITE_SINGULAR_QUESTION_BASE: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: NOUN_DEFINITE_SINGULAR_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_definite_singular(),
};

const NOUN_DEFINITE_SINGULAR_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_DEFINITE_SINGULAR_QUESTION,
        base: |word_forms| word_forms.get_indefinite_plural(),
        answer: |word_forms| word_forms.get_definite_singular(),
    };

const NOUN_DEFINITE_SINGULAR_QUESTION_DEFINITE_PLURAL: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_DEFINITE_SINGULAR_QUESTION,
        base: |word_forms| word_forms.get_definite_plural(),
        answer: |word_forms| word_forms.get_definite_singular(),
    };

const NOUN_DEFINITE_PLURAL_QUESTION: &str = "Write down the definite plural form of this word";

const NOUN_DEFINITE_PLURAL_QUESTION_BASE: QuestionTemplate<NounSpecific> = QuestionTemplate {
    question: NOUN_DEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_definite_plural(),
};

const NOUN_DEFINITE_PLURAL_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_DEFINITE_PLURAL_QUESTION,
        base: |word_forms| word_forms.get_indefinite_plural(),
        answer: |word_forms| word_forms.get_definite_plural(),
    };

const NOUN_DEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<NounSpecific> =
    QuestionTemplate {
        question: NOUN_DEFINITE_PLURAL_QUESTION,
        base: |word_forms| word_forms.get_definite_singular(),
        answer: |word_forms| word_forms.get_definite_plural(),
    };

const NOUN_QUESTIONS: [QuestionTemplate<NounSpecific>; 14] = [
    NOUN_TRANSLATION_QUESTION_E,
    NOUN_TRANSLATION_QUESTION_S,
    NOUN_BASE_QUESTION_DEFINITE_PLURAL,
    NOUN_BASE_QUESTION_DEFINITE_SINGULAR,
    NOUN_BASE_QUESTION_INDEFINITE_PLURAL,
    NOUN_DEFINITE_PLURAL_QUESTION_BASE,
    NOUN_DEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR,
    NOUN_DEFINITE_PLURAL_QUESTION_INDEFINITE_PLURAL,
    NOUN_DEFINITE_SINGULAR_QUESTION_BASE,
    NOUN_DEFINITE_SINGULAR_QUESTION_DEFINITE_PLURAL,
    NOUN_DEFINITE_SINGULAR_QUESTION_INDEFINITE_PLURAL,
    NOUN_INDEFINITE_PLURAL_QUESTION_BASE,
    NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_PLURAL,
    NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR,
];

const ADJECTIVE_TRANSLATION_QUESTION_E: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const ADJECTIVE_TRANSLATION_QUESTION_S: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_BASE_QUESTION_NEUTER: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_neuter(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_BASE_QUESTION_PLURAL: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_word(),
};

const ADJECTIVE_NEUTER_QUESTION: &str = "Write down the neuter form of this word";

const ADJECTIVE_NEUTER_QUESTION_BASE: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: ADJECTIVE_NEUTER_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_neuter(),
};

const ADJECTIVE_NEUTER_QUESTION_PLURAL: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: ADJECTIVE_NEUTER_QUESTION,
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_neuter(),
};

const ADJECTIVE_PLURAL_QUESTION: &str = "Write down the plural form of this word";

const ADJECTIVE_PLURAL_QUESTION_BASE: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: ADJECTIVE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_plural(),
};

const ADJECTIVE_PLURAL_QUESTION_NEUTER: QuestionTemplate<AdjectiveSpecific> = QuestionTemplate {
    question: ADJECTIVE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_neuter(),
    answer: |word_forms| word_forms.get_plural(),
};

const ADJECTIVE_QUESTIONS: [QuestionTemplate<AdjectiveSpecific>; 8] = [
    ADJECTIVE_TRANSLATION_QUESTION_E,
    ADJECTIVE_TRANSLATION_QUESTION_S,
    ADJECTIVE_BASE_QUESTION_NEUTER,
    ADJECTIVE_BASE_QUESTION_PLURAL,
    ADJECTIVE_NEUTER_QUESTION_BASE,
    ADJECTIVE_NEUTER_QUESTION_PLURAL,
    ADJECTIVE_PLURAL_QUESTION_BASE,
    ADJECTIVE_PLURAL_QUESTION_NEUTER,
];

const VERB_TRANSLATION_QUESTION_E: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const VERB_TRANSLATION_QUESTION_S: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_BASE_QUESTION_PRESENT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_BASE_QUESTION_PAST: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_BASE_QUESTION_PERFECT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_word(),
};

const VERB_PRESENT_QUESTION: &str = "Write down the present form of this word";

const VERB_PRESENT_QUESTION_BASE: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_present(),
};

const VERB_PRESENT_QUESTION_PAST: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_present(),
};

const VERB_PRESENT_QUESTION_PERFECT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_present(),
};

const VERB_PAST_QUESTION: &str = "Write down the past form of this word";

const VERB_PAST_QUESTION_BASE: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_past(),
};

const VERB_PAST_QUESTION_PRESENT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_past(),
};

const VERB_PAST_QUESTION_PERFECT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_past(),
};

const VERB_PERFECT_QUESTION: &str = "Write down the perfect form of this word";

const VERB_PERFECT_QUESTION_BASE: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_perfect(),
};

const VERB_PERFECT_QUESTION_PRESENT: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_perfect(),
};

const VERB_PERFECT_QUESTION_PAST: QuestionTemplate<VerbSpecific> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_perfect(),
};

const VERB_QUESTIONS: [QuestionTemplate<VerbSpecific>; 14] = [
    VERB_TRANSLATION_QUESTION_E,
    VERB_TRANSLATION_QUESTION_S,
    VERB_BASE_QUESTION_PRESENT,
    VERB_BASE_QUESTION_PAST,
    VERB_BASE_QUESTION_PERFECT,
    VERB_PRESENT_QUESTION_BASE,
    VERB_PRESENT_QUESTION_PAST,
    VERB_PRESENT_QUESTION_PERFECT,
    VERB_PAST_QUESTION_BASE,
    VERB_PAST_QUESTION_PRESENT,
    VERB_PAST_QUESTION_PERFECT,
    VERB_PERFECT_QUESTION_BASE,
    VERB_PERFECT_QUESTION_PRESENT,
    VERB_PERFECT_QUESTION_PAST,
];

const ADVERB_TRANSLATION_QUESTION_E: QuestionTemplate<AdverbSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
};

const ADVERB_TRANSLATION_QUESTION_S: QuestionTemplate<AdverbSpecific> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
};

const ADVERB_QUESTIONS: [QuestionTemplate<AdverbSpecific>; 2] =
    [ADVERB_TRANSLATION_QUESTION_E, ADVERB_TRANSLATION_QUESTION_S];

const PER_PRONOUN_TRANSLATION_QUESTION_E: QuestionTemplate<PersonalPronounSpecific> =
    QuestionTemplate {
        question: TRANSLATION_QUESTION_E,
        base: |word_forms| word_forms.get_word(),
        answer: |word_forms| word_forms.get_translation(),
    };

const PER_PRONOUN_TRANSLATION_QUESTION_S: QuestionTemplate<PersonalPronounSpecific> =
    QuestionTemplate {
        question: TRANSLATION_QUESTION_S,
        base: |word_forms| word_forms.get_translation(),
        answer: |word_forms| word_forms.get_word(),
    };

const PER_PRONOUN_BASE_QUESTION_OBJECT: QuestionTemplate<PersonalPronounSpecific> =
    QuestionTemplate {
        question: "Write down the subjective form of this word",
        base: |word_forms| word_forms.get_object(),
        answer: |word_forms| word_forms.get_word(),
    };

const PER_PRONOUN_OBJECT_QUESTION_BASE: QuestionTemplate<PersonalPronounSpecific> =
    QuestionTemplate {
        question: "Write down the objective form of this word",
        base: |word_forms| word_forms.get_word(),
        answer: |word_forms| word_forms.get_object(),
    };

const PRONOUN_QUESTIONS: [QuestionTemplate<PersonalPronounSpecific>; 4] = [
    PER_PRONOUN_TRANSLATION_QUESTION_E,
    PER_PRONOUN_TRANSLATION_QUESTION_S,
    PER_PRONOUN_BASE_QUESTION_OBJECT,
    PER_PRONOUN_OBJECT_QUESTION_BASE,
];

#[derive(PartialEq, Debug)]
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
}

impl Question {
    pub fn get_question_by_word(word: &Word) -> Question {
        match word {
            Word::Noun(noun) => {
                let template = pick_question(&NOUN_QUESTIONS);
                template.question_from_template(noun)
            }
            Word::Adjective(adj) => {
                let template = pick_question(&ADJECTIVE_QUESTIONS);
                template.question_from_template(adj)
            }
            Word::Verb(verb) => {
                let template = pick_question(&VERB_QUESTIONS);
                template.question_from_template(verb)
            }
            Word::PersonalPronoun(pr) => {
                let template = pick_question(&PRONOUN_QUESTIONS);
                template.question_from_template(pr)
            }
            Word::Adverb(adv) => {
                let template = pick_question(&ADVERB_QUESTIONS);
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

fn pick_question<'a, T>(arr: &'a [QuestionTemplate<T>]) -> &'a QuestionTemplate<'a, T> {
    let rand = rand::rng().random_range(0..arr.len());
    &arr[rand]
}

#[cfg(test)]
mod test {
    use super::super::WordGeneral;
    use super::*;

    fn setup_noun() -> WordForms<NounSpecific> {
        WordForms {
            general: WordGeneral {
                word: String::from("word_noun"),
                translation: String::from("translation_noun"),
            },
            specific: NounSpecific {
                definite_singular: String::from("definite_singular"),
                indefinite_plural: String::from("indefinite_plural"),
                definite_plural: String::from("definite_plural"),
            },
        }
    }

    fn setup_adjective() -> WordForms<AdjectiveSpecific> {
        WordForms {
            general: WordGeneral {
                word: String::from("word_adj"),
                translation: String::from("translation_adj"),
            },
            specific: AdjectiveSpecific {
                neuter: String::from("neuter"),
                plural: String::from("plural"),
            },
        }
    }

    fn setup_verb() -> WordForms<VerbSpecific> {
        WordForms {
            general: WordGeneral {
                word: String::from("word_verb"),
                translation: String::from("translation_verb"),
            },
            specific: VerbSpecific {
                present: String::from("present"),
                past: String::from("past"),
                perfect: String::from("perfect"),
            },
        }
    }

    fn setup_adverb() -> WordForms<AdverbSpecific> {
        WordForms {
            general: WordGeneral {
                word: String::from("word_adv"),
                translation: String::from("translation_adv"),
            },
            specific: AdverbSpecific,
        }
    }

    fn setup_personal_pronoun() -> WordForms<PersonalPronounSpecific> {
        WordForms {
            general: WordGeneral {
                word: String::from("word_pr"),
                translation: String::from("translation_pr"),
            },
            specific: PersonalPronounSpecific {
                object: String::from("object"),
            },
        }
    }

    fn test_template<T>(
        exp_base: &str,
        exp_answer: &str,
        template: &QuestionTemplate<T>,
        forms: &WordForms<T>,
    ) {
        let expected = Question {
            question: String::from(""),
            base: String::from(exp_base),
            answer: String::from(exp_answer),
        };
        let actual = template.question_from_template(forms);
        assert_eq!(expected.base, actual.base);
        assert_eq!(expected.answer, actual.answer);
    }

    #[test]
    fn test_noun_translation_english_q() {
        let noun = setup_noun();
        println!("noun base from Swedish to English");
        test_template(
            "word_noun",
            "translation_noun",
            &NOUN_TRANSLATION_QUESTION_E,
            &noun,
        );
    }

    #[test]
    fn test_noun_translation_swedish_q() {
        let noun = setup_noun();
        println!("noun base from English to Swedish");
        test_template(
            "translation_noun",
            "word_noun",
            &NOUN_TRANSLATION_QUESTION_S,
            &noun,
        );
    }

    #[test]
    fn test_noun_base_question_definite_plural() {
        let noun = setup_noun();
        println!("noun base from definite plural");
        test_template(
            "definite_plural",
            "word_noun",
            &NOUN_BASE_QUESTION_DEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_noun_base_question_indefinite_plural() {
        let noun = setup_noun();
        println!("noun base from indefinite plural");
        test_template(
            "indefinite_plural",
            "word_noun",
            &NOUN_BASE_QUESTION_INDEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_noun_base_question_definite_singular() {
        let noun = setup_noun();
        println!("noun base from definite singular");
        test_template(
            "definite_singular",
            "word_noun",
            &NOUN_BASE_QUESTION_DEFINITE_SINGULAR,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_plural_question_base() {
        let noun = setup_noun();
        println!("noun definite plural from base");
        test_template(
            "word_noun",
            "definite_plural",
            &NOUN_DEFINITE_PLURAL_QUESTION_BASE,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_plural_question_indefinite_plural() {
        let noun = setup_noun();
        println!("noun definite plural from indefinite plural");
        test_template(
            "indefinite_plural",
            "definite_plural",
            &NOUN_DEFINITE_PLURAL_QUESTION_INDEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_plural_question_definite_singular() {
        let noun = setup_noun();
        println!("noun definite plural from definite singular");
        test_template(
            "definite_singular",
            "definite_plural",
            &NOUN_DEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR,
            &noun,
        );
    }

    #[test]
    fn test_noun_indefinite_plural_question_base() {
        let noun = setup_noun();
        println!("noun indefinite plural from base");
        test_template(
            "word_noun",
            "indefinite_plural",
            &NOUN_INDEFINITE_PLURAL_QUESTION_BASE,
            &noun,
        );
    }

    #[test]
    fn test_noun_indefinite_plural_question_definite_plural() {
        let noun = setup_noun();
        println!("noun indefinite plural from definite plural");
        test_template(
            "definite_plural",
            "indefinite_plural",
            &NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_noun_indefinite_plural_question_definite_singular() {
        let noun = setup_noun();
        println!("noun indefinite plural from definite singular");
        test_template(
            "definite_singular",
            "indefinite_plural",
            &NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_singular_question_base() {
        let noun = setup_noun();
        println!("noun definite singular from base form");
        test_template(
            "word_noun",
            "definite_singular",
            &NOUN_DEFINITE_SINGULAR_QUESTION_BASE,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_singular_question_definite_plural() {
        let noun = setup_noun();
        println!("noun definite singular from definite plural");
        test_template(
            "definite_plural",
            "definite_singular",
            &NOUN_DEFINITE_SINGULAR_QUESTION_DEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_noun_definite_singular_question_indefinite_plural() {
        let noun = setup_noun();
        println!("noun definite singular from indefinite_plural");
        test_template(
            "indefinite_plural",
            "definite_singular",
            &NOUN_DEFINITE_SINGULAR_QUESTION_INDEFINITE_PLURAL,
            &noun,
        );
    }

    #[test]
    fn test_adj_translation_english_q() {
        let adj = setup_adjective();
        println!("adjective base from Swedish to English");
        test_template(
            "word_adj",
            "translation_adj",
            &ADJECTIVE_TRANSLATION_QUESTION_E,
            &adj,
        );
    }

    #[test]
    fn test_adj_translation_swedish_q() {
        let adj = setup_adjective();
        println!("adjective base from English to Swedish");
        test_template(
            "translation_adj",
            "word_adj",
            &ADJECTIVE_TRANSLATION_QUESTION_S,
            &adj,
        );
    }

    #[test]
    fn test_adj_base_question_neuter() {
        let adj = setup_adjective();
        println!("adjective base from neuter");
        test_template("neuter", "word_adj", &ADJECTIVE_BASE_QUESTION_NEUTER, &adj);
    }

    #[test]
    fn test_adj_base_question_plural() {
        let adj = setup_adjective();
        println!("adjective base from plural");
        test_template("plural", "word_adj", &ADJECTIVE_BASE_QUESTION_PLURAL, &adj);
    }

    #[test]
    fn test_adj_neuter_question_base() {
        let adj = setup_adjective();
        println!("adjective neuter from base");
        test_template("word_adj", "neuter", &ADJECTIVE_NEUTER_QUESTION_BASE, &adj);
    }

    #[test]
    fn test_adj_neuter_question_plural() {
        let adj = setup_adjective();
        println!("adjective neuter from plural");
        test_template("plural", "neuter", &ADJECTIVE_NEUTER_QUESTION_PLURAL, &adj);
    }

    #[test]
    fn test_adj_plural_quesiton_base() {
        let adj = setup_adjective();
        println!("adjective plural from base");
        test_template("word_adj", "plural", &ADJECTIVE_PLURAL_QUESTION_BASE, &adj);
    }

    #[test]
    fn test_adj_plural_question_neuter() {
        let adj = setup_adjective();
        println!("adjective plural from neuter");
        test_template("neuter", "plural", &ADJECTIVE_PLURAL_QUESTION_NEUTER, &adj);
    }

    #[test]
    fn test_verb_translation_english_q() {
        let verb = setup_verb();
        println!("verb base from Swedish to English");
        test_template(
            "word_verb",
            "translation_verb",
            &VERB_TRANSLATION_QUESTION_E,
            &verb,
        );
    }

    #[test]
    fn test_verb_translation_swedish_q() {
        let verb = setup_verb();
        println!("verb base from English to Swedish");
        test_template(
            "translation_verb",
            "word_verb",
            &VERB_TRANSLATION_QUESTION_S,
            &verb,
        );
    }

    #[test]
    fn test_verb_base_question_present() {
        let verb = setup_verb();
        println!("verb base from present");
        test_template("present", "word_verb", &VERB_BASE_QUESTION_PRESENT, &verb);
    }

    #[test]
    fn test_verb_base_question_past() {
        let verb = setup_verb();
        println!("verb base from past");
        test_template("past", "word_verb", &VERB_BASE_QUESTION_PAST, &verb);
    }

    #[test]
    fn test_verb_base_question_perfect() {
        let verb = setup_verb();
        println!("verb base from perfect");
        test_template("perfect", "word_verb", &VERB_BASE_QUESTION_PERFECT, &verb);
    }

    #[test]
    fn test_verb_present_question_base() {
        let verb = setup_verb();
        println!("verb present from base");
        test_template("word_verb", "present", &VERB_PRESENT_QUESTION_BASE, &verb);
    }

    #[test]
    fn test_verb_present_question_past() {
        let verb = setup_verb();
        println!("verb present from past");
        test_template("past", "present", &VERB_PRESENT_QUESTION_PAST, &verb);
    }

    #[test]
    fn test_verb_present_question_perfect() {
        let verb = setup_verb();
        println!("verb present from perfect");
        test_template("perfect", "present", &VERB_PRESENT_QUESTION_PERFECT, &verb);
    }

    #[test]
    fn test_verb_past_question_base() {
        let verb = setup_verb();
        println!("verb past from base");
        test_template("word_verb", "past", &VERB_PAST_QUESTION_BASE, &verb);
    }

    #[test]
    fn test_verb_past_question_present() {
        let verb = setup_verb();
        println!("verb past from present");
        test_template("present", "past", &VERB_PAST_QUESTION_PRESENT, &verb);
    }

    #[test]
    fn test_verb_past_question_perfect() {
        let verb = setup_verb();
        println!("verb past from perfect");
        test_template("perfect", "past", &VERB_PAST_QUESTION_PERFECT, &verb);
    }

    #[test]
    fn test_verb_perfect_question_base() {
        let verb = setup_verb();
        println!("verb perfect from base");
        test_template("word_verb", "perfect", &VERB_PERFECT_QUESTION_BASE, &verb);
    }

    #[test]
    fn test_verb_perfect_question_present() {
        let verb = setup_verb();
        println!("verb perfect from present");
        test_template("present", "perfect", &VERB_PERFECT_QUESTION_PRESENT, &verb);
    }

    #[test]
    fn test_verb_perfect_question_past() {
        let verb = setup_verb();
        println!("verb perfect from past");
        test_template("past", "perfect", &VERB_PERFECT_QUESTION_PAST, &verb);
    }

    #[test]
    fn test_adv_translation_english_q() {
        let adv = setup_adverb();
        println!("adverb from Swedish to English");
        test_template(
            "word_adv",
            "translation_adv",
            &ADVERB_TRANSLATION_QUESTION_E,
            &adv,
        );
    }

    #[test]
    fn test_adv_translation_swedish_q() {
        let adv = setup_adverb();
        println!("adverb from English to Swedish");
        test_template(
            "translation_adv",
            "word_adv",
            &ADVERB_TRANSLATION_QUESTION_S,
            &adv,
        );
    }

    #[test]
    fn test_p_pr_translation_english_q() {
        let pr = setup_personal_pronoun();
        println!("personal pronoun subjective from Swedish to English");
        test_template(
            "word_pr",
            "translation_pr",
            &PER_PRONOUN_TRANSLATION_QUESTION_E,
            &pr,
        );
    }

    #[test]
    fn test_p_pr_translation_swedish_q() {
        let pr = setup_personal_pronoun();
        println!("personal pronoun subjective from English to Swedish");
        test_template(
            "translation_pr",
            "word_pr",
            &PER_PRONOUN_TRANSLATION_QUESTION_S,
            &pr,
        );
    }

    #[test]
    fn test_p_pr_base_question_object() {
        let pr = setup_personal_pronoun();
        println!("personal pronoun from objective to subjective");
        test_template("object", "word_pr", &PER_PRONOUN_BASE_QUESTION_OBJECT, &pr);
    }

    #[test]
    fn test_p_pr_object_quesiton_base() {
        let pr = setup_personal_pronoun();
        println!("personal pronoun from subjective to objective");
        test_template("word_pr", "object", &PER_PRONOUN_OBJECT_QUESTION_BASE, &pr);
    }

    #[test]
    fn picked_question_is_from_array() {
        for _ in 1..100 {
            let q = pick_question(&NOUN_QUESTIONS);
            assert!(
                NOUN_QUESTIONS.contains(q),
                "picked noun question from another collection"
            );

            let q = pick_question(&ADJECTIVE_QUESTIONS);
            assert!(
                ADJECTIVE_QUESTIONS.contains(q),
                "picked adjective question from another collection"
            );

            let q = pick_question(&VERB_QUESTIONS);
            assert!(
                VERB_QUESTIONS.contains(q),
                "picked verb question from another collection"
            );

            let q = pick_question(&ADVERB_QUESTIONS);
            assert!(
                ADVERB_QUESTIONS.contains(q),
                "picked adverb question from another collection"
            );

            let q = pick_question(&PRONOUN_QUESTIONS);
            assert!(
                PRONOUN_QUESTIONS.contains(q),
                "picked pronoun question from another collection"
            );
        }
    }
}
