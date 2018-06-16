extern crate base64;
extern crate common;

use std::path::Path;
mod lib;

fn main() {
    let path = Path::new("./src/6-break-repeating-key-xor/6.txt");
    match lib::decrypt(path) {
        Err(error) => println!("{}", error),
        Ok(cipher_text) => println!("{}", cipher_text)
    };
}