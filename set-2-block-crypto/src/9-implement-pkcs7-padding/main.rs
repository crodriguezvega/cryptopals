extern crate shared;

use shared::pkcs7;

fn main() {
    let plain_text = String::from("YELLOW SUBMARINE");
    let padded_text = pkcs7::pad(&plain_text.as_bytes(), 20);
    println!("{:?}", padded_text);
}