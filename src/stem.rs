use std::ascii;
use std::ascii::Ascii;
use std::vec::Vec;

/// Member b is a vector of std::ascii::Ascii holding a word to be stemmed.
/// The letters are in b[0], b[1] ... ending at b[z->k]. Member k is readjusted
/// downwards as the stemming progresses. Zero termination is not in fact used
/// in the algorithm.
///
/// Note that only lower case sequences are stemmed. Forcing to lower case
/// should be done before get(...) is called.
///
///
/// Typical usage is:
///
///     let b = "pencils";
///     let res = stem::get(b);
///     assert_eq!(res, Ok("pencil".to_string()));
///
pub struct Stemmer {
    b: Vec<ascii::Ascii>,
    k: uint,
    j: uint,
}

impl Stemmer {
    pub fn new(word: &str) -> Result<Stemmer, &str> {
        if !word.is_ascii() {
            Err("Only support English words with ASCII characters")
        } else {
            let b = unsafe { word.to_ascii_nocheck().to_lowercase() };
            let k = b.len();
            Ok(Stemmer {
                b: b,
                k: k,
                j: 0,
            })
        }
    }

    /// stem.is_consonant(i) is true <=> stem[i] is a consonant
    pub fn is_consonant(&self, i: uint) -> bool {
        match self.b[i].to_char() {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => if i == 0 {
                true
            } else {
                !self.is_consonant(i - 1)
            },
            _ => true,
        }
    }


