rust-stem
=========

Porter's stemmer for rust

## How to use ##
1. Clone and compile the code
   ```bash
   git clone https://github.com/mrordinaire/rust-stem.git
   cd rust-stem
   make
   ```

2. Example program
   ```rust
   use stem::*;
   let word = "pencils";
   let s = stem::get(s); // stem == "pencil"
   ```

3. Compile
   ```base
   rustc example.rs -L /path/to/folder/containing/libstem*.so
   ```
