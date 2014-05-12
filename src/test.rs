extern crate stem;

#[cfg(test)]
mod test {
    use std::io::File;
    use std::io::BufferedReader;
    use std::path;
    use stem::get;

    use super::stem;

    #[test]
    fn lexicon() {
        let input = File::open(&path::Path::new("test-data/voc.txt")).unwrap();
        let result = File::open(&path::Path::new("test-data/output.txt")).unwrap();
        let mut input_reader = BufferedReader::new(input);
        let mut result_reader = BufferedReader::new(result);
        loop {
            match input_reader.read_line() {
                Ok(word) => match stem::get(word.trim()) {
                    Ok(stem) => {
                        match result_reader.read_line() {
                            Ok(answer) => if answer.trim() != stem {
                                fail!("\n[FAILED] '{:s}' != '{:s}'", stem, answer);
                            } else {
                                print!(".");
                            },
                            Err(_) => break,
                        }
                    },
                    Err(e) => fail!("\n[FAILED] Cannot get stem for '{:s}': {:s}", word, e),
                },
                Err(_) => break,
            }
        }
        println!("");
    }
}
