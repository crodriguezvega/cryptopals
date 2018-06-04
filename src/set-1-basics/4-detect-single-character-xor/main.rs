use std::path::Path;

mod lib;

fn main() {
    let path = Path::new("./src/set-1-basics/4-detect-single-character-xor/4.txt");
    match lib::read_file(&path) {
        Err(error) => println!("{}", error),
        Ok(_) => println!("Ok")
    };
}