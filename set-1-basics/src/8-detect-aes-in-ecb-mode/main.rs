extern crate itertools;
extern crate common;

use std::path::Path;
mod lib;

fn main() {
    let path = Path::new("./src/8-detect-aes-in-ecb-mode/8.txt");
    match lib::detect(path) {
        Err(error) => println!("{}", error),
        Ok(cipher_text) => println!("{}", cipher_text)
    };
}