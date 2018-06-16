use std::path::Path;
use common::{ file, mappings, xor_cipher };
 
pub fn break_encryption(path: &Path) -> Result<String, &'static str> {
    if !path.exists() {
        Err("File not found")
    } else {
        let mut maximum_score = 0.0;
        let mut secret_message = String::new();
  
        file::read_lines(path).iter()
            .map(|line| mappings::hex_string_to_bytes(line))
            .flat_map(|bytes| xor_cipher::try_decrypt_single_character_xor(&bytes))
            .for_each(|res| {
                if res.score > maximum_score {
                    maximum_score = res.score;
                    secret_message = res.secret_message.clone();
                }
            });

        Ok(secret_message)
    }
}