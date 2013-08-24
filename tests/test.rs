extern mod stem;

use std::io;
use std::path;

use stem::*;

fn main() {
    match io::file_reader(&path::PosixPath("tests/voc.txt")) {
        Ok(input) => match io::file_reader(&path::PosixPath("tests/output.txt")) {
            Ok(result) => {
                do input.each_line |word| {
                    match stem::get(word) {
                        Ok(stem) => {
                            let answer = result.read_line();
                            if stem != answer {
                                printfln!("\n[FAILED] '%s' != '%s'", stem, answer);
                            } else {
                                print(".");
                            }
                        },
                        Err(e) => printfln!("\n[FAILED] Cannot get stem for '%s': %s", word, e),
                    }
                    true
                };
            },
            Err(e) => println(e),
        },
        Err(e) => println(e),
    }
}
