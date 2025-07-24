use std::{error::Error, ffi::OsString};

use super::Word;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WordCSV {
    #[serde(rename = "word")]
    word: String,
    #[serde(rename = "translation")]
    translation: String,
    #[serde(rename = "class")]
    class: String,
    #[serde(rename = "definite(noun)")]
    definite: Option<String>,
    #[serde(rename = "plural(noun;adj)")]
    plural: Option<String>,
    #[serde(rename = "neuter(adj)")]
    neuter: Option<String>,
    #[serde(rename = "present(verb)")]
    present: Option<String>,
    #[serde(rename = "past(verb)")]
    past: Option<String>,
    #[serde(rename = "possessive(pronoun)")]
    possessive: Option<String>,
}

pub fn read_csv(dict_path: OsString) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(dict_path)?;
    let mut res = Vec::new();

    for result in rdr.deserialize() {
        let record: WordCSV = result?;
        let word = word_csv_to_word(record)?;
        res.push(word);
    }

    Ok(res)
}

fn word_csv_to_word(word_csv: WordCSV) -> Result<Word, String> {
    let class = &word_csv.class;

    let word = if class == "NOUN" {
        word_csv_to_noun(word_csv)
    } else if class == "ADJECTIVE" {
        word_csv_to_adjective(word_csv)
    } else if class == "VERB" {
        word_csv_to_verb(word_csv)
    } else if class == "ADVERB" {
        word_csv_to_adverb(word_csv)
    } else if class == "PRONOUN" {
        word_csv_to_pronoun(word_csv)
    } else {
        None
    };

    match word {
        Some(ret) => Ok(ret),
        None => Err(String::from("Wrong class")),
    }
}

fn word_csv_to_noun(word_csv: WordCSV) -> Option<Word> {
    if let Some(definite) = word_csv.definite
        && let Some(plural) = word_csv.plural
    {
        Some(Word::new_noun(
            word_csv.word,
            word_csv.translation,
            definite,
            plural,
        ))
    } else {
        None
    }
}

fn word_csv_to_adjective(word_csv: WordCSV) -> Option<Word> {
    if let Some(neuter) = word_csv.neuter
        && let Some(plural) = word_csv.plural
    {
        Some(Word::new_adjective(
            word_csv.word,
            word_csv.translation,
            neuter,
            plural,
        ))
    } else {
        None
    }
}

fn word_csv_to_verb(word_csv: WordCSV) -> Option<Word> {
    if let Some(present) = word_csv.present
        && let Some(past) = word_csv.past
    {
        Some(Word::new_verb(
            word_csv.word,
            word_csv.translation,
            present,
            past,
        ))
    } else {
        None
    }
}

fn word_csv_to_adverb(word_csv: WordCSV) -> Option<Word> {
    Some(Word::new_adverb(word_csv.word, word_csv.translation))
}

