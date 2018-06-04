use common::mappings;

pub fn xor(a: &str, b: &str) -> Result<String, &'static str> {
    if !(a.is_ascii() && b.is_ascii()) {
        Err("Input string(s) are not valid ASCII")
    } else if a.len() != b.len() {
        Err("Input strings are not the same length")
    } else {
        let a_numbers = to_numbers(a);
        let b_numbers = to_numbers(b);

        let xor = a_numbers.iter()
            .zip(&b_numbers)
            .map(|(x, y)| x ^ y)
            .map(|x| mappings::number_to_hex(x))
            .collect::<Vec<String>>()
          .concat();
            
        Ok(xor)
    }
}

fn to_numbers(input: &str) -> Vec<u8> {
    input.chars()
        .map(|hex| mappings::hex_to_number(&hex.to_string()))
        .collect()
}