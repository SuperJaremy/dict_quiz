pub mod question;
pub mod word_csv;

#[derive(PartialEq, Debug)]
pub struct NounSpecific {
    definite: String,
    plural: String,
}

#[derive(PartialEq, Debug)]
pub struct AdjectiveSpecific {
    neuter: String,
    plural: String,
}

#[derive(PartialEq, Debug)]
pub struct VerbSpecific {
    present: String,
    past: String,
}

#[derive(PartialEq, Debug)]
pub struct AdverbSpecific;

#[derive(PartialEq, Debug)]
pub struct PronounSpecific {
    possessive: String,
}

#[derive(PartialEq, Debug)]
pub struct WordGeneral {
    word: String,
    translation: String,
}

#[derive(PartialEq, Debug)]
pub struct WordForms<T> {
    general: WordGeneral,
    specific: T,
}

#[derive(PartialEq, Debug)]
pub enum Word {
    NOUN(WordForms<NounSpecific>),
    ADJECTIVE(WordForms<AdjectiveSpecific>),
    VERB(WordForms<VerbSpecific>),
    ADVERB(WordForms<AdverbSpecific>),
    PRONOUN(WordForms<PronounSpecific>),
}

impl Word {
    fn new_noun(word: String, translation: String, definite: String, plural: String) -> Word {
        Word::NOUN(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: NounSpecific {
                definite: definite,
                plural: plural,
            },
        })
    }

    fn new_adjective(word: String, translation: String, neuter: String, plural: String) -> Word {
        Word::ADJECTIVE(WordForms {
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

    fn new_verb(word: String, translation: String, present: String, past: String) -> Word {
        Word::VERB(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: VerbSpecific {
                present: present,
                past: past,
            },
        })
    }

    fn new_adverb(word: String, translation: String) -> Word {
        Word::ADVERB(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: AdverbSpecific,
        })
    }

    fn new_pronoun(word: String, translation: String, possessive: String) -> Word {
        Word::PRONOUN(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: PronounSpecific {
                possessive: possessive,
            },
        })
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
    fn get_plural(&self) -> &str {
        &self.specific.plural
    }

    fn get_definite(&self) -> &str {
        &self.specific.definite
    }
}

impl WordForms<AdjectiveSpecific> {
    fn get_neuter(&self) -> &str {
        &self.specific.neuter
    }

    fn get_plural(&self) -> &str {
        &self.specific.plural
    }
}

impl WordForms<VerbSpecific> {
    fn get_present(&self) -> &str {
        &self.specific.present
    }

    fn get_past(&self) -> &str {
        &self.specific.past
    }
}

impl WordForms<PronounSpecific> {
    fn get_possessive(&self) -> &str {
        &self.specific.possessive
    }
}
