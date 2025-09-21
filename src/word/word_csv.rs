//! Parse csv entry into `dict_quiz::word::Word`.
//!
//! # Format
//! The csv file should comply to the following format:
//!
//! `word,translation,class,definite_s(noun),plural(noun;adj),definite_p(noun),neuter(adj),present(verb),past(verb),perfect(verb),object(per_pronoun)`.
//!
//! Unrelated fields must be left blank.
//!
//! # Supported Word Classes
//! The choice of class should be based on the word forms of a particular word,
//! rather than actual classification. Currently supported classes are:
//! - `NOUN`: word, translation, class, definite_s, plural, definite_p;
//! - `ADJECTIVE`: word, translation, class, plural, neuter;
//! - `VERB`: word, translation, class, present, past, perfect;
//! - `ADVERB`: word, translation, class;
//! - `PER_PRONOUN`: word, translation, class, object.
//!
//! # Examples
//! - Noun: `ord,word,NOUN,ordet,ord,orden,,,,,`;
//! - Adjective: `grön,green,ADJECTIVE,,gröna,,grönt,,,,`;
//! - Verb: `studera,study,VERB,,,,,studerar,studerade,studerat,`;
//! - Adverb: `inte,not,ADVERB,,,,,,,,`;
//! - Personal pronoun: `hon,she,PER_PRONOUN,,,,,,,,henne`.

use super::question::Pick;
use crate::word::Adjective;
use crate::word::Adverb;
use crate::word::Noun;
use crate::word::PersonalPronoun;
use crate::word::Verb;
use serde::Deserialize;

/// csv representation of `dict_quiz::word::Word`.
#[derive(Debug, Deserialize)]
pub struct WordCSV {
    #[serde(rename = "word")]
    word: String,
    #[serde(rename = "translation")]
    translation: String,
    #[serde(rename = "class")]
    class: String,
    #[serde(rename = "definite_s(noun)")]
    definite_singular: Option<String>,
    #[serde(rename = "plural(noun;adj)")]
    plural: Option<String>,
    #[serde(rename = "definite_p(noun)")]
    definite_plural: Option<String>,
    #[serde(rename = "neuter(adj)")]
    neuter: Option<String>,
    #[serde(rename = "present(verb)")]
    present: Option<String>,
    #[serde(rename = "past(verb)")]
    past: Option<String>,
    #[serde(rename = "perfect(verb)")]
    perfect: Option<String>,
    #[serde(rename = "object(per_pronoun)")]
    object: Option<String>,
}
/// Transforms WordCSV to Word
pub fn word_csv_to_word(word_csv: WordCSV) -> Result<Box<dyn Pick>, &'static str> {
    let class = &word_csv.class;

    match class.as_str() {
        "NOUN" => {
            word_csv_to_noun(word_csv).map_or(Err("Error parsing noun"), |word| Ok(Box::new(word)))
        }
        "ADJECTIVE" => word_csv_to_adjective(word_csv)
            .map_or(Err("Error parsing adjective"), |word| Ok(Box::new(word))),
        "VERB" => {
            word_csv_to_verb(word_csv).map_or(Err("Error parsing verb"), |word| Ok(Box::new(word)))
        }
        "ADVERB" => word_csv_to_adverb(word_csv)
            .map_or(Err("Error parsing adverb"), |word| Ok(Box::new(word))),
        "PER_PRONOUN" => word_csv_to_personal_pronoun(word_csv)
            .map_or(Err("Error parsing personal pronoun"), |word| {
                Ok(Box::new(word))
            }),
        &_ => Err("Wrong class"),
    }
}

fn word_csv_to_noun(word_csv: WordCSV) -> Option<Noun> {
    if let Some(definite_s) = word_csv.definite_singular
        && let Some(plural_i) = word_csv.plural
        && let Some(definite_p) = word_csv.definite_plural
    {
        Some(Noun::new(
            word_csv.word,
            word_csv.translation,
            definite_s,
            plural_i,
            definite_p,
        ))
    } else {
        None
    }
}

fn word_csv_to_adjective(word_csv: WordCSV) -> Option<Adjective> {
    if let Some(neuter) = word_csv.neuter
        && let Some(plural) = word_csv.plural
    {
        Some(Adjective::new(
            word_csv.word,
            word_csv.translation,
            neuter,
            plural,
        ))
    } else {
        None
    }
}

fn word_csv_to_verb(word_csv: WordCSV) -> Option<Verb> {
    if let Some(present) = word_csv.present
        && let Some(past) = word_csv.past
        && let Some(perfect) = word_csv.perfect
    {
        Some(Verb::new(
            word_csv.word,
            word_csv.translation,
            present,
            past,
            perfect,
        ))
    } else {
        None
    }
}

fn word_csv_to_adverb(word_csv: WordCSV) -> Option<Adverb> {
    Some(Adverb::new(word_csv.word, word_csv.translation))
}

