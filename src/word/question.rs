//! The collection of all available quiz questions.

use super::{Adjective, Adverb, Forms, Noun, PersonalPronoun, Verb};
use categories::{Category, Property};
use rand::seq::IteratorRandom;

pub mod categories {
    pub enum Property {
        EnglishToSwedishTranslation,
        SwedishToEnglishTranslation,
        SwedishFormToSwedishForm,
    }

    struct PropertyFilter {
        filter: fn(&Property) -> bool,
    }

    const E_T_S_FILTER: PropertyFilter = PropertyFilter {
        filter: |property| matches!(property, Property::EnglishToSwedishTranslation),
    };

    const S_T_E_FILTER: PropertyFilter = PropertyFilter {
        filter: |property| matches!(property, Property::SwedishToEnglishTranslation),
    };

    const BLANK_FILTER: PropertyFilter = PropertyFilter {
        filter: |_property| true,
    };

    pub struct Category {
        apply: fn(&Property) -> bool,
    }

    pub const CATEGORY_BEGINNER: Category = Category {
        apply: |property| (S_T_E_FILTER.filter)(property),
    };

    pub const CATEGORY_ADVANCED: Category = Category {
        apply: |property| (E_T_S_FILTER.filter)(property) || (S_T_E_FILTER.filter)(property),
    };

    pub const CATEGORY_PROFICIENT: Category = Category {
        apply: |property| (BLANK_FILTER.filter)(property),
    };

    impl Category {
        pub fn apply(&self, property: &Property) -> bool {
            (self.apply)(property)
        }
    }
}

const TRANSLATION_QUESTION_E: &str = "Translate this word in English";
const TRANSLATION_QUESTION_S: &str = "Translate this word in Swedish";
const BASE_QUESTION: &str = "Write down the base form of this word";

const NOUN_TRANSLATION_QUESTION_E: QuestionTemplate<Noun> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |noun| noun.get_word(),
    answer: |noun| noun.get_translation(),
    property: Property::SwedishToEnglishTranslation,
};

const NOUN_TRANSLATION_QUESTION_S: QuestionTemplate<Noun> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::EnglishToSwedishTranslation,
};

