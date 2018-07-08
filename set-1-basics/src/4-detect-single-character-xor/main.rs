extern crate shared;

use std::path::Path;
use shared::file;
mod lib;

fn main() {
    let path = Path::new("./src/4-detect-single-character-xor/4.txt");
    if !path.exists() {
        println!("File not found");
    } else {
        let lines = file::read_lines(path);
        let secret_message = lib::break_encryption(&lines);
        println!("{}", secret_message);
    }
}