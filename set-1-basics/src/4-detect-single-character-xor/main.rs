extern crate common;

use std::path::Path;
mod lib;

fn main() {
    let path = Path::new("./src/4-detect-single-character-xor/4.txt");
    match lib::decrypt(path) {
        Err(error) => println!("{}", error),
        Ok(message) => println!("{}", message)
    };
}