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
    possessive: String,
}

struct WordGeneral {
    word: String,
    translation: String,
}

pub enum Word {
    NOUN(WordGeneral, NounSpecific),
    ADJECTIVE(WordGeneral, AdjectiveSpecific),
    VERB(WordGeneral, VerbSpecific),
    ADVERB(WordGeneral, AdverbSpecific),
    PRONOUN(WordGeneral, PronounSpecific),
}

impl Word {
    pub fn new_noun(word: String, translation: String, definite: String, plural: String) -> Word {
        Word::NOUN(
            WordGeneral {
                word: word,
                translation: translation,
            },
            NounSpecific {
                definite: definite,
                plural: plural,
            },
        )
    }

    pub fn new_adjective(
        word: String,
        translation: String,
        neuter: String,
        plural: String,
    ) -> Word {
        Word::ADJECTIVE(
            WordGeneral {
                word: word,
                translation: translation,
            },
            AdjectiveSpecific {
                neuter: neuter,
                plural: plural,
            },
        )
    }

    pub fn new_verb(word: String, translation: String, present: String, past: String) -> Word {
        Word::VERB(
            WordGeneral {
                word: word,
                translation: translation,
            },
            VerbSpecific {
                present: present,
                past: past,
            },
        )
    }

    pub fn new_adverb(word: String, translation: String) -> Word {
        Word::ADVERB(
            WordGeneral {
                word: word,
                translation: translation,
            },
            AdverbSpecific,
        )
    }

    pub fn new_pronoun(word: String, translation: String, possessive: String) -> Word {
        Word::PRONOUN(
            WordGeneral {
                word: word,
                translation: translation,
            },
            PronounSpecific {
                possessive: possessive,
            },
        )
    }

    pub fn get_word(&self) -> &str {
        &self.general().word
    }

    pub fn get_translation(&self) -> &str {
        &self.general().translation
    }

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
