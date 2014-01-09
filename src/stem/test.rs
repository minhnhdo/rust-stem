mod lib;

#[cfg(test)]
mod test {
    use std::io::File;
    use std::io::buffered::BufferedReader;
    use std::path;
    use lib::get;

    use super::lib;

    #[test]
    fn lexicon() {
        match File::open(&path::Path::new("test-data/voc.txt")) {
            Some(input) => match File::open(&path::Path::new("test-data/output.txt")) {
                Some(result) => {
                    let mut input_reader = BufferedReader::new(input);
                    let mut result_reader = BufferedReader::new(result);
                    loop {
                        match input_reader.read_line() {
                            Some(word) => match lib::get(word.trim()) {
                                Ok(stem) => {
                                    match result_reader.read_line() {
                                        Some(answer) => if answer.trim() != stem {
                                            println!("\n[FAILED] '{:s}' != '{:s}'", stem, answer);
                                        } else {
                                            print(".");
                                        },
                                        None => break,
                                    }
                                },
                                Err(e) => println!("\n[FAILED] Cannot get stem for '{:s}': {:s}", word, e),
                            },
                            None => break,
                        }
                    };
                },
                None => fail!("Error openning result file for testing."),
            },
            None => fail!("Error opening input file for testing."),
        }
        println("");
    }
}