fn word_csv_to_personal_pronoun(word_csv: WordCSV) -> Option<PersonalPronoun> {
    if let Some(object) = word_csv.object {
        Some(PersonalPronoun::new(
            word_csv.word,
            word_csv.translation,
            object,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_word_csv() -> WordCSV {
        WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("c"),
            definite_singular: Some(String::from("d")),
            plural: Some(String::from("e")),
            definite_plural: Some(String::from("f")),
            neuter: Some(String::from("g")),
            present: Some(String::from("h")),
            past: Some(String::from("i")),
            perfect: Some(String::from("j")),
            object: Some(String::from("k")),
        }
    }

    #[test]
    fn cannot_create_word_with_incorrect_class() {
        let mut empty = setup_word_csv();
        empty.class = String::from("");

        let act = word_csv_to_word(empty);
        assert!(act.is_err(), "Created a word with empty string for class");

        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("jdakljsdkl");

        let act = word_csv_to_word(incorrect);
        assert!(act.is_err(), "Created a word with class jdakljsdkl");
    }

    #[test]
    fn cannot_create_noun_without_definite_singular_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("NOUN");
        incorrect.definite_singular = None;

        let act = word_csv_to_noun(incorrect);
        assert!(
            act.is_none(),
            "Created a noun without definite singular form"
        );
    }

    #[test]
    fn cannot_create_noun_without_indefinite_plural_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("NOUN");
        incorrect.plural = None;

        let act = word_csv_to_noun(incorrect);
        assert!(
            act.is_none(),
            "Created a noun without indefinite plural form"
        );
    }

    #[test]
    fn cannot_create_noun_without_definite_plural_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("NOUN");
        incorrect.definite_plural = None;

        let act = word_csv_to_noun(incorrect);
        assert!(act.is_none(), "Created a noun without definite plural form");
    }

    #[test]
    fn can_create_noun_whith_definite_singular_and_definite_plural_and_indefinite_plural() {
        let mut correct = setup_word_csv();
        correct.class = String::from("NOUN");
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act1 = word_csv_to_noun(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_noun cannot create a noun with definite and plural forms"
        );

        let mut correct = setup_word_csv();
        correct.class = String::from("NOUN");
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a noun with definite and plural forms"
        );
    }

    #[test]
    fn cannot_create_adjective_without_neuter_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("ADJECTIVE");
        incorrect.neuter = None;

        let act = word_csv_to_adjective(incorrect);
        assert!(act.is_none(), "Created an adjective without neuter form");
    }

    #[test]
    fn cannot_create_adjective_without_plural_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("ADJECTIVE");
        incorrect.plural = None;

        let act = word_csv_to_adjective(incorrect);
        assert!(act.is_none(), "Created an adjective without plural form");
    }

    #[test]
    fn can_create_adjective_whith_neuter_and_plural() {
        let mut correct = setup_word_csv();
        correct.class = String::from("ADJECTIVE");
        correct.definite_singular = None;
        correct.definite_plural = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act1 = word_csv_to_adjective(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_adjective cannot create an adjective with neuter and plural forms"
        );

        let mut correct = setup_word_csv();
        correct.class = String::from("ADJECTIVE");
        correct.definite_singular = None;
        correct.definite_plural = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create an adjective with neuter and plural forms"
        );
    }

    #[test]
    fn cannot_create_verb_without_present_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("VERB");
        incorrect.present = None;

        let act = word_csv_to_verb(incorrect);
        assert!(act.is_none(), "Created a verb without present form");
    }

    #[test]
    fn cannot_create_verb_without_past_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("VERB");
        incorrect.past = None;

        let act = word_csv_to_verb(incorrect);
        assert!(act.is_none(), "Created a verb without past form");
    }

    #[test]
    fn cannot_create_verb_without_perfect_form() {
        let mut incorrect = setup_word_csv();
        incorrect.class = String::from("VERB");
        incorrect.perfect = None;

        let act = word_csv_to_verb(incorrect);
        assert!(act.is_none(), "Created a verb without past form");
    }

    #[test]
    fn can_create_verb_whith_present_and_past_and_perfect() {
        let mut correct = setup_word_csv();
        correct.class = String::from("VERB");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.object = None;

        let act1 = word_csv_to_verb(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_verb cannot create a verb with present and past forms"
        );

        let mut correct = setup_word_csv();
        correct.class = String::from("VERB");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.object = None;

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a verb with present and past forms"
        );
    }

    #[test]
    fn can_create_adverb() {
        let mut correct = setup_word_csv();
        correct.class = String::from("ADVERB");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act1 = word_csv_to_adverb(correct);
        assert!(act1.is_some(), "word_csv_to_adverb cannot create an adverb");

        let mut correct = setup_word_csv();
        correct.class = String::from("ADVERB");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;
        correct.object = None;

        let act2 = word_csv_to_word(correct);
        assert!(act2.is_ok(), "word_csv_to_word cannot create an adverb");
    }

    #[test]
    fn cannot_create_personal_pronoun_without_objective_form() {
        let mut incorrect = setup_word_csv();
        incorrect.object = None;

        let act = word_csv_to_personal_pronoun(incorrect);
        assert!(
            act.is_none(),
            "Created a personal pronoun without objective form"
        );
    }

    #[test]
    fn can_create_pronoun_with_objective() {
        let mut correct = setup_word_csv();
        correct.class = String::from("PER_PRONOUN");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;

        let act1 = word_csv_to_personal_pronoun(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_pronoun cannot create a pronoun with possessive form"
        );

        let mut correct = setup_word_csv();
        correct.class = String::from("PER_PRONOUN");
        correct.definite_singular = None;
        correct.plural = None;
        correct.definite_plural = None;
        correct.neuter = None;
        correct.present = None;
        correct.past = None;
        correct.perfect = None;

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a pronoun with possessive form"
        );
    }
}
