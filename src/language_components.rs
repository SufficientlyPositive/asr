// honestly this is probably pretty rubbish, just start from importing mp3s and mp4s ffs.

use std::fmt;

#[derive(Debug)]
pub struct Syllable {
    onset : Onset,
    nucleus : Nucleus,
    coda : Coda,
}

impl Syllable {
    fn getPhonemes() -> String {
        todo!();
    }
}

fn create_syllable(onset : String, nucleus : String, coda : String) -> Syllable {
    todo!();
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // want something in the format "onset-nucleus-coda"
        write!(f, "{}-{}-{}", self.onset, self.nucleus, self.coda)
    }
}

#[derive(Debug)]
enum Onset {
    None,
    C1(Phoneme),
    C2(Phoneme, Phoneme),
    C3(Phoneme, Phoneme, Phoneme)
}

impl fmt::Display for Onset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Onset::None => write!(f, ""),
            Onset::C1(a) => write!(f, "{}", a),
            Onset::C2(a, b) => write!(f, "{}{}", a, b),
            Onset::C3(a, b, c) => write!(f, "{}{}{}", a, b, c)
        }
    }
}

#[derive(Debug)]
enum Nucleus {
    V(Phoneme)
}

impl fmt::Display for Nucleus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {Nucleus::V(a) => a})
    }
}

#[derive(Debug)]
enum Coda {
    None,
    C1(Phoneme),
    C2(Phoneme, Phoneme),
    C3(Phoneme, Phoneme, Phoneme),
    C4(Phoneme, Phoneme, Phoneme, Phoneme),
    C5(Phoneme, Phoneme, Phoneme, Phoneme, Phoneme)
}

impl fmt::Display for Coda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Coda::None => write!(f, ""),
            Coda::C1(a) => write!(f, "{}", a),
            Coda::C2(a, b) => write!(f, "{}{}", a, b),
            Coda::C3(a, b, c) => write!(f, "{}{}{}", a, b, c),
            Coda::C4(a, b, c, d) => write!(f, "{}{}{}{}", a, b, c, d),
            Coda::C5(a, b, c, d, e) => write!(f, "{}{}{}{}{}", a, b, c, d, e),
        }
    }
}

// all available english phonemes not including "None"
// generally from https://thesoundofenglish.org/ipa/ but with proper IPA notation from https://www.oed.com/
// note in particular that several
#[derive(Debug)]
pub enum Phoneme {
    Consonant(String),
    NucleusConsonant(String),
    Vowel(String)
}

impl fmt::Display for Phoneme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Phoneme::Consonant(str) => write!(f, "{}", str),
            Phoneme::NucleusConsonant(str) => write!(f, "{}", str),
            Phoneme::Vowel(str) => write!(f, "{}", str),
        }
    }
}

// Phonemes in English
// m, n, ŋ, p, b, t, d, tʃ, dʒ, k, g, f, v, θ, ð, s, z, ʃ, ʒ, x, h, l, r, j, w, // consonants (25 total)
// ɪ, iː, ʊ, uː, ɔː, e, ɛː, ə, ɜː, ɒ, æ, ʌ, ã, ɑː, eɪ, aɪ, ɔɪ, aʊ, əʊ, ɪə, ʊə // vowels (21 total)
