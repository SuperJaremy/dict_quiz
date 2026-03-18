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
//! - Personal pronouns;
//! - Numerals.
//!
//! All other word classes should be assigned to present ones in
//! a case by case manner.

pub mod question;
pub mod word_csv;

pub trait Forms {
    fn get_word(&self) -> &str;

    fn get_translation(&self) -> &str;

    fn get_meaning(&self) -> &str;

    fn get_example(&self) -> &str;

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Meaning", self.get_meaning()),
            ("Example", self.get_example()),
        ]
    }
}

/// All noun word forms
#[derive(Debug, PartialEq)]
pub struct Noun {
    word: String,
    translation: String,
    meaning: String,
    example: String,
    definite_singular: String,
    indefinite_plural: String,
    definite_plural: String,
}

/// All adjective word forms
#[derive(PartialEq, Debug)]
pub struct Adjective {
    word: String,
    translation: String,
    meaning: String,
    example: String,
    neuter: String,
    plural: String,
}

/// All verb word forms
#[derive(PartialEq, Debug)]
pub struct Verb {
    word: String,
    translation: String,
    meaning: String,
    example: String,
    present: String,
    past: String,
    perfect: String,
}

/// All adverb word forms
#[derive(PartialEq, Debug)]
pub struct Adverb {
    word: String,
    translation: String,
    meaning: String,
    example: String,
}

/// All personal pronoun word forms
#[derive(PartialEq, Debug)]
pub struct PersonalPronoun {
    word: String,
    translation: String,
    meaning: String,
    example: String,
    object: String,
}

pub struct Numeral {
    word: String,
    transaltion: String,
    meaning: String,
    example: String,
    ordinal: String,
}

impl Noun {
    fn new(
        word: String,
        translation: String,
        meaning: String,
        example: String,
        definite_singular: String,
        indefinite_plural: String,
        definite_plural: String,
    ) -> Self {
        Noun {
            word,
            translation,
            meaning,
            example,
            definite_singular,
            indefinite_plural,
            definite_plural,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }

    fn get_forms<'a>(&'a self) -> Vec<(&'a str, &'a str)> {
        let mut forms = Vec::new();
        forms.push(("Word", self.get_word()));
        forms.push(("Translation", self.get_translation()));
        forms.push(("Meaning", self.get_meaning()));
        forms.push(("Example", self.get_example()));
        forms.push(("Definite Singular", self.get_definite_singular()));
        forms.push(("Indefinite Plural", self.get_indefinite_plural()));
        forms.push(("Definite Plural", self.get_definite_plural()));
        forms
    }
}

impl Adjective {
    fn new(
        word: String,
        translation: String,
        meaning: String,
        example: String,
        neuter: String,
        plural: String,
    ) -> Adjective {
        Adjective {
            word,
            translation,
            meaning,
            example,
            neuter,
            plural,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Meaning", self.get_meaning()),
            ("Example", self.get_example()),
            ("Neuter", self.get_neuter()),
            ("Plural", self.get_plural()),
        ]
    }
}

impl Verb {
    pub fn new(
        word: String,
        translation: String,
        meaning: String,
        example: String,
        present: String,
        past: String,
        perfect: String,
    ) -> Verb {
        Verb {
            word,
            translation,
            meaning,
            example,
            present,
            past,
            perfect,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Meaning", self.get_meaning()),
            ("Example", self.get_example()),
            ("Present", self.get_present()),
            ("Past", self.get_past()),
            ("Perfect", self.get_perfect()),
        ]
    }
}

impl Adverb {
    fn new(word: String, translation: String, meaning: String, example: String) -> Adverb {
        Adverb {
            word,
            translation,
            meaning,
            example,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }
}

impl PersonalPronoun {
    fn new(
        word: String,
        translation: String,
        meaning: String,
        example: String,
        object: String,
    ) -> PersonalPronoun {
        PersonalPronoun {
            word,
            translation,
            meaning,
            example,
            object,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Meaning", self.get_meaning()),
            ("Example", self.get_example()),
            ("Object", self.get_object()),
        ]
    }
}

impl Numeral {
    fn new(
        word: String,
        transaltion: String,
        meaning: String,
        example: String,
        ordinal: String,
    ) -> Numeral {
        Numeral {
            word,
            transaltion,
            meaning,
            example,
            ordinal,
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

    fn get_meaning(&self) -> &str {
        &self.meaning
    }

    fn get_example(&self) -> &str {
        &self.example
    }

    fn get_forms(&self) -> Vec<(&str, &str)> {
        vec![
            ("Word", self.get_word()),
            ("Translation", self.get_translation()),
            ("Meaning", self.get_meaning()),
            ("Example", self.get_example()),
            ("Ordinal", self.get_ordinal()),
        ]
    }
}
