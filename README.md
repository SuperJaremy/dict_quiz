# Swedish Dictionary Quiz
Small program to make quizzes out of Swedish dictionaries written in Rust.

## Quiz
Throughout the quiz, you’ll be asked to answer questions. In each
question you’ll be given a word form which you’ll have to transform to another form,
be it to translate the word, to provide a singular or plural form, etc. The choice
of the question, as well as the order of words, is randomized to be more
engaging.

![quiz presentation](gif/quiz.GIF)

## Quick start
### Prerequisites
- `rustc ~= 1.89.0`
- `cargo ~= 1.89.0`

### Run
`$ cargo run --release /path/to/dicitonary`

## Dictionary
The dictionary is a `.csv` file where the words which you want to appear in
your quiz, as well as all their forms, are listed (see `res/dictionary.csv` as an example).
Each entry must have one of the supported classes.
One can add words’ lexical meanings and examples of usage to the `.csv` file.

### Word Classes
For now, the supported word classes are:
- Nouns;
- Adjectives;
- Verbs;
- Adverbs;
- Personal pronouns;
- Numerals.

This classification is inaccurate, as grammatical classification of words is based on their
meaning and role in a sentence rather than on the number of forms, which makes
it difficult for some words to be processed programmatically. For that reason,
rather than use the actual classification, in the dictionary you should base
the word class on the word’s forms. For example, the word “vad” can be classified
as an adverb, while the word “min” is an adjective.

## Closing words
I started this endeavour after noticing that I struggle to learn Swedish words.
I thought that if I made the process of learning words interactive and also
if I’d be able to choose the words I want to learn, then I’d finally
start to grasp the intricacies of Swedish grammar and I’d do it much quicker.
It’s also my first ever project written in Rust, because I’d always wanted to
learn this programming language. So, for me this is an opportunity to kill two birds,
or languages, with one stone.