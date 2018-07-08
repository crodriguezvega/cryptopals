extern crate shared;

mod lib;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match lib::hex_to_base64(input) {
        Err(error) => println!("{}", error),
        Ok(base64) => println!("{}", base64)
    };
}