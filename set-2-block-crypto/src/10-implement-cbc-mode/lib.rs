use shared::aes;

pub fn decrypt(cipher_text: &[u8], key: &str) -> Result<String, &'static str> {
    let iv = vec![0; 16]; 
    match aes::cbc_decrypt(&cipher_text, &key.as_bytes(), &iv) {
        Err(error) => Err(error),
        Ok(plain_bytes) => {
            match String::from_utf8(plain_bytes) {
                Err(_) => Err("Plain text is not valid UTF8"),
                Ok(plain_text) => Ok(plain_text)
            }
        }
    }
}