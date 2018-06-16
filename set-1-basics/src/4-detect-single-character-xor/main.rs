extern crate common;

use std::path::Path;
mod lib;

fn main() {
    let path = Path::new("./src/4-detect-single-character-xor/4.txt");
    match lib::break_encryption(path) {
        Err(error) => println!("{}", error),
        Ok(secret_message) => println!("{}", secret_message)
    };
}