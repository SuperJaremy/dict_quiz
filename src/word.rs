//! Grammatically inaccurate Swedish word classes.
//!
//! # Inaccuracies
//! Swedish grammar classfies words by their meaning and role in the
//! sentence. For us that means, that some words from one class
//! might have the different word forms than the others.
//! For example, numerals, conjunctions and some pronouns have the same number of forms
//! as adverbs. Possessive pronouns have the same forms as adjectives, while
//! personal pronouns have their own form called "objektsform".
//!
//! To avoid unnecessary duplucation, we only have next word classes:
//! - Nouns;
//! - Adjectives;
//! - Verbs;
//! - Adverbs;
//! - Personal pronouns.
//!
//! All other word classes should be assigned to present ones in
//! a case by case manner.

pub mod question;
pub mod word_csv;

/// Noun specific word forms
#[derive(PartialEq, Debug)]
pub struct NounSpecific {
    definite_singular: String,
    indefinite_plural: String,
    definite_plural: String,
}

/// Adjective specific word forms
#[derive(PartialEq, Debug)]
pub struct AdjectiveSpecific {
    neuter: String,
    plural: String,
}

/// Verb specific word forms
#[derive(PartialEq, Debug)]
pub struct VerbSpecific {
    present: String,
    past: String,
    perfect: String,
}

/// Adverb specific word forms
#[derive(PartialEq, Debug)]
pub struct AdverbSpecific;

/// Personal pronoun specific word forms
#[derive(PartialEq, Debug)]
pub struct PersonalPronounSpecific {
    object: String,
}

/// Word forms present for all classes
#[derive(PartialEq, Debug)]
pub struct WordGeneral {
    word: String,
    translation: String,
}

/// All word forms for a word
#[derive(PartialEq, Debug)]
pub struct WordForms<T> {
    general: WordGeneral,
    specific: T,
}

/// All existent word classes
#[derive(PartialEq, Debug)]
pub enum Word {
    Noun(WordForms<NounSpecific>),
    Adjective(WordForms<AdjectiveSpecific>),
    Verb(WordForms<VerbSpecific>),
    Adverb(WordForms<AdverbSpecific>),
    PersonalPronoun(WordForms<PersonalPronounSpecific>),
}

impl Word {
    fn new_noun(
        word: String,
        translation: String,
        definite_singular: String,
        indefinite_plural: String,
        definite_plural: String,
    ) -> Word {
        Word::Noun(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: NounSpecific {
                definite_singular: definite_singular,
                indefinite_plural: indefinite_plural,
                definite_plural: definite_plural,
            },
        })
    }

    fn new_adjective(word: String, translation: String, neuter: String, plural: String) -> Word {
        Word::Adjective(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: AdjectiveSpecific {
                neuter: neuter,
                plural: plural,
            },
        })
    }

    fn new_verb(
        word: String,
        translation: String,
        present: String,
        past: String,
        perfect: String,
    ) -> Word {
        Word::Verb(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: VerbSpecific {
                present: present,
                past: past,
                perfect: perfect,
            },
        })
    }

    fn new_adverb(word: String, translation: String) -> Word {
        Word::Adverb(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: AdverbSpecific,
        })
    }

    fn new_personal_pronoun(word: String, translation: String, object: String) -> Word {
        Word::PersonalPronoun(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: PersonalPronounSpecific { object: object },
        })
    }
    /// Get vector of tuples `(word_form_name, word_form)`.
    pub fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        match self {
            Word::Noun(noun) => noun.get_forms(),
            Word::Adjective(adj) => adj.get_forms(),
            Word::Verb(verb) => verb.get_forms(),
            Word::Adverb(adv) => adv.get_forms(),
            Word::PersonalPronoun(pr) => pr.get_forms(),
        }
    }
}

impl<T> WordForms<T> {
    fn get_word(&self) -> &str {
        &self.general.word
    }

    fn get_translation(&self) -> &str {
        &self.general.translation
    }
}

impl WordForms<NounSpecific> {
    fn get_definite_plural(&self) -> &str {
        &self.specific.definite_plural
    }

    fn get_definite_singular(&self) -> &str {
        &self.specific.definite_singular
    }

    fn get_indefinite_plural(&self) -> &str {
        &self.specific.indefinite_plural
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Definite Singular", self.get_definite_singular()));
        forms.push(("Indefinite Plural", self.get_indefinite_plural()));
        forms.push(("Definite Plural", self.get_definite_plural()));
        forms
    }
}

impl WordForms<AdjectiveSpecific> {
    fn get_neuter(&self) -> &str {
        &self.specific.neuter
    }

    fn get_plural(&self) -> &str {
        &self.specific.plural
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Neuter", self.get_neuter()));
        forms.push(("Plural", self.get_plural()));
        forms
    }
}

impl WordForms<VerbSpecific> {
    fn get_present(&self) -> &str {
        &self.specific.present
    }

    fn get_past(&self) -> &str {
        &self.specific.past
    }

    fn get_perfect(&self) -> &str {
        &self.specific.perfect
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Present", self.get_present()));
        forms.push(("Past", self.get_past()));
        forms.push(("Perfecct", self.get_perfect()));
        forms
    }
}

impl WordForms<PersonalPronounSpecific> {
    fn get_object(&self) -> &str {
        &self.specific.object
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Object", self.get_object()));
        forms
    }
}

impl WordForms<AdverbSpecific> {
    fn get_forms<'a>(&'a self) -> Vec<(&'static str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", &self.general.word[..]));
        forms.push(("Translation", &self.general.translation[..]));
        forms
    }
}
