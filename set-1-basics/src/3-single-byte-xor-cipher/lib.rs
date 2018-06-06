use common::xor_cipher;

pub fn decrypt(input: &str) -> Result<String, &'static str> {
    if !input.is_ascii() {
        Err("Input string is not valid ASCII")
    } else {
        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
        xor_cipher::try_single_character_xor_decrypt(input).iter()
            .for_each(|res| {
                if res.score > maximum_score {
                    maximum_score = res.score;
                    secret_message = res.secret_message.clone();
                }
            });
        
        Ok(secret_message)
    }    
}