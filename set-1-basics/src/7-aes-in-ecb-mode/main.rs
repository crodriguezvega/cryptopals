extern crate crypto;
extern crate common;

use std::path::Path;
mod lib;

fn main() {
    let key = "YELLOW SUBMARINE".to_string();
    let path = Path::new("./src/7-aes-in-ecb-mode/7.txt");
    match lib::decrypt(path, &key) {
        Err(error) => println!("{}", error),
        Ok(plain_text) => println!("{}", plain_text)
    };
}