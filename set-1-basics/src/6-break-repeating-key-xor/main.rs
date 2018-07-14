extern crate shared;
extern crate base64;

use base64::decode;
use std::path::Path;
use shared::file;
mod lib;

fn main() {
    let path = Path::new("./src/6-break-repeating-key-xor/6.txt");
    if !path.exists() {
        println!("File not found");
    } else {
        let content = file::read_file(path);
        let cipher_bytes = match decode(&content) {
            Err(_) => Vec::new(),
            Ok(cipher_bytes) => cipher_bytes
        };

        match lib::break_encryption(&cipher_bytes) {
            Err(error) => println!("{}", error),
            Ok(plain_text) => println!("{}", plain_text)
        }
    }
}