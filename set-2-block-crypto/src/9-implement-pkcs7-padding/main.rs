mod lib;

fn main() {
    let plain_text = String::from("YELLOW SUBMARINE");
    let padded_text = lib::pad(&plain_text.as_bytes(), 20);
    println!("{:?}", padded_text);
}