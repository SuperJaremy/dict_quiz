use serde::Deserialize;
use std::{env, error::Error, ffi::OsString, io, process};

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

struct NounSpecific {
    definite: String,
    plural: String,
}

struct AdjectiveSpecific {
    neuter: String,
    plural: String,
}

struct VerbSpecific {
    present: String,
    past: String,
}

struct AdverbSpecific;

struct PronounSpecific {
    possesive: String,
}

struct WordGeneral {
    word: String,
    translation: String,
}

enum Word {
    NOUN(WordGeneral, NounSpecific),
    ADJECTIVE(WordGeneral, AdjectiveSpecific),
    VERB(WordGeneral, VerbSpecific),
    ADVERB(WordGeneral, AdverbSpecific),
    PRONOUN(WordGeneral, PronounSpecific),
}

impl Word {
    fn general(&self) -> &WordGeneral {
        match self {
            Self::NOUN(wg, _) => wg,
            Self::ADJECTIVE(wg, _) => wg,
            Self::VERB(wg, _) => wg,
            Self::ADVERB(wg, _) => wg,
            Self::PRONOUN(wg, _) => wg,
        }
    }
}

fn word_csv_to_noun(word_csv: WordCSV) -> Option<Word> {
    if let Some(definite) = word_csv.definite
        && let Some(plural) = word_csv.plural
    {
        Some(Word::NOUN(
            WordGeneral {
                word: word_csv.word,
                translation: word_csv.translation,
            },
            NounSpecific {
                definite: definite,
                plural: plural,
            },
        ))
    } else {
        None
    }
}

fn word_csv_to_adjective(word_csv: WordCSV) -> Option<Word> {
    if let Some(neuter) = word_csv.neuter
        && let Some(plural) = word_csv.plural
    {
        Some(Word::ADJECTIVE(
            WordGeneral {
                word: word_csv.word,
                translation: word_csv.translation,
            },
            AdjectiveSpecific {
                neuter: neuter,
                plural: plural,
            },
        ))
    } else {
        None
    }
}

fn word_csv_to_verb(word_csv: WordCSV) -> Option<Word> {
    if let Some(present) = word_csv.present
        && let Some(past) = word_csv.past
    {
        Some(Word::VERB(
            WordGeneral {
                word: word_csv.word,
                translation: word_csv.translation,
            },
            VerbSpecific {
                present: present,
                past: past,
            },
        ))
    } else {
        None
    }
}

fn word_csv_to_adverb(word_csv: WordCSV) -> Option<Word> {
    Some(Word::ADVERB(
        WordGeneral {
            word: word_csv.word,
            translation: word_csv.translation,
        },
        AdverbSpecific,
    ))
}

fn word_csv_to_pronoun(word_csv: WordCSV) -> Option<Word> {
    let Some(possesive) = word_csv.possesive else {
        return None;
    };

    Some(Word::PRONOUN(
        WordGeneral {
            word: word_csv.word,
            translation: word_csv.translation,
        },
        PronounSpecific {
            possesive: possesive,
        },
    ))
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

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_frist_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: WordCSV = result?;
        let word = word_csv_to_word(record)?;
        if ask_translation(&word) {
            println!("✅Correct!");
        } else {
            println!("❌Incorrect");
        }
    }

    Ok(())
}

fn get_frist_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 arguemnt, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn ask_translation(word: &Word) -> bool {
    let general = word.general();
    println!("{}", general.word);
    println!("Translate this word in English");

    let mut answer = String::new();

    loop {
        match io::stdin().read_line(&mut answer) {
            Ok(_) => break,
            Err(_) => {
                println!("Try again");
                answer.clear();
            }
        }
    }

    general.translation == answer.trim().to_lowercase()
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