const NOUN_BASE_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<Noun> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_indefinite_plural(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_BASE_QUESTION_DEFINITE_PLURAL: QuestionTemplate<Noun> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_definite_plural(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_BASE_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<Noun> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_definite_singular(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_INDEFINITE_PLURAL_QUESTION: &str = "Write down the indefinite plural form of this word";

const NOUN_INDEFINITE_PLURAL_QUESTION_BASE: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_INDEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_indefinite_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<Noun> =
    QuestionTemplate {
        question: NOUN_INDEFINITE_PLURAL_QUESTION,
        base: |word_forms| word_forms.get_definite_singular(),
        answer: |word_forms| word_forms.get_indefinite_plural(),
        property: Property::SwedishFormToSwedishForm,
    };

const NOUN_INDEFINITE_PLURAL_QUESTION_DEFINITE_PLURAL: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_INDEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_definite_plural(),
    answer: |word_forms| word_forms.get_indefinite_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_DEFINITE_SINGULAR_QUESTION: &str = "Write down the definite singular form of this word";

const NOUN_DEFINITE_SINGULAR_QUESTION_BASE: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_DEFINITE_SINGULAR_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_definite_singular(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_DEFINITE_SINGULAR_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<Noun> =
    QuestionTemplate {
        question: NOUN_DEFINITE_SINGULAR_QUESTION,
        base: |word_forms| word_forms.get_indefinite_plural(),
        answer: |word_forms| word_forms.get_definite_singular(),
        property: Property::SwedishFormToSwedishForm,
    };

const NOUN_DEFINITE_SINGULAR_QUESTION_DEFINITE_PLURAL: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_DEFINITE_SINGULAR_QUESTION,
    base: |word_forms| word_forms.get_definite_plural(),
    answer: |word_forms| word_forms.get_definite_singular(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_DEFINITE_PLURAL_QUESTION: &str = "Write down the definite plural form of this word";

const NOUN_DEFINITE_PLURAL_QUESTION_BASE: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_DEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_definite_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_DEFINITE_PLURAL_QUESTION_INDEFINITE_PLURAL: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_DEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_indefinite_plural(),
    answer: |word_forms| word_forms.get_definite_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_DEFINITE_PLURAL_QUESTION_DEFINITE_SINGULAR: QuestionTemplate<Noun> = QuestionTemplate {
    question: NOUN_DEFINITE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_definite_singular(),
    answer: |word_forms| word_forms.get_definite_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const NOUN_QUESTIONS: [QuestionTemplate<Noun>; 14] = [
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

const ADJECTIVE_TRANSLATION_QUESTION_E: QuestionTemplate<Adjective> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
    property: Property::SwedishToEnglishTranslation,
};

const ADJECTIVE_TRANSLATION_QUESTION_S: QuestionTemplate<Adjective> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::EnglishToSwedishTranslation,
};

const ADJECTIVE_BASE_QUESTION_NEUTER: QuestionTemplate<Adjective> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_neuter(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_BASE_QUESTION_PLURAL: QuestionTemplate<Adjective> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_NEUTER_QUESTION: &str = "Write down the neuter form of this word";

const ADJECTIVE_NEUTER_QUESTION_BASE: QuestionTemplate<Adjective> = QuestionTemplate {
    question: ADJECTIVE_NEUTER_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_neuter(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_NEUTER_QUESTION_PLURAL: QuestionTemplate<Adjective> = QuestionTemplate {
    question: ADJECTIVE_NEUTER_QUESTION,
    base: |word_forms| word_forms.get_plural(),
    answer: |word_forms| word_forms.get_neuter(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_PLURAL_QUESTION: &str = "Write down the plural form of this word";

const ADJECTIVE_PLURAL_QUESTION_BASE: QuestionTemplate<Adjective> = QuestionTemplate {
    question: ADJECTIVE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_PLURAL_QUESTION_NEUTER: QuestionTemplate<Adjective> = QuestionTemplate {
    question: ADJECTIVE_PLURAL_QUESTION,
    base: |word_forms| word_forms.get_neuter(),
    answer: |word_forms| word_forms.get_plural(),
    property: Property::SwedishFormToSwedishForm,
};

const ADJECTIVE_QUESTIONS: [QuestionTemplate<Adjective>; 8] = [
    ADJECTIVE_TRANSLATION_QUESTION_E,
    ADJECTIVE_TRANSLATION_QUESTION_S,
    ADJECTIVE_BASE_QUESTION_NEUTER,
    ADJECTIVE_BASE_QUESTION_PLURAL,
    ADJECTIVE_NEUTER_QUESTION_BASE,
    ADJECTIVE_NEUTER_QUESTION_PLURAL,
    ADJECTIVE_PLURAL_QUESTION_BASE,
    ADJECTIVE_PLURAL_QUESTION_NEUTER,
];

const VERB_TRANSLATION_QUESTION_E: QuestionTemplate<Verb> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
    property: Property::SwedishToEnglishTranslation,
};

const VERB_TRANSLATION_QUESTION_S: QuestionTemplate<Verb> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::EnglishToSwedishTranslation,
};

const VERB_BASE_QUESTION_PRESENT: QuestionTemplate<Verb> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_BASE_QUESTION_PAST: QuestionTemplate<Verb> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_BASE_QUESTION_PERFECT: QuestionTemplate<Verb> = QuestionTemplate {
    question: BASE_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PRESENT_QUESTION: &str = "Write down the present form of this word";

const VERB_PRESENT_QUESTION_BASE: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_present(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PRESENT_QUESTION_PAST: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_present(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PRESENT_QUESTION_PERFECT: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PRESENT_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_present(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PAST_QUESTION: &str = "Write down the past form of this word";

const VERB_PAST_QUESTION_BASE: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_past(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PAST_QUESTION_PRESENT: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_past(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PAST_QUESTION_PERFECT: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PAST_QUESTION,
    base: |word_forms| word_forms.get_perfect(),
    answer: |word_forms| word_forms.get_past(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PERFECT_QUESTION: &str = "Write down the perfect form of this word";

const VERB_PERFECT_QUESTION_BASE: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_perfect(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PERFECT_QUESTION_PRESENT: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_present(),
    answer: |word_forms| word_forms.get_perfect(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_PERFECT_QUESTION_PAST: QuestionTemplate<Verb> = QuestionTemplate {
    question: VERB_PERFECT_QUESTION,
    base: |word_forms| word_forms.get_past(),
    answer: |word_forms| word_forms.get_perfect(),
    property: Property::SwedishFormToSwedishForm,
};

const VERB_QUESTIONS: [QuestionTemplate<Verb>; 14] = [
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

const ADVERB_TRANSLATION_QUESTION_E: QuestionTemplate<Adverb> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
    property: Property::SwedishToEnglishTranslation,
};

const ADVERB_TRANSLATION_QUESTION_S: QuestionTemplate<Adverb> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::EnglishToSwedishTranslation,
};

const ADVERB_QUESTIONS: [QuestionTemplate<Adverb>; 2] =
    [ADVERB_TRANSLATION_QUESTION_E, ADVERB_TRANSLATION_QUESTION_S];

const PER_PRONOUN_TRANSLATION_QUESTION_E: QuestionTemplate<PersonalPronoun> = QuestionTemplate {
    question: TRANSLATION_QUESTION_E,
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_translation(),
    property: Property::SwedishToEnglishTranslation,
};

const PER_PRONOUN_TRANSLATION_QUESTION_S: QuestionTemplate<PersonalPronoun> = QuestionTemplate {
    question: TRANSLATION_QUESTION_S,
    base: |word_forms| word_forms.get_translation(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::EnglishToSwedishTranslation,
};

const PER_PRONOUN_BASE_QUESTION_OBJECT: QuestionTemplate<PersonalPronoun> = QuestionTemplate {
    question: "Write down the subjective form of this word",
    base: |word_forms| word_forms.get_object(),
    answer: |word_forms| word_forms.get_word(),
    property: Property::SwedishFormToSwedishForm,
};

const PER_PRONOUN_OBJECT_QUESTION_BASE: QuestionTemplate<PersonalPronoun> = QuestionTemplate {
    question: "Write down the objective form of this word",
    base: |word_forms| word_forms.get_word(),
    answer: |word_forms| word_forms.get_object(),
    property: Property::SwedishFormToSwedishForm,
};

const PRONOUN_QUESTIONS: [QuestionTemplate<PersonalPronoun>; 4] = [
    PER_PRONOUN_TRANSLATION_QUESTION_E,
    PER_PRONOUN_TRANSLATION_QUESTION_S,
    PER_PRONOUN_BASE_QUESTION_OBJECT,
    PER_PRONOUN_OBJECT_QUESTION_BASE,
];

struct QuestionTemplate<'a, T: Forms> {
    question: &'a str,
    base: fn(&T) -> &str,
    answer: fn(&T) -> &str,
    property: Property,
}

pub struct Question {
    question: String,
    base: String,
    answer: String,
}

impl<'a, T> QuestionTemplate<'_, T>
where
    T: Forms,
{
    fn random_template(
        arr: &'a [QuestionTemplate<T>],
        category: &'a Category,
    ) -> &'a QuestionTemplate<'a, T> {
        arr.iter()
            .filter(|template| category.apply(&template.property))
            .choose(&mut rand::rng())
            .expect("Empty question array")
    }

    fn question_from_template(&self, forms: &T) -> Question {
        Question {
            question: String::from(self.question),
            base: String::from((self.base)(forms)),
            answer: String::from((self.answer)(forms)),
        }
    }
}

impl Question {
    /// Get the question text.
    pub fn get_question(&self) -> &str {
        &self.question
    }

    /// Get the word form this question is based on.
    pub fn get_base(&self) -> &str {
        &self.base
    }

    /// Get the expected word form
    pub fn get_answer(&self) -> &str {
        &self.answer
    }
}

pub trait Pick: super::Forms {
    fn get_question(&self, category: &Category) -> Question;
}

impl Pick for Noun {
    fn get_question(&self, category: &Category) -> Question {
        QuestionTemplate::random_template(&NOUN_QUESTIONS, category).question_from_template(&self)
    }
}

impl Pick for Adjective {
    fn get_question(&self, category: &Category) -> Question {
        QuestionTemplate::random_template(&ADJECTIVE_QUESTIONS, category)
            .question_from_template(&self)
    }
}

impl Pick for Verb {
    fn get_question(&self, category: &Category) -> Question {
        QuestionTemplate::random_template(&VERB_QUESTIONS, category).question_from_template(&self)
    }
}

impl Pick for Adverb {
    fn get_question(&self, category: &Category) -> Question {
        QuestionTemplate::random_template(&ADVERB_QUESTIONS, category).question_from_template(&self)
    }
}

impl Pick for PersonalPronoun {
    fn get_question(&self, category: &Category) -> Question {
        QuestionTemplate::random_template(&PRONOUN_QUESTIONS, category)
            .question_from_template(&self)
    }
}

#[cfg(test)]
mod test {
    use super::{
        categories::{CATEGORY_ADVANCED, CATEGORY_BEGINNER, CATEGORY_PROFICIENT},
        *,
    };

    fn setup_noun() -> Noun {
        Noun {
            word: String::from("word_noun"),
            translation: String::from("translation_noun"),
            definite_singular: String::from("definite_singular"),
            indefinite_plural: String::from("indefinite_plural"),
            definite_plural: String::from("definite_plural"),
        }
    }

    fn setup_adjective() -> Adjective {
        Adjective {
            word: String::from("word_adj"),
            translation: String::from("translation_adj"),

            neuter: String::from("neuter"),
            plural: String::from("plural"),
        }
    }

    fn setup_verb() -> Verb {
        Verb {
            word: String::from("word_verb"),
            translation: String::from("translation_verb"),
            present: String::from("present"),
            past: String::from("past"),
            perfect: String::from("perfect"),
        }
    }

    fn setup_adverb() -> Adverb {
        Adverb {
            word: String::from("word_adv"),
            translation: String::from("translation_adv"),
        }
    }

    fn setup_personal_pronoun() -> PersonalPronoun {
        PersonalPronoun {
            word: String::from("word_pr"),
            translation: String::from("translation_pr"),
            object: String::from("object"),
        }
    }

    fn test_template<T>(exp_base: &str, exp_answer: &str, template: &QuestionTemplate<T>, forms: &T)
    where
        T: Forms,
    {
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
    fn test_pick_beginner() {
        let noun = setup_noun();
        let adj = setup_adjective();
        let verb = setup_verb();
        let adv = setup_adverb();
        let pr = setup_personal_pronoun();
        let category = CATEGORY_BEGINNER;

        noun.get_question(&category);
        adj.get_question(&category);
        verb.get_question(&category);
        adv.get_question(&category);
        pr.get_question(&category);
    }

    #[test]
    fn test_pick_advanced() {
        let noun = setup_noun();
        let adj = setup_adjective();
        let verb = setup_verb();
        let adv = setup_adverb();
        let pr = setup_personal_pronoun();
        let category = CATEGORY_ADVANCED;

        noun.get_question(&category);
        adj.get_question(&category);
        verb.get_question(&category);
        adv.get_question(&category);
        pr.get_question(&category);
    }

    #[test]
    fn test_pick_proficient() {
        let noun = setup_noun();
        let adj = setup_adjective();
        let verb = setup_verb();
        let adv = setup_adverb();
        let pr = setup_personal_pronoun();
        let category = CATEGORY_PROFICIENT;

        noun.get_question(&category);
        adj.get_question(&category);
        verb.get_question(&category);
        adv.get_question(&category);
        pr.get_question(&category);
    }
}
