use std::bool;
use std::ascii;

pub struct Stemmer {
    priv word: ~[ascii::Ascii],
    priv j: uint,
}

impl Stemmer {
    pub fn new(word: &str) -> Result<Stemmer, ~str> {
        if bool::not(word.is_ascii()) {
            Err(~"Only support English words with ASCII characters")
        } else {
            Ok(Stemmer {
                word: unsafe { word.to_ascii_nocheck().to_lower() },
                j: 0,
            })
        }
    }

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    pub fn is_consonant(&self, i: uint) -> bool {
        match self.word[i].to_char() {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true,
        }
    }

    pub fn get(&self) -> ~str {
        self.word.to_str_ascii()
    }
}

pub fn get(word: &str) -> Result<~str, ~str> {
    if word.len() > 2 {
        match Stemmer::new(word) {
            Ok(w) => Ok(w.get()),
            Err(e) => Err(e),
        }
    } else {
        Ok(word.to_owned())
    }
}
