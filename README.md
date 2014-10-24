rust-stem
=========

Porter's stemmer for rust

## How to use ##

1. Add the dependency to your Cargo.toml

    ```toml
    [dependencies.stem]
    git = "https://github.com/mrordinaire/rust-stem"
    ```
2. Usage
   ```rust
   // lib.rs
   extern crate stem

   // your code
   use stem;
   let word = "pencils"
   let s = stem::get(word);
   match s {
      Ok(stemmed) => println!("{} => {}", word, stemmed),
      Err(e) => println!("could not stem! reason: {}", e),
   }
   ```
3. Compile / Run

   `$ cargo run`
