pub mod question;

pub struct NounSpecific {
    definite: String,
    plural: String,
}

pub struct AdjectiveSpecific {
    neuter: String,
    plural: String,
}

pub struct VerbSpecific {
    present: String,
    past: String,
}

pub struct AdverbSpecific;

pub struct PronounSpecific {
    possessive: String,
}

pub struct WordGeneral {
    word: String,
    translation: String,
}

pub struct WordForms<T> {
    general: WordGeneral,
    specific: T,
}

pub enum Word {
    NOUN(WordForms<NounSpecific>),
    ADJECTIVE(WordForms<AdjectiveSpecific>),
    VERB(WordForms<VerbSpecific>),
    ADVERB(WordForms<AdverbSpecific>),
    PRONOUN(WordForms<PronounSpecific>),
}

impl Word {
    pub fn new_noun(word: String, translation: String, definite: String, plural: String) -> Word {
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

    pub fn new_adjective(
        word: String,
        translation: String,
        neuter: String,
        plural: String,
    ) -> Word {
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

    pub fn new_verb(word: String, translation: String, present: String, past: String) -> Word {
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

    pub fn new_adverb(word: String, translation: String) -> Word {
        Word::ADVERB(WordForms {
            general: WordGeneral {
                word: word,
                translation: translation,
            },
            specific: AdverbSpecific,
        })
    }

    pub fn new_pronoun(word: String, translation: String, possessive: String) -> Word {
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
    pub fn get_word(&self) -> &str {
        &self.general.word
    }

    pub fn get_translation(&self) -> &str {
        &self.general.translation
    }
}

impl WordForms<NounSpecific> {
    pub fn get_plural(&self) -> &str {
        &self.specific.plural
    }

    pub fn get_definite(&self) -> &str {
        &self.specific.definite
    }
}

impl WordForms<AdjectiveSpecific> {
    pub fn get_neuter(&self) -> &str {
        &self.specific.neuter
    }

    pub fn get_plural(&self) -> &str {
        &self.specific.plural
    }
}

impl WordForms<VerbSpecific> {
    pub fn get_present(&self) -> &str {
        &self.specific.present
    }

    pub fn get_past(&self) -> &str {
        &self.specific.past
    }
}

impl WordForms<PronounSpecific> {
    pub fn get_possessive(&self) -> &str {
        &self.specific.possessive
    }
}
