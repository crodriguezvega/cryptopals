extern crate base64;
extern crate shared;

use std::path::Path;
use base64::decode;
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
        let content = file::read_file(path);
        let cipher_bytes = match decode(&content) {
            Err(_) => Vec::new(),
            Ok(cipher_bytes) => cipher_bytes
        };

        match lib::decrypt(&cipher_bytes, &key) {
            Err(error) => println!("{}", error),
            Ok(plain_text) => println!("{}", plain_text)
        }    
    }
}