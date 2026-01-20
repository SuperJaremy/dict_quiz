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

pub trait Forms {
    fn get_word(&self) -> &str;

    fn get_translation(&self) -> &str;

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
        ]
    }
}

/// All noun word forms
#[derive(Debug, PartialEq)]
pub struct Noun {
    word: String,
    translation: String,
    definite_singular: String,
    indefinite_plural: String,
    definite_plural: String,
}

/// All adjective word forms
#[derive(PartialEq, Debug)]
pub struct Adjective {
    word: String,
    translation: String,
    neuter: String,
    plural: String,
}

/// All verb word forms
#[derive(PartialEq, Debug)]
pub struct Verb {
    word: String,
    translation: String,
    present: String,
    past: String,
    perfect: String,
}

/// All adverb word forms
#[derive(PartialEq, Debug)]
pub struct Adverb {
    word: String,
    translation: String,
}

/// All personal pronoun word forms
#[derive(PartialEq, Debug)]
pub struct PersonalPronoun {
    word: String,
    translation: String,
    object: String,
}

pub struct Numeral {
    word: String,
    transaltion: String,
    ordinal: String,
}

impl Noun {
    fn new(
        word: String,
        translation: String,
        definite_singular: String,
        indefinite_plural: String,
        definite_plural: String,
    ) -> Self {
        Noun {
            word: word,
            translation: translation,
            definite_singular: definite_singular,
            indefinite_plural: indefinite_plural,
            definite_plural: definite_plural,
        }
    }

    fn get_definite_plural(&self) -> &str {
        &self.definite_plural
    }

    fn get_definite_singular(&self) -> &str {
        &self.definite_singular
    }

    fn get_indefinite_plural(&self) -> &str {
        &self.indefinite_plural
    }
}

impl Forms for Noun {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.translation
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'a str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Definite Singular", self.get_definite_singular()));
        forms.push(("Indefinite Plural", self.get_indefinite_plural()));
        forms.push(("Definite Plural", self.get_definite_plural()));
        forms
    }
}

impl Adjective {
    fn new(word: String, translation: String, neuter: String, plural: String) -> Adjective {
        Adjective {
            word: word,
            translation: translation,
            neuter: neuter,
            plural: plural,
        }
    }

    fn get_neuter(&self) -> &str {
        &self.neuter
    }

    fn get_plural(&self) -> &str {
        &self.plural
    }
}

impl Forms for Adjective {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.translation
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Neuter", self.get_neuter()),
            ("Plural", self.get_plural()),
        ]
    }
}

impl Verb {
    pub fn new(
        word: String,
        translation: String,
        present: String,
        past: String,
        perfect: String,
    ) -> Verb {
        Verb {
            word: word,
            translation: translation,
            present: present,
            past: past,
            perfect: perfect,
        }
    }

    pub fn get_present(&self) -> &str {
        &self.present
    }

    pub fn get_past(&self) -> &str {
        &self.past
    }

    pub fn get_perfect(&self) -> &str {
        &self.perfect
    }
}

impl Forms for Verb {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.translation
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Present", self.get_present()),
            ("Past", self.get_past()),
            ("Perfect", self.get_perfect()),
        ]
    }
}

impl Adverb {
    fn new(word: String, translation: String) -> Adverb {
        Adverb {
            word: word,
            translation: translation,
        }
    }
}

impl Forms for Adverb {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.translation
    }
}

impl PersonalPronoun {
    fn new(word: String, translation: String, object: String) -> PersonalPronoun {
        PersonalPronoun {
            word: word,
            translation: translation,
            object: object,
        }
    }

    fn get_object(&self) -> &str {
        &self.object
    }
}

impl Forms for PersonalPronoun {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.translation
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Object", self.get_object()),
        ]
    }
}

impl Numeral {
    fn new(word: String, transaltion: String, ordinal: String) -> Numeral {
        Numeral {
            word: word,
            transaltion: transaltion,
            ordinal: ordinal,
        }
    }

    fn get_ordinal(&self) -> &str {
        &self.ordinal
    }
}

impl Forms for Numeral {
    fn get_word(&self) -> &str {
        &self.word
    }

    fn get_translation(&self) -> &str {
        &self.transaltion
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Ordinal", self.get_ordinal()),
        ]
    }
}
