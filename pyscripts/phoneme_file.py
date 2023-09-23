# Python script for building a phoneme file in TOML
# Note: current Rust functionality doesn't allow for parsing anything other than arrays at the current time...

import os
import codecs

def writePhonemeFile(filename, text):
    f = codecs.open(filename, mode="w", encoding="utf-8")
    f.write(text)
    f.close()

dirname = os.path.dirname(__file__)
filename = os.path.join(dirname, "../data/phonemes.toml")

# better to get some form of TOML deserialiser for this sort of thing...
# or at the very least keep a list that can be added to that match a deserialised class for phonemes
# ideally something where adding a phoneme to the list is somewhat painless, maybe learn basic UI to add some form of program?
# definitely should be defining phonemes as their own objects within toml and placing them into arrays, each phoneme having it's own category values etc.
text = """# TOML file for phoneme category identification
# Theoretically, eventually dynamically generated, but for now statically generated through phoneme_file.py

[phonemes.categories]
consonants = ["nasal", "plosive", "affricate", "fricative", "approximant"]
vowels = ["monophthong", "potential_diphthong", "diphthong", "rhotacised", "reduced"]

[phonemes.consonants]
nasal = ["m", "n", "ŋ"]
plosives = ["p", "b", "t", "d", "g"]
affricate = ["tʃ", "dʒ"]
fricative = ["f", "v", "θ", "ð", "s", "z", "ʃ", "ʒ", "x", "h"]
approximant = ["l", "r", "j", "w"]

[phonemes.vowels]
monophthong = ["æ", "ɑː", "ɒ", "ɔː", "ɪ", "e", "ʌ", "ʊ", "ã"]
potential_diphthong = ["eɪ", "əʊ", "iː", "uː"]
diphthong = ["aɪ", "ɔɪ", "aʊ"]
rhotacised = ["ɜː", "ɑː", "ɔː", "ɪə", "ɛː", "ʊə"]
reduced = ["ə", "i"]"""

if os.path.exists(filename):
    var = input("File " + filename + " already exists, do you wish to overwrite? y/n")
    if var == "y" or var == "Y":
        writePhonemeFile(filename, text)
else:
    writePhonemeFile(filename, text)