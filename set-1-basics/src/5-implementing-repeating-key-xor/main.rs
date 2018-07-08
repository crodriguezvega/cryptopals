extern crate shared;

mod lib;

fn main() {
    let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    match lib::encrypt(text, &"ICE") {
        Err(error) => println!("{}", error),
        Ok(cipher_text) => println!("{}", cipher_text)
    };
}