extern crate base64;
extern crate shared;

use std::path::Path;
use base64::decode;
use shared::file;
mod lib;

fn main() {  
    let path = Path::new("./src/10-implement-cbc-mode/10.txt");
    if !path.exists() {
        println!("File not found")
    } else {
        let content = file::read_file(path);
        let cipher_bytes = match decode(&content) {
            Err(_) => Vec::new(),
            Ok(cipher_bytes) => cipher_bytes
        };
        
        let key = String::from("YELLOW SUBMARINE");
        match lib::decrypt(&cipher_bytes, &key) {
            Err(error) => println!("{}", error),
            Ok(plain_text) => println!("{}", plain_text)
        }        
    }
}