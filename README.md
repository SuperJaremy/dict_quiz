# Swedish Dictionary Quiz

Small program to make quizes out of Swedish dictionaries written in Rust.

## Quiz
Throughout the quiz you'll be asked to answer a number of questions. In each
question you'll be given a word form which you'll have to transform to another form,
be it to translate the word, to provide a singualr or plural form, etc. The choice
of the question, as well as the order of words, is randomized to be more
engaging.

![Quiz process](https://drive.google.com/file/d/1i4_imoUVvWf-WX_gOiNAs_8jxuDKSOkP/view?usp=sharing)

## Dictionary
The dictionary is a csv file where the words, which you want to appear in 
your quiz, as well as all their forms are listed (see `res/dictionary.csv`).
Each entry must have one of the supported classes.

Unfortunately, as of now it's impossible to add context to the words,
so adding homographs to your dictionary is discouraged.

### Word Classes
For now the supported word classes are:
- Nouns;
- Adjectives;
- Verbs;
- Adverbs;
- Personal pronouns.

This classifiaction is inacuarate as grammatical classification of words is based on their
meaning and role in a sentnce rather than on the number of forms, which makes
it difficult for some words to be processed programmatically. For that reason,
rather than use the actual classification, in the dictionary you should base
the word class on the word's forms. For example, the word "vad" can be classified
as adverb, while the word "min" - as adjective.

## Closing words
I started this endeavor after noticing that I struggle learning Swedish words.
I thought that if I would make the process of learning words interactive and also
if I'd be able to choose the words I want to learn, then I'd finally
start to grasp the intricasies of Swedish grammar and I'd do it much quicker.
It's also my first ever project written in Rust, because I'd always wanted to
learn this programming language. So, for me this is an opportunity to kill two birds,
or languages, with one stone.