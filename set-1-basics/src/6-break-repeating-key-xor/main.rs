extern crate shared;

use std::path::Path;
use shared::file;
mod lib;

fn main() {
    let path = Path::new("./src/6-break-repeating-key-xor/6.txt");
    if !path.exists() {
        println!("File not found");
    } else {
        let cipher_text = file::read_file(path);
        match lib::break_encryption(&cipher_text) {
            Err(error) => println!("{}", error),
            Ok(cipher_text) => println!("{}", cipher_text)
        }
    }
}