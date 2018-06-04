extern crate common;

mod lib;

fn main() {
    match lib::xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965") {
        Err(error) => println!("{}", error),
        Ok(xor) => println!("{}", xor)
    };
}