pub struct Stemmer<'self>(&'self str);

impl<'self> Stemmer<'self> {
    pub fn new<'a>(word: &'a str) -> Stemmer<'a> {
        Stemmer(word)
    }

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    pub fn is_consonant(&self, i: uint) -> bool {
        match self.char_at(i) {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true,
        }
    }
}

pub fn stem<'a>(word: &'a str) -> &'a str {
    word.slice_to(word.len() - 1)
}
