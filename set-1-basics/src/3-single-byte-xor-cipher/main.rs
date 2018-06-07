extern crate common;

mod lib;

fn main() {
    match lib::decrypt("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736") {
        Err(error) => println!("{}", error),
        Ok(secret_message) => println!("{}", secret_message)
    };
}