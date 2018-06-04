pub fn hex_to_number(input: &str) -> u8 {
    u8::from_str_radix(input, 16).unwrap()
}

pub fn number_to_hex(input: u8) -> String {
    format!("{:x?}", input)
}

pub fn number_to_binary(input: u8) -> String {
    format!("{:0width$b}", input, width = 4)
}

pub fn binary_to_number(input: &str) -> u8 {
    u8::from_str_radix(input, 2).unwrap()
}

pub fn chars_to_string(input: &[char]) -> String {
    input.iter().collect::<String>()
}

pub fn hex_string_to_numbers(input: &str) -> Vec<u8> {
    input.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chars| hex_to_number(&chars_to_string(&chars)))
        .collect()
}