    /// stem.measure() measures the number of consonant sequences in [0, j).
    /// if c is a consonant sequence and v a vowel sequence, and <..> indicates
    /// arbitrary presence,
    ///
    /// ~~~notrust
    ///    <c><v>       gives 0
    ///    <c>vc<v>     gives 1
    ///    <c>vcvc<v>   gives 2
    ///    <c>vcvcvc<v> gives 3
    ///    ....
    /// ~~~
    pub fn measure(&self) -> uint {
        let mut n = 0u;
        let mut i = 0u;
        let j = self.j;
        loop {
            if i >= j { return n }
            if !self.is_consonant(i) { break }
            i += 1;
        }
        i += 1;
        loop {
            loop {
                if i >= j { return n }
                if self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
            n += 1;
            loop {
                if i >= j { return n }
                if !self.is_consonant(i) { break }
                i += 1;
            }
            i += 1;
        }
    }

    /// stem.has_vowel() is TRUE <=> [0, j-1) contains a vowel
    pub fn has_vowel(&self) -> bool {
        for i in range(0, self.j) {
            if !self.is_consonant(i) {
                return true;
            }
        }
        return false;
    }

    /// stem.double_consonant(i) is TRUE <=> i,(i-1) contain a double consonant.
    pub fn double_consonant(&self, i: uint) -> bool {
        if i < 1 {
            false
        } else if self.b[i] != self.b[i - 1] {
            false
        } else {
            self.is_consonant(i)
        }
    }

    /// cvc(z, i) is TRUE <=> i-2,i-1,i has the form consonant - vowel - consonant
    /// and also if the second c is not w,x or y. this is used when trying to
    /// restore an e at the end of a short word. e.g.
    ///
    /// ~~~notrust
    ///    cav(e), lov(e), hop(e), crim(e), but
    ///    snow, box, tray.
    /// ~~~
    pub fn cvc(&self, i: uint) -> bool {
        if i < 2 || !self.is_consonant(i) || self.is_consonant(i - 1)
            || !self.is_consonant(i - 2) { return false }
        match self.b[i].to_char() {
            'w' | 'x' | 'y' => false,
            _ => true,
        }
    }

    /// stem.ends(s) is true <=> [0, k) ends with the string s.
    pub fn ends(&mut self, s: &str) -> bool {
        let s = s.as_bytes();
        let len = s.len();
        let k = self.k;
        if s[len - 1] != self.b[k-1].to_byte() { return false } /* tiny speed-up */
        if len > k { return false }
        let mut iter = s.iter();
        for ac in self.b.slice(k - len, k).iter() {
            if ac.to_byte() != *iter.next().unwrap() { return false }
        }
        self.j = k - len;
        return true;
    }

    /// stem.setto(s) sets [j,k) to the characters in the string s,
    /// readjusting k.
    fn set_to(&mut self, s: &str) {
        let s = s.as_bytes();
        let length = s.len();
        let j = self.j;
        for i in range(0, length) {
            self.b.as_mut_slice()[j + i] = s[i].to_ascii();
        }
        self.k = j + length;
    }

    /// self.replace(s) is used further down.
    fn r(&mut self, s: &str) {
        if self.measure() > 0 {
            self.set_to(s);
        }
    }

    /// stem.step1ab() gets rid of plurals and -ed or -ing. e.g.
    ///
    /// ~~~~notrust
    ///     caresses  ->  caress
    ///     ponies    ->  poni
    ///     ties      ->  ti
    ///     caress    ->  caress
    ///     cats      ->  cat
    ///
    ///     feed      ->  feed
    ///     agreed    ->  agree
    ///     disabled  ->  disable
    ///
    ///     matting   ->  mat
    ///     mating    ->  mate
    ///     meeting   ->  meet
    ///     milling   ->  mill
    ///     messing   ->  mess
    ///
    ///     meetings  ->  meet
    /// ~~~~
    pub fn step1ab(&mut self) {
        if self.b[self.k - 1].to_char() == 's' {
            if self.ends("sses") {
                self.k -= 2;
            } else if self.ends("ies") {
                self.set_to("i");
            } else if self.b[self.k - 2].to_char() != 's' {
                self.k -= 1;
            }
        }
        if self.ends("eed") {
            if self.measure() > 0 { self.k -= 1 }
        } else if (self.ends("ed") || self.ends("ing")) && self.has_vowel() {
            self.k = self.j;
            if self.ends("at") {
                self.set_to("ate");
            } else if self.ends("bl") {
                self.set_to("ble");
            } else if self.ends("iz") {
                self.set_to("ize");
            } else if self.double_consonant(self.k - 1) {
                self.k -= 1;
                match self.b[self.k - 1].to_char() {
                    'l' | 's' | 'z' => self.k += 1,
                    _ => (),
                }
            } else if self.measure() == 1 && self.cvc(self.k - 1) {
                self.set_to("e");
            }
        }
    }

    /// stem.step1c() turns terminal y to i when there is another vowel in the stem.
    pub fn step1c(&mut self) {
       if self.ends("y") && self.has_vowel() {
           self.b.as_mut_slice()[self.k-1] = 'i'.to_ascii();
        }
    }

    /// stem.step2() maps double suffices to single ones. so -ization ( = -ize
    /// plus -ation) maps to -ize etc. note that the string before the suffix
    /// must give m(z) > 0.
    pub fn step2(&mut self) {
        match self.b[self.k-2].to_char() {
            'a' => {
                if self.ends("ational") { self.r("ate"); return }
                if self.ends("tional") { self.r("tion"); return }
            },
            'c' => {
                if self.ends("enci") { self.r("ence"); return }
                if self.ends("anci") { self.r("ance"); return }
            },
            'e' => if self.ends("izer") { self.r("ize"); return },
            'l' => {
                if self.ends("bli") { self.r("ble"); return } /*-DEPARTURE-*/

             /* To match the published algorithm, replace this line with
                'l' => {
                    if self.ends("abli") { self.r("able"); return } */

                 if self.ends("alli") { self.r("al"); return }
                 if self.ends("entli") { self.r("ent"); return }
                 if self.ends("eli") { self.r("e"); return }
                 if self.ends("ousli") { self.r("ous"); return }
            },
            'o' => {
                if self.ends("ization") { self.r("ize"); return }
                if self.ends("ation") { self.r("ate"); return }
                if self.ends("ator") { self.r("ate"); return }
            },
            's' => {
                if self.ends("alism") { self.r("al"); return }
                if self.ends("iveness") { self.r("ive"); return }
                if self.ends("fulness") { self.r("ful"); return }
                if self.ends("ousness") { self.r("ous"); return }
            },
            't' => {
                if self.ends("aliti") { self.r("al"); return }
                if self.ends("iviti") { self.r("ive"); return }
                if self.ends("biliti") { self.r("ble"); return }
            },
            'g' => if self.ends("logi") { self.r("log"); return }, /*-DEPARTURE-*/
             /* To match the published algorithm, delete this line */
            _ => (),
        }
    }

    /// stem.step3() deals with -ic-, -full, -ness etc. similar strategy to step2.
    pub fn step3(&mut self) {
        match self.b[self.k-1].to_char() {
            'e' => {
                if self.ends("icate") { self.r("ic"); return }
                if self.ends("ative") { self.r(""); return }
                if self.ends("alize") { self.r("al"); return }
            },
            'i' => if self.ends("iciti") { self.r("ic"); return },
            'l' => {
                if self.ends("ical") { self.r("ic"); return }
                if self.ends("ful") { self.r(""); return }
            },
            's' => if self.ends("ness") { self.r(""); return },
            _ => (),
        }
    }

    /// stem.step4() takes off -ant, -ence etc., in context <c>vcvc<v>.
    pub fn step4(&mut self) {
        match self.b[self.k-2].to_char() {
            'a' => {
                if self.ends("al") {}
                else { return }
            }
            'c' => {
                if self.ends("ance") {}
                else if self.ends("ence") {}
                else { return }
            },
            'e' => {
                if self.ends("er") {}
                else { return }
            }
            'i' => {
                if self.ends("ic") {}
                else { return }
            },
            'l' => {
                if self.ends("able") {}
                else if self.ends("ible") {}
                else { return }
            },
            'n' => {
                if self.ends("ant") {}
                else if self.ends("ement") {}
                else if self.ends("ment") {}
                else if self.ends("ent") {}
                else { return }
            },
            'o' => {
                if self.ends("ion")
                    && (self.b[self.j-1].to_char() == 's' || self.b[self.j-1].to_char() == 't') {}
                else if self.ends("ou") {}
                else { return }
                /* takes care of -ous */
            },
            's' => {
                if self.ends("ism") {}
                else { return }
            },
            't' => {
                if self.ends("ate") {}
                else if self.ends("iti") {}
                else { return }
            },
            'u' => {
                if self.ends("ous") {}
                else { return }
            },
            'v' => {
                if self.ends("ive") {}
                else { return }
            },
            'z' => {
                if self.ends("ize") {}
                else { return }
            },
            _ => return,
        }
        if self.measure() > 1 { self.k = self.j }
    }

    /// stem.step5() removes a final -e if self.measure() > 1, and changes -ll
    /// to -l if self.measure() > 1.
    pub fn step5(&mut self) {
       self.j = self.k;
       if self.b[self.k - 1].to_char() == 'e' {
           let a = self.measure();
           if a > 1 || a == 1 && !self.cvc(self.k - 2) { self.k -= 1 }
       }
       if self.b[self.k-1].to_char() == 'l'
           && self.double_consonant(self.k-1) && self.measure() > 1 {
           self.k-=1;
       }
    }

    pub fn get(&self) -> String {
        let borrowed = self.b.slice_to(self.k);
        borrowed.as_str_ascii().into_string()
    }
}

pub fn get(word: &str) -> Result<String, &str> {
    if word.len() > 2 {
        match Stemmer::new(word) {
            Ok(w) => {
                let mut mw = w;
                mw.step1ab();
                mw.step1c();
                mw.step2();
                mw.step3();
                mw.step4();
                mw.step5();
                Ok(mw.get())
            }
            Err(e) => Err(e),
        }
    } else {
        Ok(word.into_string())
    }
}

#[cfg(test)]
mod test {
    use std::io::File;
    use std::io::BufferedReader;
    use std::path;

    use super::get;

    #[test]
    fn lexicon() {
        let input = File::open(&path::Path::new("test-data/voc.txt")).unwrap();
        let result = File::open(&path::Path::new("test-data/output.txt")).unwrap();
        let mut input_reader = BufferedReader::new(input);
        let mut result_reader = BufferedReader::new(result);
        loop {
            match input_reader.read_line() {
                Ok(word) => match get(word.as_slice().trim()) {
                    Ok(stem) => {
                        match result_reader.read_line() {
                            Ok(answer) => if answer.as_slice().trim() != stem.as_slice() {
                                panic!("\n[FAILED] '{:s}' != '{:s}'", stem, answer);
                            } else {
                                print!(".");
                            },
                            Err(_) => break,
                        }
                    },
                    Err(e) => panic!("\n[FAILED] Cannot get stem for '{:s}': {:s}", word, e),
                },
                Err(_) => break,
            }
        }
        println!("");
    }
}
