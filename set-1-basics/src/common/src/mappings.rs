pub enum Padding {
    No,
    Two
}

pub fn chars_to_string(input: &[char]) -> String {
    input.iter().collect::<String>()
}

pub fn hex_to_byte(input: &str) -> u8 {
    u8::from_str_radix(input, 16).unwrap()
}

pub fn byte_to_hex(input: u8, padding: Padding) -> String {
    match padding {
        Padding::No => format!("{:x}", input),
        Padding::Two => format!("{:02x}", input) 
    }    
}

pub fn byte_to_binary(input: u8) -> String {
    format!("{:0width$b}", input, width = 4)
}

pub fn binary_to_byte(input: &str) -> u8 {
    u8::from_str_radix(input, 2).unwrap()
}

pub fn hex_string_to_bytes(input: &str) -> Vec<u8> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chars| hex_to_byte(&chars_to_string(&chars)))
        .collect()
}

pub fn bytes_to_hex_string(input: &[u8]) -> String {
    input.iter()
        .map(|byte| byte_to_hex(*byte, Padding::Two))
        .collect::<Vec<String>>()
        .concat()
}