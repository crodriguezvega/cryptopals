use shared::{ mappings, xor_cipher };
 
pub fn break_encryption(lines: &[String]) -> String {
    let mut maximum_score = 0.0;
    let mut secret_message = String::new();

    lines.iter()
        .map(|line| mappings::hex_string_to_bytes(line))
        .flat_map(|bytes| xor_cipher::try_decrypt_single_character_xor(&bytes))
        .for_each(|res| {
            if res.score > maximum_score {
                maximum_score = res.score;
                secret_message = res.secret_message.clone();
            }
        });

    secret_message
}