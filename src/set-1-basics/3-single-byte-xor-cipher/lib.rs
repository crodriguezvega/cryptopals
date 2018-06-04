use common::mappings;
use common::xor_cipher;

pub fn decrypt(input: &str) -> Result<String, &'static str> {
    if !input.is_ascii() {
        Err("Input string is not valid ASCII")
    } else {
        let numbers = mappings::hex_string_to_numbers(input);

        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
        (0..=255).map(|ch| xor_cipher::xor(&numbers, ch))
            .map(|xor| mappings::chars_to_string(&xor))
            .for_each(|msg| {
                let value = xor_cipher::score(&msg);
                if value > maximum_score {
                    maximum_score = value;
                    secret_message = msg;
                }
            });
        
        Ok(secret_message)
    }    
}