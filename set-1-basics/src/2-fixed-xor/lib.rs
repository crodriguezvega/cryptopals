use shared::mappings;

pub fn xor(a: &str, b: &str) -> Result<String, &'static str> {
    if !(a.is_ascii() && b.is_ascii()) {
        Err("Input string(s) are not valid ASCII")
    } else if a.len() != b.len() {
        Err("Input strings are not the same length")
    } else {
        let a_numbers = to_bytes(a);
        let b_numbers = to_bytes(b);

        let xor = a_numbers.iter()
            .zip(&b_numbers)
            .map(|(x, y)| x ^ y)
            .map(|byte| mappings::byte_to_hex(byte, mappings::Padding::No))
            .collect::<Vec<String>>()
            .concat();
            
        Ok(xor)
    }
}

fn to_bytes(input: &str) -> Vec<u8> {
    input.chars()
        .map(|hex| mappings::hex_to_byte(&hex.to_string()))
        .collect()
}