extern crate itertools;
extern crate shared;

use std::path::Path;
use shared::file;
mod lib;

fn main() {
    let path = Path::new("./src/8-detect-aes-in-ecb-mode/8.txt");
    if !path.exists() {
        println!("File not found")
    } else {
        let lines = file::read_lines(path);
        let cipher_text = lib::detect(&lines);
        println!("{}", cipher_text);
    }
}