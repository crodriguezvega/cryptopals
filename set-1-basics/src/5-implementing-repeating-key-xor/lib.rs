use shared::mappings;

pub fn encrypt(input: &str, key: &str) -> Result<String, &'static str> {
    if !input.is_ascii() {
        Err("Input string is not valid ASCII")
    } else if !key.is_ascii() {
        Err("Key is not valid ASCII")
    } else {
        let key_length = key.len();
        let key_chars: Vec<char> = key.chars().collect(); 
        let mut cipher_bytes = Vec::new();
        input.chars()
            .enumerate()     
            .for_each(|(i, ch)| {
                let plain_byte = ch as u8;
                let key_byte = key_chars[i % key_length] as u8;
                let cipher_byte = plain_byte ^ key_byte;
                cipher_bytes.push(cipher_byte)               
            });

        let cipher_text = mappings::bytes_to_hex_string(&cipher_bytes);
        Ok(cipher_text)
    }
}