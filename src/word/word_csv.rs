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
    possesive: Option<String>,
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
    let Some(possessive) = word_csv.possesive else {
        return None;
    };

    Some(Word::new_pronoun(
        word_csv.word,
        word_csv.translation,
        possessive,
    ))
}
