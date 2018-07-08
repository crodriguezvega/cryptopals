extern crate crypto;
extern crate shared;

use std::path::Path;
use shared::file;
mod lib;

fn main() {
    let key = "YELLOW SUBMARINE".to_string();
    let path = Path::new("./src/7-aes-in-ecb-mode/7.txt");
    if !path.exists() {
        println!("File not found");
    } else if !key.is_ascii() {
        println!("Key is not valid ASCII");
    } else {
        let cipher_text = file::read_file(path);
        match lib::decrypt(&cipher_text, &key) {
            Err(error) => println!("{}", error),
            Ok(plain_text) => println!("{}", plain_text)
        }
    };
}