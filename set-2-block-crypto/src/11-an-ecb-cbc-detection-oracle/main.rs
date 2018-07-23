extern crate itertools;
extern crate rand;
extern crate shared;

mod lib;

fn main() {     
    let plain_text = String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    match lib::encryption_oracle(&plain_text.as_bytes()) {
        Err(error) => println!("{}", error),
        Ok(result) => {
            if result.0 == lib::detect_aes_mode(&result.1) {
                println!("Correctly detected {:?} mode", result.0)
            } else {
                println!("Did not detect {:?} mode", result.0)
            }            
        }
    }
}