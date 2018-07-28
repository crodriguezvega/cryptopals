extern crate itertools;
extern crate base64;
extern crate rand;
extern crate shared;

use rand::{ thread_rng, Rng };
use base64::decode;
mod lib;

// https://crypto.stackexchange.com/questions/42891/chosen-plaintext-attack-on-aes-in-ecb-mode

fn main() {    
    let mut rng = thread_rng();
    let key: [u8; 16] = rng.gen();

    let base64_text = String::from("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28g\
bXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");

    let unknown_bytes = match decode(&base64_text) {
        Err(_) => Vec::new(),
        Ok(unknown_bytes) => unknown_bytes
    };

    match lib::decrypt(&unknown_bytes, &key) {
        Err(error) => println!("{}", error),
        Ok(plain_text) => println!("{}", plain_text)
    }
}

