use itertools::Itertools;
use rand::{ thread_rng, Rng };
use shared::{ aes, pkcs7 };

pub struct EncryptionOracle<'a> {
    pub key: &'a[u8],
    pub unknown_bytes: &'a[u8]
}

impl<'a> EncryptionOracle<'a> {
    pub fn encrypt(&self, known_bytes: &[u8]) -> Result<Vec<u8>, &'static str> {
        let input = [&known_bytes[..], &self.unknown_bytes[..]].concat();  
        let padded_input = pkcs7::pad(&input, 16);
        aes::ecb_encrypt(&padded_input, &self.key)
    }
}

pub fn byte_at_a_time_decrypt(encryption_oracle: &EncryptionOracle) -> Result<String, &'static str> {
    let block_size = match find_block_size(encryption_oracle) {
        Err(error) => return Err(error),
        Ok(block_size) => block_size
    };

    let _is_aes_ecb = match is_aes_ecb(encryption_oracle, block_size) {
        Err(error) => return Err(error),
        Ok(is_aes_ecb) => is_aes_ecb
    };

    if _is_aes_ecb {
        let mut decrypted_bytes = Vec::new();
        loop {            
            let result = match decrypt_byte(encryption_oracle, &decrypted_bytes, block_size) {
                Err(error) => return Err(error),
                Ok(result) => result
            };

            match result {
                None => break,
                Some(byte) => decrypted_bytes.push(byte)
            };
        }
        
        match String::from_utf8(decrypted_bytes) {
            Err(_) => Err("Plain text is not valid UTF8"),
            Ok(plain_text) => Ok(plain_text)
        }
    } else {
        Err("Not using ECB mode")
    }
}

fn find_block_size(encryption_oracle: &EncryptionOracle) -> Result<usize, &'static str> {
    let mut known_bytes = Vec::new();
    let len_1 = match encryption_oracle.encrypt(&known_bytes) {
        Err(error) => return Err(error),
        Ok(cipher_text) => cipher_text.len()
    };

    loop {
        known_bytes.push(0);

        let len_2 = match encryption_oracle.encrypt(&known_bytes) {
            Err(error) => return Err(error),
            Ok(cipher_text) => cipher_text.len()
        };

        if len_1 != len_2 {
            return Ok(len_2 - len_1)
        }
    }
}

fn is_aes_ecb(encryption_oracle: &EncryptionOracle, block_size: usize) -> Result<bool, &'static str> {
    let mut random_bytes = vec!(0; block_size);
    thread_rng().fill(&mut random_bytes[..]);

    let known_bytes = [&random_bytes[..], &random_bytes[..]].concat();

    let cipher_text = match encryption_oracle.encrypt(&known_bytes) {
        Err(error) => return Err(error),
        Ok(cipher_text) => cipher_text
    };

    let result = cipher_text.chunks(block_size)
        .combinations(2)
        .fold(0, |acc, combination| {
            if combination[0] == combination[1] {
                acc + 1
            } else {
                acc
            }
        });

    if result > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn decrypt_byte(encryption_oracle: &EncryptionOracle, decrypted_bytes: &[u8], block_size: usize) -> Result<Option<u8>, &'static str> {
    let length = block_size - decrypted_bytes.len() % block_size - 1; 
    let chosen_bytes = vec!(0; length);

    let cipher_text_1 = match encryption_oracle.encrypt(&chosen_bytes) {
        Err(error) => return Err(error),
        Ok(cipher_text) => cipher_text
    };

    for byte in 0..=255 {
        let mut known_bytes = [&chosen_bytes[..], &decrypted_bytes[..]].concat();
        known_bytes.push(byte);

        let cipher_text_2 = match encryption_oracle.encrypt(&known_bytes) {
            Err(error) => return Err(error),
            Ok(cipher_text) => cipher_text
        };

        if cipher_text_1[0..known_bytes.len()] == cipher_text_2[0..known_bytes.len()] {
            return Ok(Some(byte))
        }      
    }

    Ok(None)
}