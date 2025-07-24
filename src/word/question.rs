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
            Word::NOUN(noun) => {
                let template = pick_question(&NOUN_QUESTIONS);
                template.question_from_template(noun)
            }
            Word::ADJECTIVE(adj) => {
                let template = pick_question(&ADJECTIVE_QUESTIONS);
                template.question_from_template(adj)
            }
            Word::VERB(verb) => {
                let template = pick_question(&VERB_QUESTIONS);
                template.question_from_template(verb)
            }
            Word::PRONOUN(pr) => {
                let template = pick_question(&PRONOUN_QUESTIONS);
                template.question_from_template(pr)
            }
            Word::ADVERB(adv) => {
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

    fn setup() -> (
        WordForms<NounSpecific>,
        WordForms<AdjectiveSpecific>,
        WordForms<VerbSpecific>,
        WordForms<AdverbSpecific>,
        WordForms<PronounSpecific>,
    ) {
        let noun = WordForms {
            general: WordGeneral {
                word: String::from("word_noun"),
                translation: String::from("translation_noun"),
            },
            specific: NounSpecific {
                definite: String::from("definite"),
                plural: String::from("plural_noun"),
            },
        };
        let adj = WordForms {
            general: WordGeneral {
                word: String::from("word_adj"),
                translation: String::from("translation_adj"),
            },
            specific: AdjectiveSpecific {
                neuter: String::from("neuter"),
                plural: String::from("plural_adj"),
            },
        };
        let verb = WordForms {
            general: WordGeneral {
                word: String::from("word_verb"),
                translation: String::from("translation_verb"),
            },
            specific: VerbSpecific {
                present: String::from("present"),
                past: String::from("past"),
            },
        };
        let adv = WordForms {
            general: WordGeneral {
                word: String::from("word_adv"),
                translation: String::from("translation_adv"),
            },
            specific: AdverbSpecific,
        };
        let pr = WordForms {
            general: WordGeneral {
                word: String::from("word_pr"),
                translation: String::from("translation_pr"),
            },
            specific: PronounSpecific {
                possessive: String::from("possessive"),
            },
        };

        (noun, adj, verb, adv, pr)
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
    fn noun_translation_english_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun base from Swedish to English");
        test_template(
            "word_noun",
            "translation_noun",
            &NOUN_TRANSLATION_QUESTION_E,
            &noun,
        );
    }

    #[test]
    fn noun_translation_swedish_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun base from English to Swedish");
        test_template(
            "translation_noun",
            "word_noun",
            &NOUN_TRANSLATION_QUESTION_S,
            &noun,
        );
    }

    #[test]
    fn noun_definite_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun from base to definite");
        test_template("word_noun", "definite", &NOUN_DEFINITE_QUESTION, &noun);
    }

    #[test]
    fn noun_indefinite_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun from definite to base");
        test_template("definite", "word_noun", &NOUN_INDEFINITE_QUESTION, &noun);
    }

    #[test]
    fn noun_plural_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun from base to plural");
        test_template("word_noun", "plural_noun", &NOUN_PLURAL_QUESTION, &noun);
    }

    #[test]
    fn noun_singular_q() {
        let (noun, _, _, _, _) = setup();
        println!("noun from plural to base");
        test_template("plural_noun", "word_noun", &NOUN_SINGULAR_QUESTION, &noun);
    }

    #[test]
    fn adj_translation_english_q() {
        let (_, adj, _, _, _) = setup();
        println!("adjective base from Swedish to English");
        test_template(
            "word_adj",
            "translation_adj",
            &ADJECTIVE_TRANSLATION_QUESTION_E,
            &adj,
        );
    }

    #[test]
    fn adj_translation_swedish_q() {
        let (_, adj, _, _, _) = setup();
        println!("adjective base from English to Swedish");
        test_template(
            "translation_adj",
            "word_adj",
            &ADJECTIVE_TRANSLATION_QUESTION_S,
            &adj,
        );
    }

    #[test]
    fn adj_neuter_q() {
        let (_, adj, _, _, _) = setup();
        println!("adjective from base to neuter");
        test_template("word_adj", "neuter", &ADJECTIVE_NEUTER_QUESTION, &adj);
    }

    #[test]
    fn adj_non_neuter_q() {
        let (_, adj, _, _, _) = setup();
        println!("adjective from neuter to base");
        test_template("neuter", "word_adj", &ADJECTIVE_NON_NEUTER_QUESTION, &adj);
    }

    #[test]
    fn adj_plural_q() {
        let (_, adj, _, _, _) = setup();
        println!("adjective from base to plural");
        test_template("word_adj", "plural_adj", &ADJECTIVE_PLURAL_QUESTION, &adj);
    }

    #[test]
    fn adj_singular_q() {
        let (_, adj, _, _, _) = setup();
        print!("adjective from plural to base");
        test_template("plural_adj", "word_adj", &ADJECTIVE_SINGULAR_QUESTION, &adj);
    }

    #[test]
    fn verb_translation_english_q() {
        let (_, _, verb, _, _) = setup();
        println!("verb base from Swedish to English");
        test_template(
            "word_verb",
            "translation_verb",
            &VERB_TRANSLATION_QUESTION_E,
            &verb,
        );
    }

    #[test]
    fn verb_translation_swedish_q() {
        let (_, _, verb, _, _) = setup();
        println!("verb base from English to Swedish");
        test_template(
            "translation_verb",
            "word_verb",
            &VERB_TRANSLATION_QUESTION_S,
            &verb,
        );
    }

    #[test]
    fn verb_present_q() {
        let (_, _, verb, _, _) = setup();
        println!("verb from infinitive to present");
        test_template("word_verb", "present", &VERB_PRESENT_QUESTION, &verb);
    }

    #[test]
    fn verb_past_q() {
        let (_, _, verb, _, _) = setup();
        println!("verb from infinitive to past");
        test_template("word_verb", "past", &VERB_PAST_QUESTION, &verb);
    }

    #[test]
    fn verb_infinitive_q() {
        let (_, _, verb, _, _) = setup();
        println!("verb form present to infinitive");
        test_template("present", "word_verb", &VERB_INFINITIVE_QUESTION, &verb);
    }

    #[test]
    fn adv_translation_english_q() {
        let (_, _, _, adv, _) = setup();
        println!("adverb from Swedish to English");
        test_template(
            "word_adv",
            "translation_adv",
            &ADVERB_TRANSLATION_QUESTION_E,
            &adv,
        );
    }

    #[test]
    fn adv_translation_swedish_q() {
        let (_, _, _, adv, _) = setup();
        println!("adverb from English to Swedish");
        test_template(
            "translation_adv",
            "word_adv",
            &ADVERB_TRANSLATION_QUESTION_S,
            &adv,
        );
    }

    #[test]
    fn pr_translation_english_q() {
        let (_, _, _, _, pr) = setup();
        println!("pronoun subjective from Swedish to English");
        test_template(
            "word_pr",
            "translation_pr",
            &PRONOUN_TRANSLATION_QUESTION_E,
            &pr,
        );
    }

    #[test]
    fn pr_translation_swedish_q() {
        let (_, _, _, _, pr) = setup();
        println!("pronoun subjective from English to Swedish");
        test_template(
            "translation_pr",
            "word_pr",
            &PRONOUN_TRANSLATION_QUESTION_S,
            &pr,
        );
    }

    #[test]
    fn pr_possessive_q() {
        let (_, _, _, _, pr) = setup();
        println!("pronoun from subjective to possessive");
        test_template("word_pr", "possessive", &PRONOUN_POSSESSIVE_QUESTION, &pr);
    }

    #[test]
    fn pr_subjective_q() {
        let (_, _, _, _, pr) = setup();
        println!("pronoun from possessive to subjective");
        test_template("possessive", "word_pr", &PRONOUN_SUBJECTIVE_QUESTION, &pr);
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