fn word_csv_to_pronoun(word_csv: WordCSV) -> Option<Word> {
    let Some(possessive) = word_csv.possessive else {
        return None;
    };

    Some(Word::new_pronoun(
        word_csv.word,
        word_csv.translation,
        possessive,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_create_word_with_incorrect_class() {
        let empty = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from(""),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_word(empty);
        assert!(act.is_err(), "Created a word with empty string for class");

        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("jdakljsdkl"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_word(incorrect);
        assert!(act.is_err(), "Created a word with class jdakljsdkl");
    }

    #[test]
    fn cannot_create_noun_without_definite_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("NOUN"),
            definite: None,
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_noun(incorrect);
        assert!(act.is_none(), "Created a noun without definite form");
    }

    #[test]
    fn cannot_create_noun_without_plural_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("NOUN"),
            definite: Some(String::from("c")),
            plural: None,
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_noun(incorrect);
        assert!(act.is_none(), "Created a noun without plural form");
    }

    #[test]
    fn can_create_noun_whith_definite_and_plural() {
        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("NOUN"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: None,
            present: None,
            past: None,
            possessive: None,
        };

        let act1 = word_csv_to_noun(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_noun cannot create a noun with definite and plural forms"
        );

        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("NOUN"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: None,
            present: None,
            past: None,
            possessive: None,
        };

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a noun with definite and plural forms"
        );

        if let Some(act1) = act1
            && let Ok(act2) = act2
        {
            assert_eq!(
                act1, act2,
                "word_csv_to_word and word_csv_to_noun produce different results"
            );
        }
    }

    #[test]
    fn cannot_create_adjective_without_neuter_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADJECTIVE"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: None,
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_adjective(incorrect);
        assert!(act.is_none(), "Created an adjective without neuter form");
    }

    #[test]
    fn cannot_create_adjective_without_plural_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADJECTIVE"),
            definite: Some(String::from("c")),
            plural: None,
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_adjective(incorrect);
        assert!(act.is_none(), "Created an adjective without plural form");
    }

    #[test]
    fn can_create_adjective_whith_neuter_and_plural() {
        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADJECTIVE"),
            definite: None,
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: None,
            past: None,
            possessive: None,
        };

        let act1 = word_csv_to_adjective(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_adjective cannot create an adjective with neuter and plural forms"
        );

        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADJECTIVE"),
            definite: None,
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: None,
            past: None,
            possessive: None,
        };

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create an adjective with neuter and plural forms"
        );

        if let Some(act1) = act1
            && let Ok(act2) = act2
        {
            assert_eq!(
                act1, act2,
                "word_csv_to_word and word_csv_to_adjective produce different results"
            );
        }
    }

    #[test]
    fn cannot_create_verb_without_present_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("VERB"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: None,
            past: Some(String::from("g")),
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_verb(incorrect);
        assert!(act.is_none(), "Created a verb without present form");
    }

    #[test]
    fn cannot_create_verb_without_past_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("VERB"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: None,
            possessive: Some(String::from("h")),
        };

        let act = word_csv_to_verb(incorrect);
        assert!(act.is_none(), "Created a verb without past form");
    }

    #[test]
    fn can_create_verb_whith_present_and_past() {
        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("VERB"),
            definite: None,
            plural: None,
            neuter: None,
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: None,
        };

        let act1 = word_csv_to_verb(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_verb cannot create a verb with present and past forms"
        );

        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("VERB"),
            definite: None,
            plural: None,
            neuter: None,
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: None,
        };

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a verb with present and past forms"
        );

        if let Some(act1) = act1
            && let Ok(act2) = act2
        {
            assert_eq!(
                act1, act2,
                "word_csv_to_word and word_csv_to_verb produce different results"
            );
        }
    }

    #[test]
    fn can_create_adverb() {
        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADVERB"),
            definite: None,
            plural: None,
            neuter: None,
            present: None,
            past: None,
            possessive: None,
        };

        let act1 = word_csv_to_adverb(correct);
        assert!(act1.is_some(), "word_csv_to_adverb cannot create an adverb");

        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("ADVERB"),
            definite: None,
            plural: None,
            neuter: None,
            present: None,
            past: None,
            possessive: None,
        };

        let act2 = word_csv_to_word(correct);
        assert!(act2.is_ok(), "word_csv_to_word cannot create an adverb");

        if let Some(act1) = act1
            && let Ok(act2) = act2
        {
            assert_eq!(
                act1, act2,
                "word_csv_to_word and word_csv_to_adverb produce different results"
            );
        }
    }

    #[test]
    fn cannot_create_pronoun_without_possessive_form() {
        let incorrect = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("PRONOUN"),
            definite: Some(String::from("c")),
            plural: Some(String::from("d")),
            neuter: Some(String::from("e")),
            present: Some(String::from("f")),
            past: Some(String::from("g")),
            possessive: None,
        };

        let act = word_csv_to_pronoun(incorrect);
        assert!(act.is_none(), "Created a pronoun without possessive form");
    }

    #[test]
    fn can_create_pronoun_with_possessive() {
        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("PRONOUN"),
            definite: None,
            plural: None,
            neuter: None,
            present: None,
            past: None,
            possessive: Some(String::from("h")),
        };

        let act1 = word_csv_to_pronoun(correct);
        assert!(
            act1.is_some(),
            "word_csv_to_pronoun cannot create a pronoun with possessive form"
        );

        let correct = WordCSV {
            word: String::from("a"),
            translation: String::from("b"),
            class: String::from("PRONOUN"),
            definite: None,
            plural: None,
            neuter: None,
            present: None,
            past: None,
            possessive: Some(String::from("h")),
        };

        let act2 = word_csv_to_word(correct);
        assert!(
            act2.is_ok(),
            "word_csv_to_word cannot create a pronoun with possessive form"
        );

        if let Some(act1) = act1
            && let Ok(act2) = act2
        {
            assert_eq!(
                act1, act2,
                "word_csv_to_word and word_csv_to_pronoun produce different results"
            );
        }
    }
}
