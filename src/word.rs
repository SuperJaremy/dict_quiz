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

    pub fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        match self {
            Word::NOUN(noun) => noun.get_forms(),
            Word::ADJECTIVE(adj) => adj.get_forms(),
            Word::VERB(verb) => verb.get_forms(),
            Word::ADVERB(adv) => adv.get_forms(),
            Word::PRONOUN(pr) => pr.get_forms(),
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
    fn get_plural(&self) -> &str {
        &self.specific.plural
    }

    fn get_definite(&self) -> &str {
        &self.specific.definite
    }

    fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        let mut forms = Vec::new();
        forms.push(&self.general.word[..]);
        forms.push(&self.general.translation[..]);
        forms.push(&self.specific.definite[..]);
        forms.push(&self.specific.plural[..]);
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

    fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        let mut forms = Vec::new();
        forms.push(&self.general.word[..]);
        forms.push(&self.general.translation[..]);
        forms.push(&self.specific.neuter[..]);
        forms.push(&self.specific.plural[..]);
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

    fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        let mut forms = Vec::new();
        forms.push(&self.general.word[..]);
        forms.push(&self.general.translation[..]);
        forms.push(&self.specific.past[..]);
        forms.push(&&self.specific.present[..]);
        forms
    }
}

impl WordForms<PronounSpecific> {
    fn get_possessive(&self) -> &str {
        &self.specific.possessive
    }

    fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        let mut forms = Vec::new();
        forms.push(&self.general.word[..]);
        forms.push(&self.general.translation[..]);
        forms.push(&self.specific.possessive[..]);
        forms
    }
}

impl WordForms<AdverbSpecific> {
    fn get_forms<'a>(&'a self) -> Vec<&'a str> {
        let mut forms = Vec::new();
        forms.push(&self.general.word[..]);
        forms.push(&self.general.translation[..]);
        forms
    }
}
